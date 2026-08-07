use super::*;

pub(super) fn map_ticket(row: &Row<'_>) -> rusqlite::Result<Ticket> {
    Ok(Ticket {
            id: row.get(0)?,
            name: row.get(1)?,
            width_mm: row.get(2)?,
            height_mm: row.get(3)?,
            background_svg_path: row.get(4)?,
            printer_queue: row.get(5)?,
            border_style: row.get(6)?,
            border_width_mm: row.get(7)?,
            dpi: row.get(8)?,
            number_start: row.get(9)?,
            number_increment: row.get(10)?,
            number_padding: row.get(11)?,
            number_prefix: row.get(12)?,
            number_suffix: row.get(13)?,
            recorded_at: row.get(14)?,
            updated_at: row.get(15)?,
    })
}

pub(super) fn map_field(row: &Row<'_>) -> rusqlite::Result<TicketField> {
    let template_kind: String = row.get(2)?;
    let field_kind: String = row.get(3)?;
    let dynamic_kind: Option<String> = row.get(4)?;
    Ok(TicketField {
            id: row.get(0)?,
            ticket_id: row.get(1)?,
            template_kind: normalize_template_kind(&template_kind).to_owned(),
            field_kind: field_kind_from_db(&field_kind),
            dynamic_kind: dynamic_kind.as_deref().map(dynamic_kind_from_db),
            text: row.get(5)?,
            x_mm: row.get(6)?,
            y_mm: row.get(7)?,
            width_mm: row.get(8)?,
            height_mm: row.get(9)?,
            font_family: row.get(10)?,
            font_size_pt: row.get(11)?,
            font_weight: row.get(12)?,
            horizontal_alignment: row.get(13)?,
            vertical_alignment: row.get(14)?,
            date_format: row.get(15)?,
            prefix: row.get(16)?,
            suffix: row.get(17)?,
            sort_order: row.get(18)?,
            recorded_at: row.get(19)?,
            updated_at: row.get(20)?,
    })
}

pub(super) fn map_subticket(row: &Row<'_>) -> rusqlite::Result<Subticket> {
    Ok(Subticket {
            id: row.get(0)?,
            ticket_id: row.get(1)?,
            name: row.get(2)?,
            amount_minor: row.get(3)?,
            currency: row.get(4)?,
            display_label: row.get(5)?,
            keyboard_shortcut: row.get(6)?,
            enabled: row.get::<_, i64>(7)? != 0,
            sort_order: row.get(8)?,
            recorded_at: row.get(9)?,
            updated_at: row.get(10)?,
    })
}

pub(super) fn map_counter(row: &Row<'_>) -> rusqlite::Result<DailyCounter> {
    Ok(DailyCounter {
            ticket_id: row.get(0)?,
            subticket_id: row.get(1)?,
            local_date: row.get(2)?,
            next_number: row.get(3)?,
            recorded_at: row.get(4)?,
            updated_at: row.get(5)?,
    })
}

pub(super) fn map_print_record(row: &Row<'_>) -> rusqlite::Result<PrintRecord> {
    Ok(PrintRecord {
            id: row.get(0)?,
            ticket_id: row.get(1)?,
            subticket_id: row.get(2)?,
            local_date: row.get(3)?,
            allocated_number: row.get(4)?,
            rendered_output_path: row.get(5)?,
            printer_name: row.get(6)?,
            printer_job_id: row.get(7)?,
            status: row.get(8)?,
            failure_reason: row.get(9)?,
            is_bulk: row.get::<_, i64>(10)? != 0,
            bulk_operation_id: row.get(11)?,
            recorded_at: row.get(12)?,
            started_at: row.get(13)?,
            ended_at: row.get(14)?,
            operation_label: row.get(15)?,
            sold_currency: row.get(16)?,
            sold_amount_minor: row.get(17)?,
    })
}

pub(super) fn map_counter_adjustment(row: &Row<'_>) -> rusqlite::Result<CounterAdjustment> {
    Ok(CounterAdjustment {
            id: row.get(0)?,
            ticket_id: row.get(1)?,
            subticket_id: row.get(2)?,
            local_date: row.get(3)?,
            previous_next_number: row.get(4)?,
            new_next_number: row.get(5)?,
            reason: row.get(6)?,
            recorded_at: row.get(7)?,
    })
}

pub(super) fn field_kind_to_db(kind: &FieldKind) -> &'static str {
    match kind {
        FieldKind::Static => "static",
        FieldKind::Dynamic => "dynamic",
        FieldKind::Image => "image",
    }
}

pub(super) fn field_kind_from_db(value: &str) -> FieldKind {
    match value {
        "dynamic" => FieldKind::Dynamic,
        "image" => FieldKind::Image,
        _ => FieldKind::Static,
    }
}

pub(super) fn dynamic_kind_to_db(kind: &DynamicKind) -> &'static str {
    match kind {
        DynamicKind::Price => "price",
        DynamicKind::Date => "date",
        DynamicKind::Number => "number",
        DynamicKind::BulkName => "bulk_name",
        DynamicKind::BulkItems => "bulk_items",
        DynamicKind::BulkCount => "bulk_count",
        DynamicKind::BulkTotal => "bulk_total",
    }
}

pub(super) fn dynamic_kind_from_db(value: &str) -> DynamicKind {
    match value {
        "date" => DynamicKind::Date,
        "number" => DynamicKind::Number,
        "bulk_name" => DynamicKind::BulkName,
        "bulk_items" => DynamicKind::BulkItems,
        "bulk_count" => DynamicKind::BulkCount,
        "bulk_total" => DynamicKind::BulkTotal,
        _ => DynamicKind::Price,
    }
}

pub(super) fn normalize_date_format(value: &str) -> &str {
    match value.trim() {
        "date_time" => "date_time",
        "full_date_time" => "full_date_time",
        "iso_date" => "iso_date",
        "iso_date_time" => "iso_date_time",
        "time" => "time",
        _ => "date",
    }
}
