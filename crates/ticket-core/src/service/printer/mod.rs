use super::*;

mod commands;
mod problems;
mod queues;

pub(super) use commands::*;
pub(super) use problems::*;
pub(super) use queues::*;

impl TicketService {
    pub fn printer_diagnostics(&mut self) -> Result<PrinterDiagnostics> {
        let selected_queue = self.selected_printer_queue.clone();
        let detected_queues = discover_printer_queues();
        self.cached_printer_queues = detected_queues.clone();
        let default_queue = default_printer_queue();
        let commands = diagnostic_commands(selected_queue.as_deref());
        let recent_failures = recent_printer_failures(self)?;
        let (problems, suggested_commands) = printer_problems(
            self.development_mode,
            selected_queue.as_deref(),
            &detected_queues,
            &commands,
        );

        Ok(PrinterDiagnostics {
                selected_queue,
                development_mode: self.development_mode,
                detected_queues,
                default_queue,
                problems,
                suggested_commands,
                recent_failures,
                commands,
        })
    }

    pub fn printer_activity(&mut self) -> Result<PrinterActivity> {
        let Some(queue) = self.selected_printer_queue.clone() else {
            return Ok(PrinterActivity {
                    queue: None,
                    current: None,
                    queued: Vec::new(),
            });
        };
        let mut jobs = Vec::new();
        for ticket in self.storage.list_tickets()? {
            for record in self.storage.list_recent_prints(&ticket.id, 20)? {
                if record.printer_name.as_deref() != Some(queue.as_str()) {
                    continue;
                }
                jobs.push(PrinterActivityJob {
                        ticket_id: ticket.id.clone(),
                        ticket_name: ticket.name.clone(),
                        status: record.status,
                        allocated_number: record.allocated_number,
                        is_bulk: record.is_bulk,
                        operation_label: record.operation_label,
                        recorded_at: record.recorded_at,
                });
            }
        }
        jobs.sort_by(|left, right| right.recorded_at.cmp(&left.recorded_at));
        let mut jobs = jobs.into_iter();
        let current = jobs.next();
        let queued = jobs
        .filter(|job| job.status == "pending" || job.status == "queued")
        .take(5)
        .collect();
        Ok(PrinterActivity {
                queue: Some(queue),
                current,
                queued,
        })
    }

    pub fn fix_printer(&mut self, use_admin: bool) -> Result<PrinterFixResult> {
        self.fix_printer_with_cancel(use_admin, || false)
    }

    pub fn fix_printer_with_cancel<F>(
        &mut self,
        use_admin: bool,
        is_cancelled: F,
    ) -> Result<PrinterFixResult>
    where
    F: Fn() -> bool,
    {
        let diagnostics_before = self.printer_diagnostics()?;
        let mut commands = Vec::new();
        let mut changed_selected_queue = false;
        let mut detected_queues = diagnostics_before.detected_queues.clone();
        let mut selected_queue = self
        .selected_printer_queue
        .clone()
        .and_then(normalized_optional);

        macro_rules! finish_if_cancelled {
            () => {
                if is_cancelled() {
                    commands.push(app_failed_command_report("printer fix cancelled"));
                    return Ok(PrinterFixResult {
                            used_admin: use_admin,
                            selected_queue: self.selected_printer_queue.clone(),
                            changed_selected_queue,
                            commands,
                            diagnostics_before: diagnostics_before.clone(),
                            diagnostics_after: diagnostics_before,
                    });
                }
            };
        }

        finish_if_cancelled!();
        if let Some(queue) = choose_repair_queue(
            selected_queue.as_deref(),
            diagnostics_before.default_queue.as_deref(),
            &detected_queues,
        ) {
            if selected_queue.as_deref() != Some(queue.as_str()) {
                self.set_selected_printer_queue(Some(&queue))?;
                changed_selected_queue = true;
                commands.push(app_command_report(format!(
                            "selected printer queue changed to {queue}"
                )));
            }
            selected_queue = Some(queue);
        }

        finish_if_cancelled!();
        if let Some(queue) = selected_queue.as_deref() {
            commands.push(run_printer_command_cancelable(
                    "lpoptions",
                    &["-d", queue],
                    &is_cancelled,
            ));
            finish_if_cancelled!();
            commands.push(run_printer_command_cancelable(
                    "cupsaccept",
                    &[queue],
                    &is_cancelled,
            ));
            finish_if_cancelled!();
            commands.push(run_printer_command_cancelable(
                    "cupsenable",
                    &[queue],
                    &is_cancelled,
            ));
        } else {
            commands.push(app_command_report(
                    "no single printer queue could be selected automatically",
            ));
        }

        finish_if_cancelled!();
        if use_admin {
            commands.push(run_admin_printer_fix_command(
                    should_start_cups(&diagnostics_before),
                    selected_queue.as_deref(),
                    &is_cancelled,
            ));
            finish_if_cancelled!();
            detected_queues = discover_printer_queues();
            if let Some(queue) =
            choose_repair_queue(selected_queue.as_deref(), None, &detected_queues)
            {
                if selected_queue.as_deref() != Some(queue.as_str()) {
                    self.set_selected_printer_queue(Some(&queue))?;
                    changed_selected_queue = true;
                    commands.push(app_command_report(format!(
                                "selected printer queue changed to {queue}"
                    )));
                }
            } else if selected_queue.is_none() {
                commands.push(app_command_report(
                        "admin repair could not select a printer queue automatically",
                ));
            }
        }

        let diagnostics_after = self.printer_diagnostics()?;
        Ok(PrinterFixResult {
                used_admin: use_admin,
                selected_queue: self.selected_printer_queue.clone(),
                changed_selected_queue,
                commands,
                diagnostics_before,
                diagnostics_after,
        })
    }
}

fn diagnostic_commands(selected_queue: Option<&str>) -> Vec<PrinterCommandReport> {
    let mut commands = vec![
        run_printer_command("lpstat", &["-r"]),
        run_printer_command("lpstat", &["-d"]),
        run_printer_command("lpstat", &["-a"]),
        run_printer_command("lpstat", &["-p"]),
        run_printer_command("lpstat", &["-t"]),
    ];
    if let Some(queue) = selected_queue {
        commands.push(run_printer_command("lpstat", &["-a", queue]));
        commands.push(run_printer_command("lpstat", &["-p", queue, "-l"]));
        commands.push(run_printer_command("lpq", &["-P", queue]));
        commands.push(run_printer_command("lpoptions", &["-p", queue, "-l"]));
    }
    commands.push(cups_journal_command());
    commands.push(run_printer_command(
            "tail",
            &["-n", "80", "/var/log/cups/error_log"],
    ));
    commands
}

fn cups_journal_command() -> PrinterCommandReport {
    run_printer_command(
        "journalctl",
        &[
            "-u",
            "cups",
            "--since",
            "30 minutes ago",
            "--no-pager",
            "-n",
            "80",
        ],
    )
}

fn recent_printer_failures(service: &mut TicketService) -> Result<Vec<PrintRecord>> {
    let mut recent_failures = Vec::new();
    for ticket in service.storage.list_tickets()? {
        for record in service.storage.list_recent_prints(&ticket.id, 20)? {
            if record.status == "failed" || record.failure_reason.is_some() {
                recent_failures.push(record);
            }
        }
    }
    recent_failures.sort_by(|left, right| right.recorded_at.cmp(&left.recorded_at));
    recent_failures.truncate(12);
    Ok(recent_failures)
}
