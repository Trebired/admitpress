use super::*;

pub(in crate::service) fn ensure_printer_ready(queue: &str) -> Result<()> {
    let accepting = Command::new("lpstat").arg("-a").arg(queue).output();
    match accepting {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if text.contains("not accepting") || text.contains("rejecting jobs") {
                return Err(TicketError::PrinterQueueNotAccepting(queue.to_owned()).into());
            }
        }
        Ok(output) => {
            return Err(TicketError::PrintSubmission(command_failure("lpstat -a", &output)).into());
        }
        Err(error) => return Err(anyhow!("failed to query CUPS queue acceptance: {error}")),
    }

    let enabled = Command::new("lpstat").arg("-p").arg(queue).output();
    match enabled {
        Ok(output) if output.status.success() => {
            let text = String::from_utf8_lossy(&output.stdout).to_lowercase();
            if text.contains(" disabled ") || text.contains("rejecting jobs") {
                return Err(TicketError::PrinterQueueNotAccepting(queue.to_owned()).into());
            }
        }
        Ok(output) => {
            return Err(TicketError::PrintSubmission(command_failure("lpstat -p", &output)).into());
        }
        Err(error) => return Err(anyhow!("failed to query CUPS printer state: {error}")),
    }

    Ok(())
}

pub(in crate::service) fn run_printer_command(
    program: &str,
    args: &[&str],
) -> PrinterCommandReport {
    run_printer_command_with_timeout(program, args, PRINTER_COMMAND_TIMEOUT)
}

pub(in crate::service) fn run_printer_command_cancelable<F>(
    program: &str,
    args: &[&str],
    is_cancelled: &F,
) -> PrinterCommandReport
where
F: Fn() -> bool,
{
    run_printer_command_with_timeout_cancelable(
        program,
        args,
        PRINTER_COMMAND_TIMEOUT,
        is_cancelled,
    )
}

pub(in crate::service) fn run_admin_printer_fix_command<F>(
    start_cups: bool,
    selected_queue: Option<&str>,
    is_cancelled: &F,
) -> PrinterCommandReport
where
F: Fn() -> bool,
{
    let mut script = String::from("set -u\n");
    if start_cups {
        script.push_str(&format!(
                "{} enable --now cups\n",
                shell_quote(&command_path("systemctl"))
        ));
    }
    if let Some(queue) = selected_queue.and_then(normalized_optional) {
        script.push_str(&format!(
                "{} {}\n{} {}\n",
                shell_quote(&command_path("cupsaccept")),
                shell_quote(&queue),
                shell_quote(&command_path("cupsenable")),
                shell_quote(&queue),
        ));
    } else {
        script.push_str(
            "printf '%s\\n' 'no printer queue is selected; skipped queue enable/accept'\n",
        );
    }
    run_admin_printer_script(&script, is_cancelled)
}

fn missing_polkit_report() -> PrinterCommandReport {
    PrinterCommandReport {
        command: "pkexec".to_owned(),
        exit_code: None,
        success: false,
        timed_out: false,
        stdout: String::new(),
        stderr: missing_polkit_message(),
    }
}

fn missing_polkit_message() -> String {
    let mut message = String::from("pkexec is not installed, so the printer cannot be repaired ");
    message.push_str("with administrator rights. Install polkit and an authentication agent ");
    message.push_str("for your desktop, then try again.");
    message
}

pub(in crate::service) fn run_admin_printer_script<F>(
    script: &str,
    is_cancelled: &F,
) -> PrinterCommandReport
where
F: Fn() -> bool,
{
    if !command_available("pkexec") {
        return missing_polkit_report();
    }
    let admin_tool = "pkexec";
    let shell = command_path("sh");
    let admin_args = [shell.as_str(), "-c", script];
    run_printer_command_with_timeout_cancelable(
        admin_tool,
        &admin_args,
        ADMIN_PRINTER_COMMAND_TIMEOUT,
        is_cancelled,
    )
}

pub(in crate::service) fn run_printer_command_with_timeout(
    program: &str,
    args: &[&str],
    timeout: Duration,
) -> PrinterCommandReport {
    run_printer_command_with_timeout_cancelable(program, args, timeout, &|| false)
}

