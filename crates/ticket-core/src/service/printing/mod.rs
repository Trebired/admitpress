use super::*;

mod bulk_process;
mod plan;
mod preview;
mod process;

use plan::*;

fn preview_path_component(value: &str) -> String {
    let cleaned = value
    .chars()
    .map(|character| match character {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' => character,
            _ => '-',
    })
    .collect::<String>();
    let trimmed = cleaned.trim_matches('-');
    if trimmed.is_empty() {
        "preview".to_owned()
    } else {
        trimmed.to_owned()
    }
}

impl TicketService {
    pub fn print_one(
        &mut self,
        ticket_id: &str,
        subticket_id: &str,
        local_date: &str,
        sale_currency: Option<&str>,
        sale_name: Option<&str>,
    ) -> Result<PrintResult> {
        validate_date(local_date)?;
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        self.ensure_sales_day_unlocked(ticket_id, local_date)?;
        let subticket = self
        .storage
        .get_subticket(subticket_id)?
        .ok_or(TicketError::SubticketNotFound)?;
        if subticket.ticket_id != ticket.id || !subticket.enabled {
            return Err(TicketError::SubticketNotFound.into());
        }
        let sale_currency = self.effective_sale_currency(sale_currency);
        let sold_amounts_minor =
        self.sold_amounts_for(std::slice::from_ref(&subticket), sale_currency.as_deref())?;
        let recorded_at = now_iso();
        let printer_name = self.selected_printer_queue.clone();
        let subticket_ids = [subticket_id.to_owned()];
        let records = self.storage.reserve_print_records(PrintReservation {
                ticket: &ticket,
                local_date,
                subticket_ids: &subticket_ids,
                printer_name: printer_name.as_deref(),
                is_bulk: false,
                bulk_operation_id: None,
                operation_label: sale_name
                .map(str::trim)
                .filter(|value| !value.is_empty()),
                recorded_at: &recorded_at,
                sold_currency: sale_currency.as_deref(),
                sold_amounts_minor: sold_amounts_minor.as_ref(),
        })?;
        let record = records
        .into_iter()
        .next()
        .ok_or_else(|| anyhow!("print record allocation failed"))?;
        let completed = self.process_record(&ticket, record)?;
        let next_number =
        self.storage
        .next_number_for_subticket_date(&ticket, subticket_id, local_date)?;
        let output_path = completed.rendered_output_path.clone().unwrap_or_default();
        Ok(PrintResult {
                formatted_number: format_number(&ticket, completed.allocated_number),
                record: completed,
                output_path,
                next_number,
        })
    }

    pub fn bulk_print(
        &mut self,
        ticket_id: &str,
        local_date: &str,
        requests: &[BulkRequest],
        options: BulkPrintOptions<'_>,
    ) -> Result<BulkPrintResult> {
        validate_date(local_date)?;
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        self.ensure_sales_day_unlocked(ticket_id, local_date)?;
        let trimmed_label = options
        .label
        .map(str::trim)
        .filter(|value| !value.is_empty());
        let BulkPrintPlan {
            subtickets,
            subticket_ids,
            summary: bulk_summary,
        } = self.bulk_print_plan(&ticket, requests, trimmed_label)?;
        let bulk_fields = if options.single_ticket {
            let fields = fields_for_template(self.storage.list_fields(ticket_id)?, "bulk");
            require_bulk_ticket_fields(&fields)?;
            fields
        } else {
            Vec::new()
        };

        let operation_id = Uuid::new_v4().to_string();
        if subticket_ids.is_empty() {
            return Ok(BulkPrintResult {
                    operation_id,
                    records: Vec::new(),
                    first_number: None,
                    final_number: None,
                    next_number: subtickets
                    .first()
                    .map(|subticket| {
                            self.storage.next_number_for_subticket_date(
                                &ticket,
                                &subticket.id,
                                local_date,
                            )
                    })
                    .transpose()?
                    .unwrap_or(ticket.number_start),
                    next_numbers: self.counter_states(&ticket, &subtickets, local_date)?,
            });
        }

        let sale_currency = self.effective_sale_currency(options.sale_currency);
        let sold_amounts_minor = self.sold_amounts_for(&subtickets, sale_currency.as_deref())?;
        let recorded_at = now_iso();
        let printer_name = self.selected_printer_queue.clone();
        let reserved = self.storage.reserve_print_records(PrintReservation {
                ticket: &ticket,
                local_date,
                subticket_ids: &subticket_ids,
                printer_name: printer_name.as_deref(),
                is_bulk: true,
                bulk_operation_id: Some(&operation_id),
                operation_label: trimmed_label,
                recorded_at: &recorded_at,
                sold_currency: sale_currency.as_deref(),
                sold_amounts_minor: sold_amounts_minor.as_ref(),
        })?;
        let first_number = reserved.first().map(|record| record.allocated_number);
        let final_number = reserved.last().map(|record| record.allocated_number);
        let records = if options.count_only {
            self.complete_without_outputs(reserved)?
        } else if options.single_ticket {
            self.process_bulk_records(&ticket, reserved, &bulk_fields, &bulk_summary)?
        } else {
            self.process_print_records(&ticket, reserved)?
        };
        let next_number = subticket_ids
        .first()
        .map(|subticket_id| {
                self.storage
                .next_number_for_subticket_date(&ticket, subticket_id, local_date)
        })
        .transpose()?
        .unwrap_or(ticket.number_start);
        Ok(BulkPrintResult {
                operation_id,
                records,
                first_number,
                final_number,
                next_number,
                next_numbers: self.counter_states(&ticket, &subtickets, local_date)?,
        })
    }

