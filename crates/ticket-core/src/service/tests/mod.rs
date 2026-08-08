use super::*;
use crate::models::DynamicKind;

mod archive;
mod backup;
mod bulk;
mod calibration;
mod currency;
mod export;
mod printing;
mod printing_empty;
mod release;
mod revert;
mod save;
mod settings;

fn service() -> TicketService {
    let path = std::env::temp_dir().join(format!("ticket-service-test-{}", Uuid::new_v4()));
    let mut service = TicketService::open_at(path).expect("service should open");
    service
    .set_development_mode(true)
    .expect("test service should default to development output");
    service
}

fn field_inputs(summary: &TicketSummary) -> Vec<TicketFieldInput> {
    summary
    .fields
    .iter()
    .map(|field| TicketFieldInput {
            id: field.id.clone(),
            template_kind: field.template_kind.clone(),
            field_kind: field.field_kind.clone(),
            dynamic_kind: field.dynamic_kind.clone(),
            text: field.text.clone(),
            x_mm: field.x_mm,
            y_mm: field.y_mm,
            width_mm: field.width_mm,
            height_mm: field.height_mm,
            font_family: field.font_family.clone(),
            font_size_pt: field.font_size_pt,
            font_weight: field.font_weight,
            font_italic: field.font_italic,
            horizontal_alignment: field.horizontal_alignment.clone(),
            vertical_alignment: field.vertical_alignment.clone(),
            date_format: field.date_format.clone(),
            prefix: field.prefix.clone(),
            suffix: field.suffix.clone(),
            sort_order: field.sort_order,
    })
    .collect()
}

fn subticket_inputs(summary: &TicketSummary) -> Vec<SubticketInput> {
    summary
    .subtickets
    .iter()
    .map(|subticket| SubticketInput {
            id: subticket.id.clone(),
            name: subticket.name.clone(),
            amount_minor: subticket.amount_minor,
            currency: subticket.currency.clone(),
            display_label: subticket.display_label.clone(),
            keyboard_shortcut: subticket.keyboard_shortcut.clone(),
            enabled: subticket.enabled,
            sort_order: subticket.sort_order,
    })
    .collect()
}

fn sample_ticket_summary(
    service: &mut TicketService,
    ticket_id: &str,
    local_date: &str,
) -> TicketSummary {
    let recorded_at = now_iso();
    service
    .storage
    .replace_fields(
        ticket_id,
        &[TicketFieldInput {
                id: String::new(),
                template_kind: "classic".into(),
                field_kind: FieldKind::Dynamic,
                dynamic_kind: Some(DynamicKind::Number),
                text: String::new(),
                x_mm: 5.0,
                y_mm: 5.0,
                width_mm: 30.0,
                height_mm: 8.0,
                font_family: "Noto Sans".into(),
                font_size_pt: 12.0,
                font_weight: 400,
                font_italic: false,
                horizontal_alignment: "center".into(),
                vertical_alignment: "center".into(),
                date_format: "date".into(),
                prefix: "No. ".into(),
                suffix: String::new(),
                sort_order: 0,
        }],
        &recorded_at,
    )
    .expect("sample fields should save");
    service
    .storage
    .replace_subtickets(
        ticket_id,
        &[
            SubticketInput {
                id: String::new(),
                name: "Standard".into(),
                amount_minor: 1_000,
                currency: String::new(),
                display_label: "Standard".into(),
                keyboard_shortcut: Some("1".into()),
                enabled: true,
                sort_order: 0,
            },
            SubticketInput {
                id: String::new(),
                name: "Reduced".into(),
                amount_minor: 700,
                currency: String::new(),
                display_label: "Reduced".into(),
                keyboard_shortcut: Some("2".into()),
                enabled: true,
                sort_order: 1,
            },
            SubticketInput {
                id: String::new(),
                name: "Guest".into(),
                amount_minor: 300,
                currency: String::new(),
                display_label: "Guest".into(),
                keyboard_shortcut: Some("3".into()),
                enabled: true,
                sort_order: 2,
            },
        ],
        &recorded_at,
    )
    .expect("sample price variants should save");
    service
    .ticket_summary(ticket_id, local_date)
    .expect("summary should load")
}
