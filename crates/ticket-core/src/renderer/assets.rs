use super::*;

pub(super) fn read_svg_image(path: &str, width_mm: f64, height_mm: f64) -> Result<String> {
    let source =
    fs::read(path).with_context(|| format!("failed to read SVG background at {path}"))?;
    let source_text = String::from_utf8_lossy(&source);
    if !source_text.contains("<svg") {
        return Err(anyhow::anyhow!(
                "background file does not contain an SVG root element"
        ));
    }
    let encoded = STANDARD.encode(source);
    Ok(format!(
            r#"<image x="0" y="0" width="{width_mm}" height="{height_mm}" preserveAspectRatio="none" href="data:image/svg+xml;base64,{encoded}"/>"#
    ))
}

pub(super) fn read_ticket_image(path: &str) -> Result<String> {
    let source = fs::read(path).with_context(|| format!("failed to read image at {path}"))?;
    let mime_type = image_mime_type(path)?;
    let encoded = STANDARD.encode(source);
    Ok(format!("data:{mime_type};base64,{encoded}"))
}

pub(super) fn image_mime_type(path: &str) -> Result<&'static str> {
    match Path::new(path)
    .extension()
    .and_then(|value| value.to_str())
    .map(|value| value.to_lowercase())
    .as_deref()
    {
        Some("svg") => Ok("image/svg+xml"),
        Some("png") => Ok("image/png"),
        Some("jpg") | Some("jpeg") => Ok("image/jpeg"),
        Some("gif") => Ok("image/gif"),
        Some("bmp") => Ok("image/bmp"),
        Some("webp") => Ok("image/webp"),
        _ => Err(anyhow::anyhow!(
                "image fields support SVG, PNG, JPEG, GIF, BMP, or WebP files"
        )),
    }
}