    pub fn adjust_counter(
        &mut self,
        ticket_id: &str,
        subticket_id: &str,
        local_date: &str,
        new_next_number: i64,
        reason: &str,
    ) -> Result<i64> {
        validate_date(local_date)?;
        if new_next_number < 1 {
            return Err(TicketError::InvalidCounter.into());
        }
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        self.ensure_sales_day_unlocked(ticket_id, local_date)?;
        let subticket = self
        .storage
        .get_subticket(subticket_id)?
        .ok_or(TicketError::SubticketNotFound)?;
        if subticket.ticket_id != ticket.id || !subticket.enabled {
            return Err(TicketError::SubticketNotFound.into());
        }
        self.storage.adjust_counter(
            &ticket,
            subticket_id,
            local_date,
            new_next_number,
            reason.trim(),
            &now_iso(),
        )?;
        Ok(new_next_number)
    }

    pub fn revert_last_sale(&mut self, ticket_id: &str, local_date: &str) -> Result<i64> {
        validate_date(local_date)?;
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        self.ensure_sales_day_unlocked(ticket_id, local_date)?;
        let (date_operations, _) = self.print_operations_bundle(ticket_id, local_date, 1)?;
        let last = date_operations
        .first()
        .ok_or_else(|| anyhow!("there is no sale to revert for {local_date}"))?
        .clone();

        let recorded_at = now_iso();
        let mut reverted = 0;
        for item in &last.items {
            let current = self.storage.next_number_for_subticket_date(
                &ticket,
                &item.subticket_id,
                local_date,
            )?;
            let rolled_back = current - item.quantity * ticket.number_increment;
            let new_next_number = rolled_back.max(ticket.number_start);
            if new_next_number == current {
                continue;
            }
            self.storage.adjust_counter(
                &ticket,
                &item.subticket_id,
                local_date,
                new_next_number,
                "Revert last sale",
                &recorded_at,
            )?;
            reverted += 1;
        }
        Ok(reverted)
    }

    pub fn reset_day(&mut self, ticket_id: &str, local_date: &str) -> Result<()> {
        validate_date(local_date)?;
        self.storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        self.ensure_sales_day_unlocked(ticket_id, local_date)?;
        self.storage.reset_day(ticket_id, local_date)
    }

    fn ensure_sales_day_unlocked(&self, ticket_id: &str, local_date: &str) -> Result<()> {
        let today = today_local();
        if local_date == today || !self.storage.day_has_sales(ticket_id, local_date)? {
            return Ok(());
        }
        Err(anyhow!(
                "sales for {local_date} are locked because that day already has data and today is {today}"
        ))
    }
}
