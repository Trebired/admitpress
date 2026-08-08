use crate::models::{
    CounterAdjustment, DailyCounter, DaySalesSummary, DaySubticketSales, DynamicKind, FieldKind,
    PrintCalibration, PrintRecord, PrinterProfile, Subticket, SubticketInput, Ticket, TicketField,
    TicketFieldInput,
};
use crate::templates::normalize_template_kind;
use anyhow::{Context, Result};
use chrono::{Days, NaiveDate};
use rusqlite::{Connection, OptionalExtension, Row, TransactionBehavior, params};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use uuid::Uuid;

mod archive;
mod backup;
mod counters;
mod currency;
mod fields;
mod mapping;
mod printers;
mod records;
mod schema;
mod settings;
mod subtickets;
mod tickets;

use mapping::*;

pub struct Storage {
    connection: Connection,
}

pub struct PrintReservation<'a> {
    pub ticket: &'a Ticket,
    pub local_date: &'a str,
    pub subticket_ids: &'a [String],
    pub printer_name: Option<&'a str>,
    pub is_bulk: bool,
    pub bulk_operation_id: Option<&'a str>,
    pub operation_label: Option<&'a str>,
    pub recorded_at: &'a str,
    pub sold_currency: Option<&'a str>,
    pub sold_amounts_minor: Option<&'a HashMap<String, i64>>,
}
