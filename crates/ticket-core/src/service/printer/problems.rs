use super::*;

pub(in crate::service) fn printer_problems(
    development_mode: bool,
    selected_queue: Option<&str>,
    detected_queues: &[String],
    commands: &[PrinterCommandReport],
) -> (Vec<String>, Vec<String>) {
    let mut problems = Vec::new();
    let mut suggested_commands = Vec::new();

    if development_mode {
        push_unique(
            &mut problems,
            "development output is enabled; physical print buttons render files until it is turned off",
        );
    }

    match selected_queue {
        Some(queue) if !queue.trim().is_empty() => {
            if !detected_queues.iter().any(|detected| detected == queue) {
                push_unique(
                    &mut problems,
                    format!("selected printer queue \"{queue}\" is not listed by CUPS"),
                );
            }
        }
        _ => {
            push_unique(&mut problems, "no printer queue is selected in settings");
        }
    }

    for report in commands {
        let text = format!("{}\n{}", report.stdout, report.stderr).to_lowercase();
        if report.timed_out {
            push_unique(
                &mut problems,
                format!("{} timed out after 3 seconds", report.command),
            );
        }
        if text.contains("scheduler is not running") {
            push_unique(&mut problems, "CUPS scheduler is not running");
            push_unique(&mut suggested_commands, "sudo systemctl enable --now cups");
        }
        if text.contains("not accepting") || text.contains("rejecting jobs") {
            if let Some(queue) = selected_queue {
                push_unique(
                    &mut problems,
                    format!("printer queue \"{queue}\" is rejecting jobs"),
                );
                push_unique(&mut suggested_commands, format!("sudo cupsaccept {queue}"));
            } else {
                push_unique(&mut problems, "a CUPS queue is rejecting jobs");
            }
        }
        if text.contains(" disabled ") || text.contains("printer disabled") {
            if let Some(queue) = selected_queue {
                push_unique(
                    &mut problems,
                    format!("printer queue \"{queue}\" is disabled"),
                );
                push_unique(&mut suggested_commands, format!("sudo cupsenable {queue}"));
            } else {
                push_unique(&mut problems, "a CUPS queue is disabled");
            }
        }
        if text.contains("invalid destination") || text.contains("unknown destination") {
            if let Some(queue) = selected_queue {
                push_unique(
                    &mut problems,
                    format!("printer queue \"{queue}\" does not exist in CUPS"),
                );
            } else {
                push_unique(
                    &mut problems,
                    "CUPS reported an invalid printer destination",
                );
            }
        }
        if text.contains("no system default destination") && selected_queue.is_none() {
            push_unique(
                &mut problems,
                "CUPS has no default printer and the app has none selected",
            );
        }
        if text.contains("permission denied") {
            push_unique(
                &mut problems,
                format!(
                    "{} could not read printer logs due to permissions",
                    report.command
                ),
            );
        }
    }

    (problems, suggested_commands)
}

pub(in crate::service) fn push_unique(values: &mut Vec<String>, value: impl Into<String>) {
    let value = value.into();
    if !values.iter().any(|existing| existing == &value) {
        values.push(value);
    }
}
