pub(super) fn supported_image_extension(value: &str) -> Option<&'static str> {
    match value.trim().to_lowercase().as_str() {
        "svg" => Some("svg"),
        "png" => Some("png"),
        "jpg" | "jpeg" => Some("jpg"),
        "gif" => Some("gif"),
        "bmp" => Some("bmp"),
        "webp" => Some("webp"),
        _ => None,
    }
}

pub(super) fn image_dimensions(bytes: &[u8], extension: &str) -> Option<(u32, u32)> {
    match extension {
        "png" => png_dimensions(bytes),
        "jpg" => jpeg_dimensions(bytes),
        "svg" => svg_dimensions(bytes),
        _ => None,
    }
}

pub(super) fn png_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 24 || !bytes.starts_with(b"\x89PNG\r\n\x1A\n") {
        return None;
    }
    Some((read_be_u32(&bytes[16..20])?, read_be_u32(&bytes[20..24])?))
}

pub(super) fn jpeg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    if bytes.len() < 4 || bytes[0] != 0xFF || bytes[1] != 0xD8 {
        return None;
    }

    let mut index = 2;
    while index + 9 < bytes.len() {
        if bytes[index] != 0xFF {
            index += 1;
            continue;
        }
        while index < bytes.len() && bytes[index] == 0xFF {
            index += 1;
        }
        if index >= bytes.len() {
            return None;
        }

        let marker = bytes[index];
        index += 1;
        if marker == 0xD9 || marker == 0xDA {
            return None;
        }
        if index + 2 > bytes.len() {
            return None;
        }
        let length = u16::from_be_bytes([bytes[index], bytes[index + 1]]) as usize;
        if length < 2 || index + length > bytes.len() {
            return None;
        }
        if matches!(
            marker,
            0xC0 | 0xC1
            | 0xC2
            | 0xC3
            | 0xC5
            | 0xC6
            | 0xC7
            | 0xC9
            | 0xCA
            | 0xCB
            | 0xCD
            | 0xCE
            | 0xCF
        ) && length >= 7
        {
            let height = u16::from_be_bytes([bytes[index + 3], bytes[index + 4]]) as u32;
            let width = u16::from_be_bytes([bytes[index + 5], bytes[index + 6]]) as u32;
            return Some((width, height));
        }
        index += length;
    }
    None
}

pub(super) fn svg_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    let text = std::str::from_utf8(bytes).ok()?;
    if let Some(view_box) = svg_attribute(text, "viewBox") {
        let values = view_box
        .replace(',', " ")
        .split_whitespace()
        .filter_map(|value| value.parse::<f64>().ok())
        .collect::<Vec<_>>();
        if values.len() == 4 && values[2] > 0.0 && values[3] > 0.0 {
            return Some((values[2].round() as u32, values[3].round() as u32));
        }
    }

    let width = svg_length_attribute(text, "width")?;
    let height = svg_length_attribute(text, "height")?;
    Some((width.round() as u32, height.round() as u32))
}

pub(super) fn svg_attribute(text: &str, name: &str) -> Option<String> {
    let start = text.find(name)?;
    let after_name = &text[start + name.len()..];
    let equals = after_name.find('=')?;
    let after_equals = after_name[equals + 1..].trim_start();
    let quote = after_equals.chars().next()?;
    if quote != '"' && quote != '\'' {
        return None;
    }
    let value_start = quote.len_utf8();
    let value_end = after_equals[value_start..].find(quote)?;
    Some(after_equals[value_start..value_start + value_end].to_owned())
}

pub(super) fn svg_length_attribute(text: &str, name: &str) -> Option<f64> {
    let value = svg_attribute(text, name)?;
    let number = value
    .chars()
    .take_while(|character| character.is_ascii_digit() || *character == '.')
    .collect::<String>();
    number.parse::<f64>().ok().filter(|value| *value > 0.0)
}

pub(super) fn read_be_u32(bytes: &[u8]) -> Option<u32> {
    Some(u32::from_be_bytes(bytes.try_into().ok()?))
}
