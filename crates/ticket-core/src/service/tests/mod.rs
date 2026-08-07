use super::*;
use crate::models::DynamicKind;

mod archive;
mod backup;
mod bulk;
mod currency;
mod export;
mod printing;
mod release;
mod revert;
mod save;
mod settings;

fn service() -> TicketService {
    let path = std::env::temp_dir().join(format!("ticket-service-test-{}", Uuid::new_v4()));
    TicketService::open_at(path).expect("service should open")
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
