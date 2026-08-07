use crate::error::TicketError;
use crate::models::{
    ApplicationState, BackupInfo, BackupManifest, BulkPrintOptions, BulkPrintResult, BulkRequest, CounterAdjustment,
    CurrencyOption, DailyCounter, DynamicKind, FieldKind, ImportedImage,
    PrintHistoryEntry, PrintHistoryItem, PrintRecord, PrintResult, PrinterActivity,
    PrinterActivityJob, PrinterCommandReport, PrinterDiagnostics, PrinterFixResult, ReleaseStatus,
    RenderContext,
    Subticket, SubticketCounterState, SubticketInput, SubticketSummary, Ticket, TicketField,
    TicketFieldInput, TicketSummary,
};
use crate::renderer::{format_money, format_number, render_ticket_svg, save_svg};
use crate::storage::{PrintReservation, Storage};
use crate::templates::{normalize_template_kind, valid_template_kind};
use anyhow::{Context, Result, anyhow};
use chrono::{DateTime, Local, NaiveDate};
use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{Cursor, Read};
use std::path::{Component, Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};
use tar::{Archive, Builder, Header};
use uuid::Uuid;

mod archive;
mod backup;
mod config;
mod currency;
mod day {
    mod export;
}
mod history;
mod image;
mod lifecycle;
mod printer;
mod printing;
mod release;
mod render;
mod ticket;
mod utils;

#[cfg(test)]
mod tests;

use config::*;
use image::*;
use printer::*;
use render::*;
use utils::*;

const PRINTER_COMMAND_TIMEOUT: Duration = Duration::from_secs(3);
const ADMIN_PRINTER_COMMAND_TIMEOUT: Duration = Duration::from_secs(120);
const PREVIEW_MAX_PIXEL_DIMENSION: f64 = 2048.0;
const TICKET_ARCHIVE_FORMAT: &str = "ticket.archive";
const TICKET_ARCHIVE_VERSION: u32 = 1;
const TICKET_ARCHIVE_MANIFEST: &str = "ticket-archive.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TicketArchiveManifest {
    format: String,
    version: u32,
    exported_at: String,
    ticket: Ticket,
    fields: Vec<TicketField>,
    subtickets: Vec<Subticket>,
    daily_counters: Vec<DailyCounter>,
    print_records: Vec<PrintRecord>,
    counter_adjustments: Vec<CounterAdjustment>,
    assets: Vec<TicketArchiveAsset>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TicketArchiveAsset {
    kind: String,
    original_path: String,
    archive_path: String,
}

struct ImportedTicketArchive {
    ticket: Ticket,
    fields: Vec<TicketField>,
    subtickets: Vec<Subticket>,
    daily_counters: Vec<DailyCounter>,
    print_records: Vec<PrintRecord>,
    counter_adjustments: Vec<CounterAdjustment>,
}

pub struct TicketService {
    storage: Storage,
    data_dir: PathBuf,
    development_mode: bool,
    selected_printer_queue: Option<String>,
    cached_printer_queues: Vec<String>,
    main_currency: String,
}
