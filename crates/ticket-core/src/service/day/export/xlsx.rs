use super::{DayExportRow, DayExportSheet, append_zip_file, xml_escape};
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;
use zip::write::SimpleFileOptions;
use zip::{CompressionMethod, ZipWriter};

pub(super) fn write_xlsx(destination: &Path, sheet: &DayExportSheet) -> Result<()> {
    let file = fs::File::create(destination)
    .with_context(|| format!("failed to create {}", destination.display()))?;
    let mut archive = ZipWriter::new(file);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
    append_zip_file(
        &mut archive,
        options,
        "[Content_Types].xml",
        content_types(),
    )?;
    append_zip_file(&mut archive, options, "_rels/.rels", package_rels())?;
    append_zip_file(&mut archive, options, "docProps/app.xml", app_props())?;
    append_zip_file(
        &mut archive,
        options,
        "docProps/core.xml",
        core_props(sheet),
    )?;
    append_zip_file(&mut archive, options, "xl/workbook.xml", workbook())?;
    append_zip_file(
        &mut archive,
        options,
        "xl/_rels/workbook.xml.rels",
        workbook_rels(),
    )?;
    append_zip_file(&mut archive, options, "xl/styles.xml", styles())?;
    append_zip_file(
        &mut archive,
        options,
        "xl/worksheets/sheet1.xml",
        worksheet(sheet),
    )?;
    archive.finish()?;
    Ok(())
}

fn worksheet(sheet: &DayExportSheet) -> String {
    let mut rows = vec![xlsx_row(
            1,
            &[
                XlsxCell::Text("Date".to_owned()),
                XlsxCell::Text("Tickets".to_owned()),
                XlsxCell::Text("Total".to_owned()),
                XlsxCell::Text("Breakdown".to_owned()),
            ],
    )];
    for (index, row) in sheet.rows.iter().enumerate() {
        rows.push(day_row(index + 2, row));
    }
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <worksheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
        <sheetData>{}</sheetData>
        </worksheet>"#,
        rows.join("")
    )
}

fn day_row(index: usize, row: &DayExportRow) -> String {
    xlsx_row(
        index,
        &[
            XlsxCell::Text(row.local_date.clone()),
            XlsxCell::Number(row.ticket_count),
            XlsxCell::Text(row.total_text.clone()),
            XlsxCell::Text(row.breakdown.clone()),
        ],
    )
}

fn xlsx_row(index: usize, cells: &[XlsxCell]) -> String {
    let values = cells
    .iter()
    .enumerate()
    .map(|(cell_index, cell)| cell_xml(cell, cell_reference(cell_index, index)))
    .collect::<Vec<_>>()
    .join("");
    format!(r#"<row r="{index}">{values}</row>"#)
}

fn cell_reference(cell_index: usize, row_index: usize) -> String {
    let column = (b'A' + cell_index as u8) as char;
    format!("{column}{row_index}")
}

fn cell_xml(cell: &XlsxCell, reference: String) -> String {
    match cell {
        XlsxCell::Text(value) => format!(
            r#"<c r="{reference}" t="inlineStr"><is><t>{}</t></is></c>"#,
            xml_escape(value)
        ),
        XlsxCell::Number(value) => format!(r#"<c r="{reference}"><v>{value}</v></c>"#),
    }
}

enum XlsxCell {
    Text(String),
    Number(i64),
}

fn content_types() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
    <Default Extension="rels" ContentType="application/vnd.openxmlformats-package.relationships+xml"/>
    <Default Extension="xml" ContentType="application/xml"/>
    <Override PartName="/docProps/app.xml" ContentType="application/vnd.openxmlformats-officedocument.extended-properties+xml"/>
    <Override PartName="/docProps/core.xml" ContentType="application/vnd.openxmlformats-package.core-properties+xml"/>
    <Override PartName="/xl/workbook.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml"/>
    <Override PartName="/xl/styles.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml"/>
    <Override PartName="/xl/worksheets/sheet1.xml" ContentType="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml"/>
    </Types>"#
    .to_owned()
}

fn package_rels() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument" Target="xl/workbook.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/package/2006/relationships/metadata/core-properties" Target="docProps/core.xml"/>
    <Relationship Id="rId3" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/extended-properties" Target="docProps/app.xml"/>
    </Relationships>"#
    .to_owned()
}

fn app_props() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <Properties
    xmlns="http://schemas.openxmlformats.org/officeDocument/2006/extended-properties"
    xmlns:vt="http://schemas.openxmlformats.org/officeDocument/2006/docPropsVTypes">
    <Application>Ticket</Application>
    </Properties>"#
    .to_owned()
}

fn core_props(sheet: &DayExportSheet) -> String {
    format!(
        r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
        <cp:coreProperties
        xmlns:cp="http://schemas.openxmlformats.org/package/2006/metadata/core-properties" xmlns:dc="http://purl.org/dc/elements/1.1/"
        xmlns:dcterms="http://purl.org/dc/terms/" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <dc:title>{}</dc:title>
        <dcterms:created xsi:type="dcterms:W3CDTF">{}</dcterms:created>
        </cp:coreProperties>"#,
        xml_escape(&sheet.ticket_name),
        xml_escape(&sheet.exported_at)
    )
}

fn workbook() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <workbook
    xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships">
    <sheets><sheet name="Days" sheetId="1" r:id="rId1"/></sheets>
    </workbook>"#
    .to_owned()
}

fn workbook_rels() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships">
    <Relationship Id="rId1" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet" Target="worksheets/sheet1.xml"/>
    <Relationship Id="rId2" Type="http://schemas.openxmlformats.org/officeDocument/2006/relationships/styles" Target="styles.xml"/>
    </Relationships>"#
    .to_owned()
}

fn styles() -> String {
    r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
    <styleSheet xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main">
    <fonts count="1"><font><sz val="11"/><name val="Noto Sans"/></font></fonts>
    <fills count="1"><fill><patternFill patternType="none"/></fill></fills>
    <borders count="1"><border><left/><right/><top/><bottom/><diagonal/></border></borders>
    <cellStyleXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0"/></cellStyleXfs>
    <cellXfs count="1"><xf numFmtId="0" fontId="0" fillId="0" borderId="0" xfId="0"/></cellXfs>
    </styleSheet>"#
    .to_owned()
}
