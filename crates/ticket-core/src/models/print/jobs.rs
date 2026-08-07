use crate::models::PrintRecord;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintResult {
    pub record: PrintRecord,
    pub formatted_number: String,
    pub output_path: String,
    pub next_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubticketCounterState {
    pub subticket_id: String,
    pub next_number: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkRequest {
    pub subticket_id: String,
    pub quantity: i64,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BulkPrintOptions<'a> {
    pub count_only: bool,
    pub single_ticket: bool,
    pub label: Option<&'a str>,
    pub sale_currency: Option<&'a str>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BulkPrintResult {
    pub operation_id: String,
    pub records: Vec<PrintRecord>,
    pub first_number: Option<i64>,
    pub final_number: Option<i64>,
    pub next_number: i64,
    pub next_numbers: Vec<SubticketCounterState>,
}
