use serde::{Deserialize, Serialize};

mod backup;
mod currency;
mod print;
mod printer;
mod printer_activity;
mod release;
pub use backup::*;
pub use currency::*;
pub use print::*;
pub use printer::*;
pub use printer_activity::*;
pub use release::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Ticket {
    pub id: String,
    pub name: String,
    pub width_mm: f64,
    pub height_mm: f64,
    pub background_svg_path: Option<String>,
    pub printer_queue: Option<String>,
    pub border_style: String,
    pub border_width_mm: f64,
    pub dpi: i64,
    pub number_start: i64,
    pub number_increment: i64,
    pub number_padding: i64,
    pub number_prefix: String,
    pub number_suffix: String,
    pub recorded_at: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FieldKind {
    Static,
    Dynamic,
    Image,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DynamicKind {
    Price,
    Date,
    Number,
    BulkName,
    BulkItems,
    BulkCount,
    BulkTotal,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketField {
    pub id: String,
    pub ticket_id: String,
    pub template_kind: String,
    pub field_kind: FieldKind,
    pub dynamic_kind: Option<DynamicKind>,
    pub text: String,
    pub x_mm: f64,
    pub y_mm: f64,
    pub width_mm: f64,
    pub height_mm: f64,
    pub font_family: String,
    pub font_size_pt: f64,
    pub font_weight: i64,
    pub font_italic: bool,
    pub horizontal_alignment: String,
    pub vertical_alignment: String,
    pub date_format: String,
    pub prefix: String,
    pub suffix: String,
    pub sort_order: i64,
    pub recorded_at: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketFieldInput {
    #[serde(default)]
    pub id: String,
    #[serde(default = "default_template_kind")]
    pub template_kind: String,
    pub field_kind: FieldKind,
    pub dynamic_kind: Option<DynamicKind>,
    #[serde(default)]
    pub text: String,
    pub x_mm: f64,
    pub y_mm: f64,
    pub width_mm: f64,
    pub height_mm: f64,
    #[serde(default = "default_font_family")]
    pub font_family: String,
    #[serde(default = "default_font_size")]
    pub font_size_pt: f64,
    #[serde(default = "default_font_weight")]
    pub font_weight: i64,
    #[serde(default)]
    pub font_italic: bool,
    #[serde(default = "default_horizontal_alignment")]
    pub horizontal_alignment: String,
    #[serde(default = "default_vertical_alignment")]
    pub vertical_alignment: String,
    #[serde(default = "default_date_format")]
    pub date_format: String,
    #[serde(default)]
    pub prefix: String,
    #[serde(default)]
    pub suffix: String,
    #[serde(default)]
    pub sort_order: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Subticket {
    pub id: String,
    pub ticket_id: String,
    pub name: String,
    pub amount_minor: i64,
    pub currency: String,
    pub display_label: String,
    pub keyboard_shortcut: Option<String>,
    pub enabled: bool,
    pub sort_order: i64,
    pub recorded_at: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubticketInput {
    #[serde(default)]
    pub id: String,
    pub name: String,
    pub amount_minor: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default)]
    pub display_label: String,
    #[serde(default)]
    pub keyboard_shortcut: Option<String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
    #[serde(default)]
    pub sort_order: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubticketSummary {
    pub id: String,
    pub ticket_id: String,
    pub name: String,
    pub amount_minor: i64,
    pub currency: String,
    pub display_label: String,
    pub keyboard_shortcut: Option<String>,
    pub enabled: bool,
    pub sort_order: i64,
    pub recorded_at: String,
    pub updated_at: String,
    pub next_number_today: i64,
    pub sold_count_today: i64,
    pub sold_amount_minor_today: i64,
}

impl SubticketSummary {
    pub fn from_subticket(
        subticket: Subticket,
        next_number_today: i64,
        sold_count_today: i64,
    ) -> Self {
        let sold_amount_minor_today = sold_count_today * subticket.amount_minor;
        Self {
            id: subticket.id,
            ticket_id: subticket.ticket_id,
            name: subticket.name,
            amount_minor: subticket.amount_minor,
            currency: subticket.currency,
            display_label: subticket.display_label,
            keyboard_shortcut: subticket.keyboard_shortcut,
            enabled: subticket.enabled,
            sort_order: subticket.sort_order,
            recorded_at: subticket.recorded_at,
            updated_at: subticket.updated_at,
            next_number_today,
            sold_count_today,
            sold_amount_minor_today,
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DailyCounter {
    pub ticket_id: String,
    pub subticket_id: String,
    pub local_date: String,
    pub next_number: i64,
    pub recorded_at: String,
    pub updated_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrintRecord {
    pub id: String,
    pub ticket_id: String,
    pub subticket_id: String,
    pub local_date: String,
    pub allocated_number: i64,
    pub rendered_output_path: Option<String>,
    pub printer_name: Option<String>,
    pub printer_job_id: Option<String>,
    pub status: String,
    pub failure_reason: Option<String>,
    pub is_bulk: bool,
    pub bulk_operation_id: Option<String>,
    pub operation_label: Option<String>,
    pub recorded_at: String,
    pub started_at: Option<String>,
    pub ended_at: Option<String>,
    pub sold_currency: Option<String>,
    pub sold_amount_minor: Option<i64>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CounterAdjustment {
    pub id: String,
    pub ticket_id: String,
    pub subticket_id: String,
    pub local_date: String,
    pub previous_next_number: i64,
    pub new_next_number: i64,
    pub reason: String,
    pub recorded_at: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TicketSummary {
    pub ticket: Ticket,
    pub fields: Vec<TicketField>,
    pub subtickets: Vec<SubticketSummary>,
    pub next_number_today: i64,
    pub day_summaries: Vec<DaySalesSummary>,
    pub sales_dates: Vec<String>,
    pub date_print_operations: Vec<PrintHistoryEntry>,
    pub date_counter_adjustments: Vec<CounterAdjustment>,
    pub recent_prints: Vec<PrintRecord>,
    pub recent_print_operations: Vec<PrintHistoryEntry>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ApplicationState {
    pub tickets: Vec<TicketSummary>,
    pub printer_queues: Vec<String>,
    pub selected_printer_queue: Option<String>,
    pub development_mode: bool,
    pub data_dir: String,
    pub main_currency: String,
    pub available_currencies: Vec<CurrencyOption>,
    pub ui_language: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterCommandReport {
    pub command: String,
    pub exit_code: Option<i32>,
    pub success: bool,
    pub timed_out: bool,
    pub stdout: String,
    pub stderr: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterDiagnostics {
    pub selected_queue: Option<String>,
    pub development_mode: bool,
    pub detected_queues: Vec<String>,
    pub default_queue: Option<String>,
    pub problems: Vec<String>,
    pub suggested_commands: Vec<String>,
    pub recent_failures: Vec<PrintRecord>,
    pub commands: Vec<PrinterCommandReport>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PrinterFixResult {
    pub used_admin: bool,
    pub selected_queue: Option<String>,
    pub changed_selected_queue: bool,
    pub commands: Vec<PrinterCommandReport>,
    pub diagnostics_before: PrinterDiagnostics,
    pub diagnostics_after: PrinterDiagnostics,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImportedImage {
    pub path: String,
    pub width_px: Option<u32>,
    pub height_px: Option<u32>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DaySubticketSales {
    pub subticket_id: String,
    pub name: String,
    pub display_label: String,
    pub amount_minor: i64,
    pub currency: String,
    pub quantity: i64,
    pub total_minor: i64,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DaySalesSummary {
    pub local_date: String,
    pub ticket_count: i64,
    pub total_minor: i64,
    pub currency: String,
    pub subtickets: Vec<DaySubticketSales>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RenderContext {
    pub local_date: String,
    pub local_datetime: String,
    pub allocated_number: i64,
    pub amount_minor: i64,
    pub currency: String,
    pub subticket_name: String,
    pub bulk_name: String,
    pub bulk_items: String,
    pub bulk_count: i64,
    pub bulk_total_minor: i64,
    pub bulk_currency: String,
}

fn default_font_family() -> String {
    "Noto Sans".to_owned()
}

fn default_font_size() -> f64 {
    12.0
}

fn default_font_weight() -> i64 {
    400
}

fn default_horizontal_alignment() -> String {
    "left".to_owned()
}

fn default_vertical_alignment() -> String {
    "top".to_owned()
}

fn default_date_format() -> String {
    "date".to_owned()
}

fn default_currency() -> String {
    String::new()
}

fn default_template_kind() -> String {
    "classic".to_owned()
}

fn default_enabled() -> bool {
    true
}
