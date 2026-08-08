use thiserror::Error;

#[derive(Debug, Error)]
pub enum TicketError {
    #[error("ticket name cannot be empty")]
    EmptyTicketName,
    #[error("ticket dimensions must be positive")]
    InvalidDimensions,
    #[error("ticket field dimensions and coordinates are invalid")]
    InvalidField,
    #[error("subticket names and prices must be valid")]
    InvalidSubticket,
    #[error("ticket was not found")]
    TicketNotFound,
    #[error("subticket was not found")]
    SubticketNotFound,
    #[error("Add content in the editor before printing this ticket.")]
    EmptyTicketDesign,
    #[error("the selected date must use YYYY-MM-DD format")]
    InvalidDate,
    #[error("quantity must be zero or greater")]
    InvalidQuantity,
    #[error("the next ticket number must be one or greater")]
    InvalidCounter,
    #[error("no printer queue is selected in settings")]
    PrinterNotConfigured,
    #[error(
            "printer queue {0} is disabled or rejecting jobs; enable it in printer settings or run: sudo cupsaccept {0} && sudo cupsenable {0}"
    )]
    PrinterQueueNotAccepting(String),
    #[error("the selected data directory is not writable")]
    DataDirectoryNotWritable,
    #[error("CUPS rejected the print job: {0}")]
    PrintSubmission(String),
    #[error("ticket print rendering requires rsvg-convert or Inkscape")]
    RasterizerUnavailable,
    #[error("ticket print rendering failed: {0}")]
    Rasterization(String),
}