pub(in crate::service) fn run_printer_command_with_timeout_cancelable<F>(
    program: &str,
    args: &[&str],
    timeout: Duration,
    is_cancelled: &F,
) -> PrinterCommandReport
where
F: Fn() -> bool,
{
    let command = display_command(program, args);
    let mut child = match Command::new(program)
    .args(args)
    .stdout(Stdio::piped())
    .stderr(Stdio::piped())
    .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return PrinterCommandReport {
                command,
                exit_code: None,
                success: false,
                timed_out: false,
                stdout: String::new(),
                stderr: error.to_string(),
            };
        }
    };

    let started_at = Instant::now();
    loop {
        if is_cancelled() {
            let _ = child.kill();
            return match child.wait_with_output() {
                Ok(output) => {
                    let mut report = command_report_from_output(command, false, output);
                    report.success = false;
                    report.stderr = append_line(report.stderr, "cancelled");
                    report
                }
                Err(error) => PrinterCommandReport {
                    command,
                    exit_code: None,
                    success: false,
                    timed_out: false,
                    stdout: String::new(),
                    stderr: format!("cancelled; failed to collect command output: {error}"),
                },
            };
        }
        match child.try_wait() {
            Ok(Some(_)) => {
                return match child.wait_with_output() {
                    Ok(output) => command_report_from_output(command, false, output),
                    Err(error) => PrinterCommandReport {
                        command,
                        exit_code: None,
                        success: false,
                        timed_out: false,
                        stdout: String::new(),
                        stderr: error.to_string(),
                    },
                };
            }
            Ok(None) if started_at.elapsed() >= timeout => {
                let _ = child.kill();
                return match child.wait_with_output() {
                    Ok(output) => command_report_from_output(command, true, output),
                    Err(error) => PrinterCommandReport {
                        command,
                        exit_code: None,
                        success: false,
                        timed_out: true,
                        stdout: String::new(),
                        stderr: error.to_string(),
                    },
                };
            }
            Ok(None) => thread::sleep(Duration::from_millis(25)),
            Err(error) => {
                return PrinterCommandReport {
                    command,
                    exit_code: None,
                    success: false,
                    timed_out: false,
                    stdout: String::new(),
                    stderr: error.to_string(),
                };
            }
        }
    }
}

pub(in crate::service) fn append_line(mut value: String, line: &str) -> String {
    if !value.is_empty() {
        value.push('\n');
    }
    value.push_str(line);
    value
}

pub(in crate::service) fn app_command_report(message: impl Into<String>) -> PrinterCommandReport {
    PrinterCommandReport {
        command: "Application repair".to_owned(),
        exit_code: Some(0),
        success: true,
        timed_out: false,
        stdout: message.into(),
        stderr: String::new(),
    }
}

pub(in crate::service) fn app_failed_command_report(
    message: impl Into<String>,
) -> PrinterCommandReport {
    PrinterCommandReport {
        command: "Application repair".to_owned(),
        exit_code: None,
        success: false,
        timed_out: false,
        stdout: String::new(),
        stderr: message.into(),
    }
}

pub(in crate::service) fn choose_repair_queue(
    selected_queue: Option<&str>,
    default_queue: Option<&str>,
    detected_queues: &[String],
) -> Option<String> {
    if let Some(queue) = selected_queue.and_then(normalized_optional)
    && (detected_queues.is_empty() || detected_queues.iter().any(|detected| detected == &queue))
    {
        return Some(queue);
    }

    if let Some(queue) = default_queue.and_then(normalized_optional)
    && detected_queues.iter().any(|detected| detected == &queue)
    {
        return Some(queue);
    }

    if detected_queues.len() == 1 {
        return detected_queues.first().cloned();
    }

    None
}

pub(in crate::service) fn should_start_cups(diagnostics: &PrinterDiagnostics) -> bool {
    diagnostics
    .problems
    .iter()
    .any(|problem| problem.contains("CUPS scheduler is not running"))
    || diagnostics.commands.iter().any(|report| {
            let text = format!("{}\n{}", report.stdout, report.stderr).to_lowercase();
            text.contains("scheduler is not running")
    })
}

pub(in crate::service) fn command_report_from_output(
    command: String,
    timed_out: bool,
    output: std::process::Output,
) -> PrinterCommandReport {
    PrinterCommandReport {
        command,
        exit_code: output.status.code(),
        success: output.status.success() && !timed_out,
        timed_out,
        stdout: String::from_utf8_lossy(&output.stdout).trim().to_owned(),
        stderr: String::from_utf8_lossy(&output.stderr).trim().to_owned(),
    }
}

pub(in crate::service) fn display_command(program: &str, args: &[&str]) -> String {
    std::iter::once(program)
    .chain(args.iter().copied())
    .map(shell_quote)
    .collect::<Vec<_>>()
    .join(" ")
}

pub(in crate::service) fn shell_quote(value: &str) -> String {
    if value
    .chars()
    .all(|character| character.is_ascii_alphanumeric() || "_./:=-".contains(character))
    {
        value.to_owned()
    } else {
        format!("'{}'", value.replace('\'', "'\\''"))
    }
}

pub(in crate::service) fn command_available(program: &str) -> bool {
    command_path(program) != program || Path::new(program).is_file()
}

pub(in crate::service) fn command_path(program: &str) -> String {
    if program.contains('/') {
        return program.to_owned();
    }

    let mut paths = std::env::var_os("PATH")
    .map(|value| std::env::split_paths(&value).collect::<Vec<_>>())
    .unwrap_or_default();
    for standard_path in ["/usr/sbin", "/usr/bin", "/sbin", "/bin"] {
        let path = PathBuf::from(standard_path);
        if !paths.iter().any(|existing| existing == &path) {
            paths.push(path);
        }
    }

    paths
    .into_iter()
    .map(|path| path.join(program))
    .find(|path| path.is_file())
    .map(|path| path.to_string_lossy().into_owned())
    .unwrap_or_else(|| program.to_owned())
}
