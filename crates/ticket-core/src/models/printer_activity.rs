use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterActivityJob {
    pub ticket_id: String,
    pub ticket_name: String,
    pub status: String,
    pub allocated_number: i64,
    pub is_bulk: bool,
    pub operation_label: Option<String>,
    pub recorded_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterActivity {
    pub queue: Option<String>,
    pub current: Option<PrinterActivityJob>,
    pub queued: Vec<PrinterActivityJob>,
}
