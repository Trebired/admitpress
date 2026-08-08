use super::*;

impl TicketService {
    pub(super) fn complete_without_outputs(
        &mut self,
        records: Vec<PrintRecord>,
    ) -> Result<Vec<PrintRecord>> {
        let mut completed = Vec::with_capacity(records.len());
        for record in records {
            completed.push(self.complete_without_output(record)?);
        }
        Ok(completed)
    }

    pub(super) fn process_bulk_records(
        &mut self,
        ticket: &Ticket,
        records: Vec<PrintRecord>,
        fields: &[TicketField],
        summary: &BulkRenderSummary,
    ) -> Result<Vec<PrintRecord>> {
        let Some(first_record) = records.first() else {
            return Ok(Vec::new());
        };
        let subticket = self
        .storage
        .get_subticket(&first_record.subticket_id)?
        .ok_or(TicketError::SubticketNotFound)?;
        let started_at = now_iso();
        let physical_queue = match self.bulk_physical_queue() {
            Ok(queue) => queue,
            Err(reason) => return self.fail_bulk_records(&records, &started_at, &reason),
        };
        let currency =
        self.printed_currency(first_record.sold_currency.as_deref(), &summary.currency);
        let context = bulk_render_context(
            &first_record.local_date,
            first_record.allocated_number,
            summary,
            &currency,
        );
        let calibration = self.ticket_calibration(ticket)?;
        let svg = match render_calibrated_ticket_svg(ticket, fields, &subticket, &context, calibration)
        {
            Ok(svg) => svg,
            Err(error) => return self.fail_bulk_records(&records, &started_at, &error.to_string()),
        };
        self.write_bulk_output(
            ticket,
            &records,
            &svg,
            physical_queue.as_deref(),
            &started_at,
        )
    }

    fn bulk_physical_queue(&self) -> std::result::Result<Option<String>, String> {
        if self.development_mode {
            return Ok(None);
        }
        let Some(queue) = self
        .selected_printer_queue
        .as_deref()
        .filter(|value| !value.trim().is_empty())
        else {
            return Err("no printer queue is selected in settings".to_owned());
        };
        if let Err(error) = ensure_printer_ready(queue) {
            return Err(error.to_string());
        }
        Ok(Some(queue.to_owned()))
    }

    fn write_bulk_output(
        &self,
        ticket: &Ticket,
        records: &[PrintRecord],
        svg: &str,
        physical_queue: Option<&str>,
        started_at: &str,
    ) -> Result<Vec<PrintRecord>> {
        let first_record = records
        .first()
        .ok_or_else(|| anyhow!("bulk record allocation failed"))?;
        let root = if self.development_mode {
            self.data_dir.join("development-prints")
        } else {
            self.data_dir.join("print-jobs")
        };
        let operation = first_record
        .bulk_operation_id
        .as_deref()
        .unwrap_or(first_record.id.as_str());
        let output_dir = root.join(&first_record.local_date).join(&ticket.id);
        let output_stem = preview_path_component(&format!("bulk-{operation}"));
        let svg_output = output_dir.join(format!("{output_stem}.svg"));
        if let Err(error) = save_svg(&svg_output, svg) {
            return self.fail_bulk_records(records, started_at, &error.to_string());
        }
        if self.development_mode {
            let output_text = svg_output.to_string_lossy().into_owned();
            return self.complete_bulk_records(records, started_at, &output_text);
        }
        let queue = physical_queue.ok_or(TicketError::PrinterNotConfigured)?;
        self.write_bulk_print_output(ticket, records, &svg_output, queue, started_at)
    }

    fn write_bulk_print_output(
        &self,
        ticket: &Ticket,
        records: &[PrintRecord],
        svg_output: &Path,
        queue: &str,
        started_at: &str,
    ) -> Result<Vec<PrintRecord>> {
        let pdf_output = svg_output.with_extension("pdf");
        let svg_output_text = svg_output.to_string_lossy().into_owned();
        if let Err(error) = render_print_pdf(svg_output, &pdf_output, ticket.page_size()) {
            return self.fail_bulk_records_with_output(
                records,
                started_at,
                &error.to_string(),
                &svg_output_text,
            );
        }
        let output_text = pdf_output.to_string_lossy().into_owned();
        match submit_cups_job(queue, ticket.page_size(), &pdf_output) {
            Ok(job_id) => self.queue_bulk_records(records, started_at, &output_text, &job_id),
            Err(error) => self.fail_bulk_records_with_output(
                records,
                started_at,
                &error.to_string(),
                &output_text,
            ),
        }
    }

    fn complete_bulk_records(
        &self,
        records: &[PrintRecord],
        started_at: &str,
        output_path: &str,
    ) -> Result<Vec<PrintRecord>> {
        let ended_at = now_iso();
        self.update_bulk_records(
            records,
            "completed",
            Some(output_path),
            None,
            None,
            Some(started_at),
            Some(&ended_at),
        )?;
        self.current_bulk_records(records)
    }

    fn queue_bulk_records(
        &self,
        records: &[PrintRecord],
        started_at: &str,
        output_path: &str,
        job_id: &str,
    ) -> Result<Vec<PrintRecord>> {
        let ended_at = now_iso();
        self.update_bulk_records(
            records,
            "queued",
            Some(output_path),
            Some(job_id),
            None,
            Some(started_at),
            Some(&ended_at),
        )?;
        self.current_bulk_records(records)
    }

    fn fail_bulk_records(
        &self,
        records: &[PrintRecord],
        started_at: &str,
        reason: &str,
    ) -> Result<Vec<PrintRecord>> {
        self.fail_bulk_records_with_path(records, started_at, reason, None)
    }

    fn fail_bulk_records_with_output(
        &self,
        records: &[PrintRecord],
        started_at: &str,
        reason: &str,
        output_path: &str,
    ) -> Result<Vec<PrintRecord>> {
        self.fail_bulk_records_with_path(records, started_at, reason, Some(output_path))
    }

    fn fail_bulk_records_with_path(
        &self,
        records: &[PrintRecord],
        started_at: &str,
        reason: &str,
        output_path: Option<&str>,
    ) -> Result<Vec<PrintRecord>> {
        let ended_at = now_iso();
        self.update_bulk_records(
            records,
            "failed",
            output_path,
            None,
            Some(reason),
            Some(started_at),
            Some(&ended_at),
        )?;
        self.current_bulk_records(records)
    }

    #[allow(clippy::too_many_arguments)]
    fn update_bulk_records(
        &self,
        records: &[PrintRecord],
        status: &str,
        rendered_output_path: Option<&str>,
        printer_job_id: Option<&str>,
        failure_reason: Option<&str>,
        started_at: Option<&str>,
        ended_at: Option<&str>,
    ) -> Result<()> {
        for record in records {
            self.storage.update_print_record(
                &record.id,
                status,
                rendered_output_path,
                printer_job_id,
                failure_reason,
                started_at,
                ended_at,
            )?;
        }
        Ok(())
    }

    fn current_bulk_records(&self, records: &[PrintRecord]) -> Result<Vec<PrintRecord>> {
        let mut current = Vec::with_capacity(records.len());
        for record in records {
            current.push(
                self.storage
                .get_print_record(&record.id)?
                .unwrap_or_else(|| record.clone()),
            );
        }
        Ok(current)
    }
}
