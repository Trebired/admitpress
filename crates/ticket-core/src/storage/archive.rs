use super::*;

impl Storage {
    pub fn insert_ticket_archive(
        &mut self,
        ticket: &Ticket,
        fields: &[TicketField],
        subtickets: &[Subticket],
        daily_counters: &[DailyCounter],
        print_records: &[PrintRecord],
        counter_adjustments: &[CounterAdjustment],
    ) -> Result<()> {
        let transaction = self
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute(
            r#"
            INSERT INTO tickets (
            id, name, width_mm, height_mm, background_svg_path, printer_queue,
            border_style, border_width_mm, dpi, number_start, number_increment,
            number_padding, number_prefix, number_suffix, recorded_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            params![
                &ticket.id,
                &ticket.name,
                ticket.width_mm,
                ticket.height_mm,
                ticket.background_svg_path.as_deref(),
                ticket.printer_queue.as_deref(),
                &ticket.border_style,
                ticket.border_width_mm,
                ticket.dpi,
                ticket.number_start,
                ticket.number_increment,
                ticket.number_padding,
                &ticket.number_prefix,
                &ticket.number_suffix,
                &ticket.recorded_at,
                &ticket.updated_at,
            ],
        )?;

        for field in fields {
            transaction.execute(
                r#"
                INSERT INTO ticket_fields (
                id, ticket_id, template_kind, field_kind, dynamic_kind, text, x_mm, y_mm,
                width_mm, height_mm, font_family, font_size_pt, font_weight,
                horizontal_alignment, vertical_alignment, date_format, prefix, suffix,
                sort_order, recorded_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    &field.id,
                    &field.ticket_id,
                    normalize_template_kind(&field.template_kind),
                    field_kind_to_db(&field.field_kind),
                    field.dynamic_kind.as_ref().map(dynamic_kind_to_db),
                    &field.text,
                    field.x_mm,
                    field.y_mm,
                    field.width_mm,
                    field.height_mm,
                    &field.font_family,
                    field.font_size_pt,
                    field.font_weight,
                    &field.horizontal_alignment,
                    &field.vertical_alignment,
                    normalize_date_format(&field.date_format),
                    &field.prefix,
                    &field.suffix,
                    field.sort_order,
                    &field.recorded_at,
                    &field.updated_at,
                ],
            )?;
        }

        for subticket in subtickets {
            transaction.execute(
                r#"
                INSERT INTO subtickets (
                id, ticket_id, name, amount_minor, currency, display_label,
                keyboard_shortcut, enabled, sort_order, recorded_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    &subticket.id,
                    &subticket.ticket_id,
                    &subticket.name,
                    subticket.amount_minor,
                    &subticket.currency,
                    &subticket.display_label,
                    subticket.keyboard_shortcut.as_deref(),
                    subticket.enabled as i64,
                    subticket.sort_order,
                    &subticket.recorded_at,
                    &subticket.updated_at,
                ],
            )?;
        }

        for counter in daily_counters {
            transaction.execute(
                "INSERT INTO daily_counters \
                (ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at) \
                VALUES (?, ?, ?, ?, ?, ?)",
                params![
                    &counter.ticket_id,
                    &counter.subticket_id,
                    &counter.local_date,
                    counter.next_number,
                    &counter.recorded_at,
                    &counter.updated_at,
                ],
            )?;
        }

        for record in print_records {
            transaction.execute(
                r#"
                INSERT INTO print_records (
                id, ticket_id, subticket_id, local_date, allocated_number,
                rendered_output_path, printer_name, printer_job_id, status,
                failure_reason, is_bulk, bulk_operation_id, recorded_at,
                started_at, ended_at, operation_label, sold_currency, sold_amount_minor
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    &record.id,
                    &record.ticket_id,
                    &record.subticket_id,
                    &record.local_date,
                    record.allocated_number,
                    record.rendered_output_path.as_deref(),
                    record.printer_name.as_deref(),
                    record.printer_job_id.as_deref(),
                    &record.status,
                    record.failure_reason.as_deref(),
                    record.is_bulk as i64,
                    record.bulk_operation_id.as_deref(),
                    &record.recorded_at,
                    record.started_at.as_deref(),
                    record.ended_at.as_deref(),
                    record.operation_label.as_deref(),
                    record.sold_currency.as_deref(),
                    record.sold_amount_minor,
                ],
            )?;
        }

        for adjustment in counter_adjustments {
            transaction.execute(
                "INSERT INTO counter_adjustments \
                (id, ticket_id, subticket_id, local_date, previous_next_number, new_next_number, \
                reason, recorded_at) \
                VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                params![
                    &adjustment.id,
                    &adjustment.ticket_id,
                    &adjustment.subticket_id,
                    &adjustment.local_date,
                    adjustment.previous_next_number,
                    adjustment.new_next_number,
                    &adjustment.reason,
                    &adjustment.recorded_at,
                ],
            )?;
        }

        transaction.commit()?;
        Ok(())
    }
}
