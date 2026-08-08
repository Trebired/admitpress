use super::*;

pub(super) struct BulkRenderSummary {
    pub name: String,
    pub items: String,
    pub count: i64,
    pub total_minor: i64,
    pub currency: String,
}

pub(super) struct BulkPrintPlan {
    pub subtickets: Vec<Subticket>,
    pub subticket_ids: Vec<String>,
    pub summary: BulkRenderSummary,
}

pub(super) fn fields_for_template(
    fields: Vec<TicketField>,
    template_kind: &str,
) -> Vec<TicketField> {
    fields
    .into_iter()
    .filter(|field| normalize_template_kind(&field.template_kind) == template_kind)
    .filter(|field| template_kind != "bulk" || !is_deprecated_bulk_dynamic_field(field))
    .collect()
}

fn is_deprecated_bulk_dynamic_field(field: &TicketField) -> bool {
    field.field_kind == FieldKind::Dynamic
    && matches!(
        field.dynamic_kind.as_ref(),
        Some(DynamicKind::BulkCount | DynamicKind::BulkTotal)
    )
}

pub(super) fn bulk_render_context(
    local_date: &str,
    allocated_number: i64,
    summary: &BulkRenderSummary,
    currency: &str,
) -> RenderContext {
    RenderContext {
        local_date: local_date.to_owned(),
        local_datetime: now_iso(),
        allocated_number,
        amount_minor: summary.total_minor,
        currency: currency.to_owned(),
        subticket_name: "Group ticket".to_owned(),
        bulk_name: summary.name.clone(),
        bulk_items: summary.items.clone(),
        bulk_count: summary.count,
        bulk_total_minor: summary.total_minor,
        bulk_currency: currency.to_owned(),
    }
}

pub(super) fn require_bulk_ticket_fields(fields: &[TicketField]) -> Result<()> {
    let required = [DynamicKind::BulkName, DynamicKind::BulkItems];
    let missing = required.iter().any(|kind| {
            !fields.iter().any(|field| {
                    field.field_kind == FieldKind::Dynamic && field.dynamic_kind.as_ref() == Some(kind)
            })
    });
    if missing {
        Err(anyhow!(
                "group ticket template must include Group name and Tickets fields"
        ))
    } else {
        Ok(())
    }
}

impl TicketService {
    pub(super) fn bulk_print_plan(
        &mut self,
        ticket: &Ticket,
        requests: &[BulkRequest],
        label: Option<&str>,
    ) -> Result<BulkPrintPlan> {
        let subtickets = self
        .storage
        .list_subtickets(&ticket.id)?
        .into_iter()
        .filter(|subticket| subticket.enabled)
        .collect::<Vec<_>>();
        let by_id = subtickets
        .iter()
        .map(|subticket| (subticket.id.clone(), subticket.clone()))
        .collect::<HashMap<_, _>>();
        let mut subticket_ids = Vec::new();
        let mut lines = Vec::new();
        let mut count = 0;
        let mut total_minor = 0;
        let mut currency = String::new();
        for request in requests {
            if request.quantity < 0 {
                return Err(TicketError::InvalidQuantity.into());
            }
            let subticket = by_id
            .get(&request.subticket_id)
            .ok_or(TicketError::SubticketNotFound)?;
            if request.quantity == 0 {
                continue;
            }
            for _ in 0..request.quantity {
                subticket_ids.push(request.subticket_id.clone());
            }
            let price_currency = self.current_price_currency(&subticket.currency);
            if currency.is_empty() {
                currency = price_currency.clone();
            }
            count += request.quantity;
            total_minor += request.quantity * subticket.amount_minor;
            lines.push(format!(
                    "{}x {}",
                    request.quantity,
                    format_money(subticket.amount_minor, &price_currency)
            ));
        }
        if !lines.is_empty() {
            lines.push(format!("Total: {}", format_money(total_minor, &currency)));
        }
        Ok(BulkPrintPlan {
                subtickets,
                subticket_ids,
                summary: BulkRenderSummary {
                    name: label.unwrap_or_default().trim().to_owned(),
                    items: lines.join("\n"),
                    count,
                    total_minor,
                    currency,
                },
        })
    }
}
