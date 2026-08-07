use super::{DayExportRow, DayExportSheet, append_zip_file, xml_escape};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

const ODS_MIMETYPE: &str = "application/vnd.oasis.opendocument.spreadsheet";

pub(super) fn write_ods(destination: &Path, sheet: &DayExportSheet) -> Result<()> {
    let file = fs::File::create(destination)
    .with_context(|| format!("failed to create {}", destination.display()))?;
    let mut archive = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    append_zip_file(&mut archive, options, "mimetype", ODS_MIMETYPE.to_owned())?;
    append_zip_file(&mut archive, options, "META-INF/manifest.xml", manifest())?;
    append_zip_file(&mut archive, options, "content.xml", content(sheet))?;
    append_zip_file(&mut archive, options, "styles.xml", styles())?;
    append_zip_file(&mut archive, options, "meta.xml", meta(sheet))?;
    archive.finish()?;
    Ok(())
}

fn content(sheet: &DayExportSheet) -> String {
    let mut rows = vec![ods_row(&[
                OdsCell::Text("Date".to_owned()),
                OdsCell::Text("Tickets".to_owned()),
                OdsCell::Text("Total".to_owned()),
                OdsCell::Text("Breakdown".to_owned()),
    ])];
    for row in &sheet.rows {
        rows.push(day_row(row));
    }
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <office:document-content
        xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:table="urn:oasis:names:tc:opendocument:xmlns:table:1.0"
        xmlns:text="urn:oasis:names:tc:opendocument:xmlns:text:1.0" office:version="1.2">
        <office:body><office:spreadsheet><table:table table:name="Days">{}</table:table></office:spreadsheet></office:body>
        </office:document-content>"#,
        rows.join("")
    )
}

fn day_row(row: &DayExportRow) -> String {
    ods_row(&[
            OdsCell::Text(row.local_date.clone()),
            OdsCell::Number(row.ticket_count),
            OdsCell::Text(row.total_text.clone()),
            OdsCell::Text(row.breakdown.clone()),
    ])
}

fn ods_row(cells: &[OdsCell]) -> String {
    let values = cells.iter().map(ods_cell).collect::<Vec<_>>().join("");
    format!("<table:table-row>{values}</table:table-row>")
}

fn ods_cell(cell: &OdsCell) -> String {
    match cell {
        OdsCell::Text(value) => format!(
            r#"<table:table-cell office:value-type="string"><text:p>{}</text:p></table:table-cell>"#,
            xml_escape(value)
        ),
        OdsCell::Number(value) => format!(
            r#"<table:table-cell office:value-type="float" office:value="{value}"><text:p>{value}</text:p></table:table-cell>"#
        ),
    }
}

enum OdsCell {
    Text(String),
    Number(i64),
}

fn manifest() -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <manifest:manifest xmlns:manifest="urn:oasis:names:tc:opendocument:xmlns:manifest:1.0" manifest:version="1.2">
        <manifest:file-entry manifest:full-path="/" manifest:media-type="{ODS_MIMETYPE}"/>
        <manifest:file-entry manifest:full-path="content.xml" manifest:media-type="text/xml"/>
        <manifest:file-entry manifest:full-path="styles.xml" manifest:media-type="text/xml"/>
        <manifest:file-entry manifest:full-path="meta.xml" manifest:media-type="text/xml"/>
        </manifest:manifest>"#
    )
}

fn styles() -> String {
    r#"<?xml version="1.0" encoding="UTF-8"?>
    <office:document-styles xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" office:version="1.2"/>"#
    .to_owned()
}

fn meta(sheet: &DayExportSheet) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
        <office:document-meta
        xmlns:office="urn:oasis:names:tc:opendocument:xmlns:office:1.0" xmlns:meta="urn:oasis:names:tc:opendocument:xmlns:meta:1.0"
        office:version="1.2">
        <office:meta><meta:generator>Ticket</meta:generator><meta:creation-date>{}</meta:creation-date><meta:title>{}</meta:title></office:meta>
        </office:document-meta>"#,
        xml_escape(&sheet.exported_at),
        xml_escape(&sheet.ticket_name)
    )
}
