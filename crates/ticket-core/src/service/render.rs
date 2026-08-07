use super::*;

pub(super) fn render_print_pdf(svg_path: &Path, pdf_path: &Path, ticket: &Ticket) -> Result<()> {
    if let Some(parent) = pdf_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut failures = Vec::new();
    let width = mm_length(ticket.width_mm);
    let height = mm_length(ticket.height_mm);
    let rsvg_result = Command::new("rsvg-convert")
    .arg("--format")
    .arg("pdf")
    .arg("--page-width")
    .arg(&width)
    .arg("--page-height")
    .arg(&height)
    .arg("--width")
    .arg(&width)
    .arg("--height")
    .arg(&height)
    .arg("--output")
    .arg(pdf_path)
    .arg(svg_path)
    .output();

    match rsvg_result {
        Ok(output) if output.status.success() => return Ok(()),
        Ok(output) => failures.push(command_failure("rsvg-convert", &output)),
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
            failures.push(format!("rsvg-convert: {error}"));
        }
        Err(_) => {}
    }

    let inkscape_result = Command::new("inkscape")
    .arg(svg_path)
    .arg("--export-type=pdf")
    .arg("--export-area-page")
    .arg(format!("--export-filename={}", pdf_path.display()))
    .output();

    match inkscape_result {
        Ok(output) if output.status.success() => Ok(()),
        Ok(output) => {
            failures.push(command_failure("inkscape", &output));
            Err(TicketError::Rasterization(failures.join("; ")).into())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && failures.is_empty() => {
            Err(TicketError::RasterizerUnavailable.into())
        }
        Err(error) => {
            failures.push(format!("inkscape: {error}"));
            Err(TicketError::Rasterization(failures.join("; ")).into())
        }
    }
}

pub(super) fn render_preview_png(svg_path: &Path, png_path: &Path, ticket: &Ticket) -> Result<()> {
    if let Some(parent) = png_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut failures = Vec::new();
    let dpi = preview_dpi(ticket).to_string();
    let rsvg_result = Command::new("rsvg-convert")
    .arg("--format")
    .arg("png")
    .arg("--dpi-x")
    .arg(&dpi)
    .arg("--dpi-y")
    .arg(&dpi)
    .arg("--output")
    .arg(png_path)
    .arg(svg_path)
    .output();

    match rsvg_result {
        Ok(output) if output.status.success() && png_path.is_file() => return Ok(()),
        Ok(output) => failures.push(command_failure("rsvg-convert", &output)),
        Err(error) if error.kind() != std::io::ErrorKind::NotFound => {
            failures.push(format!("rsvg-convert: {error}"));
        }
        Err(_) => {}
    }

    let inkscape_result = Command::new("inkscape")
    .arg(svg_path)
    .arg("--export-type=png")
    .arg(format!("--export-filename={}", png_path.display()))
    .arg(format!("--export-dpi={dpi}"))
    .output();

    match inkscape_result {
        Ok(output) if output.status.success() && png_path.is_file() => Ok(()),
        Ok(output) => {
            failures.push(command_failure("inkscape", &output));
            Err(TicketError::Rasterization(failures.join("; ")).into())
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound && failures.is_empty() => {
            Err(TicketError::RasterizerUnavailable.into())
        }
        Err(error) => {
            failures.push(format!("inkscape: {error}"));
            Err(TicketError::Rasterization(failures.join("; ")).into())
        }
    }
}

pub(super) fn preview_dpi(ticket: &Ticket) -> i64 {
    let dpi = ticket.dpi.clamp(96, 300) as f64;
    let max_axis_mm = ticket.width_mm.max(ticket.height_mm).max(1.0);
    let max_dpi = PREVIEW_MAX_PIXEL_DIMENSION * 25.4 / max_axis_mm;
    dpi.min(max_dpi).max(24.0).round() as i64
}

pub(super) fn command_failure(command: &str, output: &std::process::Output) -> String {
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_owned();
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_owned();
    let detail = if !stderr.is_empty() {
        stderr
    } else if !stdout.is_empty() {
        stdout
    } else {
        format!("exited with {}", output.status)
    };
    format!("{command}: {detail}")
}

pub(super) fn submit_cups_job(queue: &str, ticket: &Ticket, output_path: &Path) -> Result<String> {
    let mut command = Command::new("lp");
    command.arg("-d").arg(queue);
    for option in cups_job_options(ticket) {
        command.arg("-o").arg(option);
    }
    let output = command
    .arg(output_path)
    .output()
    .context("failed to execute the CUPS lp command")?;

    if !output.status.success() {
        let message = String::from_utf8_lossy(&output.stderr).trim().to_owned();
        return Err(TicketError::PrintSubmission(message).into());
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let job_id = stdout
    .split_whitespace()
    .find(|token| {
            token.contains('-') && token.chars().any(|character| character.is_ascii_digit())
    })
    .unwrap_or("cups-job")
    .trim_end_matches('.')
    .to_owned();
    Ok(job_id)
}

pub(super) fn cups_job_options(ticket: &Ticket) -> Vec<String> {
    let media = cups_media_size(ticket);
    vec![
        format!("media={media}"),
        format!("PageSize={media}"),
        "print-scaling=none".to_owned(),
        "fit-to-page=false".to_owned(),
        "scaling=100".to_owned(),
        "position=center".to_owned(),
    ]
}

pub(super) fn cups_media_size(ticket: &Ticket) -> String {
    format!(
        "Custom.{}x{}mm",
        format_mm_value(ticket.width_mm),
        format_mm_value(ticket.height_mm)
    )
}

pub(super) fn mm_length(value: f64) -> String {
    format!("{}mm", format_mm_value(value))
}

pub(super) fn format_mm_value(value: f64) -> String {
    let text = format!("{:.2}", (value * 100.0).round() / 100.0);
    text.trim_end_matches('0').trim_end_matches('.').to_owned()
}
