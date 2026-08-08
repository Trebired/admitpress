use super::*;

impl TicketService {
    pub(super) fn complete_without_output(&mut self, record: PrintRecord) -> Result<PrintRecord> {
        let completed_at = now_iso();
        self.storage.update_print_record(
            &record.id,
            "completed",
            None,
            None,
            None,
            Some(&completed_at),
            Some(&completed_at),
        )?;
        Ok(self.storage.get_print_record(&record.id)?.unwrap_or(record))
    }

    pub(super) fn process_print_records(
        &mut self,
        ticket: &Ticket,
        records: Vec<PrintRecord>,
    ) -> Result<Vec<PrintRecord>> {
        let mut processed = Vec::with_capacity(records.len());
        for record in records {
            match self.process_record(ticket, record.clone()) {
                Ok(completed) => processed.push(completed),
                Err(_) => {
                    let failed = self.storage.get_print_record(&record.id)?.unwrap_or(record);
                    processed.push(failed);
                }
            }
        }
        Ok(processed)
    }

    pub(super) fn process_record(
        &mut self,
        ticket: &Ticket,
        record: PrintRecord,
    ) -> Result<PrintRecord> {
        let subticket = self
        .storage
        .get_subticket(&record.subticket_id)?
        .ok_or(TicketError::SubticketNotFound)?;
        let fields = fields_for_template(self.storage.list_fields(&ticket.id)?, "classic");
        let context = RenderContext {
            local_date: record.local_date.clone(),
            local_datetime: now_iso(),
            allocated_number: record.allocated_number,
            amount_minor: record.sold_amount_minor.unwrap_or(subticket.amount_minor),
            currency: self.printed_currency(
                record.sold_currency.as_deref(),
                &subticket.currency,
            ),
            subticket_name: subticket.name.clone(),
            bulk_name: String::new(),
            bulk_items: String::new(),
            bulk_count: 0,
            bulk_total_minor: 0,
            bulk_currency: String::new(),
        };
        let started_at = now_iso();
        let physical_queue = if self.development_mode {
            None
        } else {
            let queue = match self
            .selected_printer_queue
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            {
                Some(queue) => queue.to_owned(),
                None => {
                    let ended_at = now_iso();
                    self.storage.update_print_record(
                        &record.id,
                        "failed",
                        None,
                        None,
                        Some("no printer queue is selected in settings"),
                        Some(&started_at),
                        Some(&ended_at),
                    )?;
                    return Err(TicketError::PrinterNotConfigured.into());
                }
            };
            if let Err(error) = ensure_printer_ready(&queue) {
                let ended_at = now_iso();
                self.storage.update_print_record(
                    &record.id,
                    "failed",
                    None,
                    None,
                    Some(&error.to_string()),
                    Some(&started_at),
                    Some(&ended_at),
                )?;
                return Err(error);
            }
            Some(queue)
        };
        let calibration = self.ticket_calibration(ticket)?;
        let svg = match render_calibrated_ticket_svg(ticket, &fields, &subticket, &context, calibration)
        {
            Ok(svg) => svg,
            Err(error) => {
                let ended_at = now_iso();
                self.storage.update_print_record(
                    &record.id,
                    "failed",
                    None,
                    None,
                    Some(&error.to_string()),
                    Some(&started_at),
                    Some(&ended_at),
                )?;
                return Err(error);
            }
        };

        let root = if self.development_mode {
            self.data_dir.join("development-prints")
        } else {
            self.data_dir.join("print-jobs")
        };
        let output_dir = root.join(&record.local_date).join(&ticket.id);
        let output_stem = format!(
            "{}-{}",
            format_number(ticket, record.allocated_number),
            record.id
        );
        let svg_output = output_dir.join(format!("{output_stem}.svg"));
        save_svg(&svg_output, &svg)?;

        if self.development_mode {
            let output_text = svg_output.to_string_lossy().into_owned();
            let ended_at = now_iso();
            self.storage.update_print_record(
                &record.id,
                "completed",
                Some(&output_text),
                None,
                None,
                Some(&started_at),
                Some(&ended_at),
            )?;
        } else {
            let svg_output_text = svg_output.to_string_lossy().into_owned();
            let queue = physical_queue
            .as_deref()
            .ok_or(TicketError::PrinterNotConfigured)?;
            let pdf_output = output_dir.join(format!("{output_stem}.pdf"));
            if let Err(error) = render_print_pdf(&svg_output, &pdf_output, ticket.page_size()) {
                let ended_at = now_iso();
                self.storage.update_print_record(
                    &record.id,
                    "failed",
                    Some(&svg_output_text),
                    None,
                    Some(&error.to_string()),
                    Some(&started_at),
                    Some(&ended_at),
                )?;
                return Err(error);
            }
            let output_text = pdf_output.to_string_lossy().into_owned();
            match submit_cups_job(queue, ticket.page_size(), &pdf_output) {
                Ok(job_id) => {
                    let ended_at = now_iso();
                    self.storage.update_print_record(
                        &record.id,
                        "queued",
                        Some(&output_text),
                        Some(&job_id),
                        None,
                        Some(&started_at),
                        Some(&ended_at),
                    )?;
                }
                Err(error) => {
                    let ended_at = now_iso();
                    self.storage.update_print_record(
                        &record.id,
                        "failed",
                        Some(&output_text),
                        None,
                        Some(&error.to_string()),
                        Some(&started_at),
                        Some(&ended_at),
                    )?;
                    return Err(error);
                }
            }
        }

        self.storage
        .get_print_record(&record.id)?
        .ok_or_else(|| anyhow!("updated print record was not found"))
    }
}
