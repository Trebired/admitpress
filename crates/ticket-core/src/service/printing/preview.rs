use super::*;

fn preview_file_stem(
    ticket: &Ticket,
    subticket: &Subticket,
    local_date: &str,
    allocated_number: i64,
) -> String {
    let number = allocated_number.to_string();
    let raw = [
        ticket.id.as_str(),
        subticket.id.as_str(),
        local_date,
        number.as_str(),
        ticket.updated_at.as_str(),
        subticket.updated_at.as_str(),
    ]
    .join("-");
    preview_path_component(&raw)
}

fn bulk_preview_file_stem(
    ticket: &Ticket,
    local_date: &str,
    requests: &[BulkRequest],
    fields: &[TicketField],
    summary: &BulkRenderSummary,
) -> String {
    let requests_key = serde_json::to_string(requests).unwrap_or_default();
    let fields_key = fields
    .iter()
    .map(|field| field.updated_at.as_str())
    .collect::<Vec<_>>()
    .join("-");
    let raw = [
        ticket.id.as_str(),
        "bulk",
        local_date,
        summary.name.as_str(),
        requests_key.as_str(),
        ticket.updated_at.as_str(),
        fields_key.as_str(),
    ]
    .join("-");
    preview_path_component(&raw)
}

impl TicketService {
    pub fn preview_ticket(
        &mut self,
        ticket_id: &str,
        subticket_id: &str,
        local_date: &str,
        allocated_number: i64,
    ) -> Result<String> {
        validate_date(local_date)?;
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let subticket = self
        .storage
        .get_subticket(subticket_id)?
        .ok_or(TicketError::SubticketNotFound)?;
        if subticket.ticket_id != ticket.id || !subticket.enabled {
            return Err(TicketError::SubticketNotFound.into());
        }
        let fields = fields_for_template(self.storage.list_fields(ticket_id)?, "classic");
        let context = RenderContext {
            local_date: local_date.to_owned(),
            local_datetime: now_iso(),
            allocated_number,
            amount_minor: subticket.amount_minor,
            currency: self.printed_currency(None, &subticket.currency),
            subticket_name: subticket.name.clone(),
            bulk_name: String::new(),
            bulk_items: String::new(),
            bulk_count: 0,
            bulk_total_minor: 0,
            bulk_currency: String::new(),
        };
        let svg = render_ticket_svg(&ticket, &fields, &subticket, &context)?;
        let preview_stem = preview_file_stem(&ticket, &subticket, local_date, allocated_number);
        let svg_path = self
        .data_dir
        .join("previews")
        .join(format!("{preview_stem}.svg"));
        save_svg(&svg_path, &svg)?;

        let png_path = self
        .data_dir
        .join("previews")
        .join(format!("{preview_stem}.png"));
        match render_preview_png(&svg_path, &png_path, &ticket) {
            Ok(()) => Ok(path_to_file_url(&png_path)),
            Err(_) => Ok(String::new()),
        }
    }

    pub fn preview_bulk_ticket(
        &mut self,
        ticket_id: &str,
        local_date: &str,
        requests: &[BulkRequest],
        label: &str,
    ) -> Result<String> {
        validate_date(local_date)?;
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let trimmed_label = label.trim();
        let plan = self.bulk_print_plan(
            &ticket,
            requests,
            (!trimmed_label.is_empty()).then_some(trimmed_label),
        )?;
        let first_subticket_id = plan
        .subticket_ids
        .first()
        .ok_or_else(|| anyhow!("select at least one ticket for the group ticket preview"))?;
        let subticket = self
        .storage
        .get_subticket(first_subticket_id)?
        .ok_or(TicketError::SubticketNotFound)?;
        let fields = fields_for_template(self.storage.list_fields(ticket_id)?, "bulk");
        require_bulk_ticket_fields(&fields)?;
        let allocated_number =
        self.storage
        .next_number_for_subticket_date(&ticket, first_subticket_id, local_date)?;
        let currency = self.printed_currency(None, &plan.summary.currency);
        let context = bulk_render_context(local_date, allocated_number, &plan.summary, &currency);
        let svg = render_ticket_svg(&ticket, &fields, &subticket, &context)?;
        let preview_stem =
        bulk_preview_file_stem(&ticket, local_date, requests, &fields, &plan.summary);
        let svg_path = self
        .data_dir
        .join("previews")
        .join(format!("{preview_stem}.svg"));
        save_svg(&svg_path, &svg)?;

        let png_path = self
        .data_dir
        .join("previews")
        .join(format!("{preview_stem}.png"));
        match render_preview_png(&svg_path, &png_path, &ticket) {
            Ok(()) => Ok(path_to_file_url(&png_path)),
            Err(_) => Ok(String::new()),
        }
    }
}
