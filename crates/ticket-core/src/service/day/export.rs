mod ods;
mod xlsx;

use self::ods::write_ods;
use self::xlsx::write_xlsx;
use crate::error::TicketError;
use crate::models::DaySalesSummary;
pub(super) use crate::renderer::escape_xml_value as xml_escape;
use crate::service::{
    TicketService, decode_file_path, format_money, now_iso, path_to_file_url, validate_date,
};
use anyhow::{Result, anyhow};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::ZipWriter;
use zip::write::SimpleFileOptions;

pub(super) struct DayExportSheet {
    pub ticket_name: String,
    pub exported_at: String,
    pub rows: Vec<DayExportRow>,
}

pub(super) struct DayExportRow {
    pub local_date: String,
    pub ticket_count: i64,
    pub total_text: String,
    pub breakdown: String,
}

enum DayExportFormat {
    Xlsx,
    Ods,
}

impl TicketService {
    pub fn export_day_sales(
        &mut self,
        ticket_id: &str,
        local_dates: &[String],
        destination_path: &str,
    ) -> Result<String> {
        if local_dates.is_empty() {
            return Err(anyhow!("select at least one day to export"));
        }
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let destination = day_export_destination_path(destination_path);
        let format = day_export_format(&destination)?;
        let summaries = self.day_export_summaries(ticket_id, local_dates)?;
        let sheet = DayExportSheet {
            ticket_name: ticket.name,
            exported_at: now_iso(),
            rows: summaries.iter().map(day_export_row).collect(),
        };
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        match format {
            DayExportFormat::Xlsx => write_xlsx(&destination, &sheet)?,
            DayExportFormat::Ods => write_ods(&destination, &sheet)?,
        }
        Ok(path_to_file_url(&destination))
    }

    fn day_export_summaries(
        &mut self,
        ticket_id: &str,
        local_dates: &[String],
    ) -> Result<Vec<DaySalesSummary>> {
        let mut summaries = Vec::new();
        for local_date in local_dates {
            validate_date(local_date)?;
            if summaries
            .iter()
            .any(|summary: &DaySalesSummary| summary.local_date == *local_date)
            {
                continue;
            }
            let summary = self
            .storage
            .list_day_sales(ticket_id, local_date, 1)?
            .into_iter()
            .find(|summary| summary.local_date == *local_date)
            .ok_or_else(|| anyhow!("day summary was not found for {local_date}"))?;
            summaries.push(summary);
        }
        Ok(summaries)
    }
}

fn day_export_destination_path(destination_path: &str) -> PathBuf {
    let mut destination = PathBuf::from(decode_file_path(destination_path));
    if destination.extension().is_none() {
        destination.set_extension("xlsx");
    }
    destination
}

fn day_export_format(destination: &Path) -> Result<DayExportFormat> {
    match destination
    .extension()
    .and_then(|extension| extension.to_str())
    .map(str::to_ascii_lowercase)
    .as_deref()
    {
        Some("xlsx") => Ok(DayExportFormat::Xlsx),
        Some("ods") => Ok(DayExportFormat::Ods),
        _ => Err(anyhow!("day export path must end in .xlsx or .ods")),
    }
}

fn day_export_row(summary: &DaySalesSummary) -> DayExportRow {
    DayExportRow {
        local_date: summary.local_date.clone(),
        ticket_count: summary.ticket_count,
        total_text: format_money(summary.total_minor, &summary.currency),
        breakdown: day_export_breakdown(summary),
    }
}

fn day_export_breakdown(summary: &DaySalesSummary) -> String {
    if summary.subtickets.is_empty() {
        return "No sales".to_owned();
    }
    summary
    .subtickets
    .iter()
    .map(|subticket| {
            format!(
                "{}x {}",
                subticket.quantity,
                format_money(subticket.amount_minor, &subticket.currency)
            )
    })
    .collect::<Vec<_>>()
    .join(", ")
}

pub(super) fn append_zip_file(
    archive: &mut ZipWriter<fs::File>,
    options: SimpleFileOptions,
    path: &str,
    contents: String,
) -> Result<()> {
    archive.start_file(path, options)?;
    archive.write_all(contents.as_bytes())?;
    Ok(())
}
