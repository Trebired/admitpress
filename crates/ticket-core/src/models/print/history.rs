use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintHistoryItem {
    pub subticket_id: String,
    pub name: String,
    pub display_label: String,
    pub amount_minor: i64,
    pub currency: String,
    pub quantity: i64,
    pub total_minor: i64,
    pub ticket_numbers: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintHistoryEntry {
    pub id: String,
    pub ticket_id: String,
    pub print_number: i64,
    pub local_date: String,
    pub is_bulk: bool,
    pub operation_label: Option<String>,
    pub status: String,
    pub failure_reason: Option<String>,
    pub ticket_count: i64,
    pub total_minor: i64,
    pub currency: String,
    pub recorded_at: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub rendered_output_path: Option<String>,
    pub printer_name: Option<String>,
    pub printer_job_id: Option<String>,
    pub items: Vec<PrintHistoryItem>,
}
