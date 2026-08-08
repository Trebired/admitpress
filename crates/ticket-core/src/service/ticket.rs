use super::*;

impl TicketService {
    pub fn ticket_summary(&mut self, ticket_id: &str, local_date: &str) -> Result<TicketSummary> {
        validate_date(local_date)?;
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let fields = self.storage.list_fields(ticket_id)?;
        let raw_subtickets = self.storage.list_subtickets(ticket_id)?;
        let subtickets = self.subticket_summaries(&ticket, raw_subtickets, local_date)?;
        let next_number_today = subtickets
        .first()
        .map(|subticket| subticket.next_number_today)
        .unwrap_or(ticket.number_start);
        let day_summaries = self.storage.list_day_sales(ticket_id, local_date, 31)?;
        let sales_dates = self.storage.list_sales_dates(ticket_id)?;
        let recent_prints = self.storage.list_recent_prints(ticket_id, 30)?;
        let (date_print_operations, recent_print_operations) =
        self.print_operations_bundle(ticket_id, local_date, 30)?;
        let date_counter_adjustments = self
        .storage
        .list_counter_adjustments_for_date(ticket_id, local_date)?;
        Ok(TicketSummary {
                ticket,
                fields,
                subtickets,
                next_number_today,
                day_summaries,
                sales_dates,
                date_print_operations,
                date_counter_adjustments,
                recent_prints,
                recent_print_operations,
        })
    }

    pub fn update_subtickets(
        &mut self,
        ticket_id: &str,
        local_date: &str,
        subtickets: &[SubticketInput],
    ) -> Result<TicketSummary> {
        self.storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        if subtickets
        .iter()
        .any(|subticket| subticket.name.trim().is_empty() || subticket.amount_minor < 0)
        {
            return Err(TicketError::InvalidSubticket.into());
        }
        let updated_at = now_iso();
        self.storage
        .replace_subtickets(ticket_id, subtickets, &updated_at)?;
        self.ticket_summary(ticket_id, local_date)
    }

    pub fn create_ticket(&mut self, name: &str, width_cm: f64, height_cm: f64) -> Result<Ticket> {
        self.create_ticket_for_printer(name, width_cm, height_cm, None)
    }

    pub fn create_ticket_for_printer(
        &mut self,
        name: &str,
        width_cm: f64,
        height_cm: f64,
        printer_queue: Option<&str>,
    ) -> Result<Ticket> {
        let name = name.trim();
        if name.is_empty() {
            return Err(TicketError::EmptyTicketName.into());
        }
        let queue = printer_queue
        .map(str::trim)
        .filter(|value| !value.is_empty());
        let page = match queue {
            Some(queue) => self.printer_page_size(queue)?,
            None => PageSize {
                width_mm: width_cm * 10.0,
                height_mm: height_cm * 10.0,
            },
        };
        if page.width_mm <= 0.0 || page.height_mm <= 0.0 {
            return Err(TicketError::InvalidDimensions.into());
        }

        let recorded_at = now_iso();
        let ticket = self
        .storage
        .create_ticket(name, page.width_mm, page.height_mm, &recorded_at)?;
        let Some(queue) = queue else {
            return Ok(ticket);
        };
        self.storage
        .set_ticket_printer_queue(&ticket.id, queue, &recorded_at)?;
        self.storage
        .get_ticket(&ticket.id)?
        .ok_or_else(|| TicketError::TicketNotFound.into())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save_ticket(
        &mut self,
        ticket_id: &str,
        name: &str,
        width_cm: f64,
        height_cm: f64,
        printer_queue: Option<&str>,
        border_style: &str,
        border_width_mm: f64,
        dpi: i64,
        number_start: i64,
        number_increment: i64,
        number_padding: i64,
        number_prefix: &str,
        number_suffix: &str,
        fields: &[TicketFieldInput],
        subtickets: &[SubticketInput],
    ) -> Result<()> {
        if name.trim().is_empty() {
            return Err(TicketError::EmptyTicketName.into());
        }
        if width_cm <= 0.0 || height_cm <= 0.0 {
            return Err(TicketError::InvalidDimensions.into());
        }
        let width_mm = width_cm * 10.0;
        let height_mm = height_cm * 10.0;
        let border_style = normalize_border_style(border_style);
        let border_width_mm = normalize_border_width_mm(border_width_mm);
        if fields.iter().any(|field| {
                field.x_mm < 0.0
                || field.y_mm < 0.0
                || field.width_mm <= 0.0
                || field.height_mm <= 0.0
                || !valid_template_kind(&field.template_kind)
                || field.x_mm + field.width_mm > width_mm + 0.001
                || field.y_mm + field.height_mm > height_mm + 0.001
        }) {
            return Err(TicketError::InvalidField.into());
        }
        if subtickets
        .iter()
        .any(|subticket| subticket.name.trim().is_empty() || subticket.amount_minor < 0)
        {
            return Err(TicketError::InvalidSubticket.into());
        }
        let existing = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let updated_at = now_iso();
        let assigned_queue = printer_queue
        .map(str::trim)
        .filter(|value| !value.is_empty());
        if let Some(queue) = assigned_queue {
            self.storage
            .upsert_printer_page_size(queue, width_mm, height_mm, &updated_at)?;
        }
        self.storage.update_ticket(
            ticket_id,
            name.trim(),
            width_mm,
            height_mm,
            existing.background_svg_path.as_deref(),
            assigned_queue,
            border_style,
            border_width_mm,
            dpi,
            number_start.max(1),
            number_increment.max(1),
            number_padding.max(0),
            number_prefix,
            number_suffix,
            &updated_at,
        )?;
        self.storage
        .replace_fields(ticket_id, fields, &updated_at)?;
        self.storage
        .replace_subtickets(ticket_id, subtickets, &updated_at)?;
        Ok(())
    }

    pub fn import_background(&mut self, ticket_id: &str, source_path: &str) -> Result<String> {
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let source = PathBuf::from(decode_file_path(source_path));
        if !source.is_file() {
            return Err(anyhow!("selected SVG file does not exist"));
        }
        if source.extension().and_then(|value| value.to_str()) != Some("svg") {
            return Err(anyhow!("background must be an SVG file"));
        }
        let target_dir = self.data_dir.join("tickets").join(ticket_id);
        fs::create_dir_all(&target_dir)?;
        let target = target_dir.join("background.svg");
        fs::copy(&source, &target).with_context(|| {
                format!(
                    "failed to copy SVG background from {} to {}",
                    source.display(),
                    target.display()
                )
        })?;
        let updated_at = now_iso();
        let target_text = target.to_string_lossy().into_owned();
        self.storage.update_ticket(
            ticket_id,
            &ticket.name,
            ticket.width_mm,
            ticket.height_mm,
            Some(&target_text),
            ticket.printer_queue.as_deref(),
            &ticket.border_style,
            ticket.border_width_mm,
            ticket.dpi,
            ticket.number_start,
            ticket.number_increment,
            ticket.number_padding,
            &ticket.number_prefix,
            &ticket.number_suffix,
            &updated_at,
        )?;
        Ok(target_text)
    }

    pub fn import_image(&mut self, ticket_id: &str, source_path: &str) -> Result<ImportedImage> {
        self.storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let source = PathBuf::from(decode_file_path(source_path));
        if !source.is_file() {
            return Err(anyhow!("selected image file does not exist"));
        }

        let extension = source
        .extension()
        .and_then(|value| value.to_str())
        .and_then(supported_image_extension)
        .ok_or_else(|| anyhow!("image must be SVG, PNG, JPEG, GIF, BMP, or WebP"))?;
        let source_bytes = fs::read(&source)
        .with_context(|| format!("failed to read image from {}", source.display()))?;
        let dimensions = image_dimensions(&source_bytes, extension);
        let target_dir = self.data_dir.join("tickets").join(ticket_id).join("images");
        fs::create_dir_all(&target_dir)?;
        let target = target_dir.join(format!("{}.{}", Uuid::new_v4(), extension));
        fs::write(&target, source_bytes).with_context(|| {
                format!(
                    "failed to copy image from {} to {}",
                    source.display(),
                    target.display()
                )
        })?;
        let target_text = target.to_string_lossy().into_owned();
        Ok(ImportedImage {
                path: target_text,
                width_px: dimensions.map(|(width, _)| width),
                height_px: dimensions.map(|(_, height)| height),
        })
    }

    pub fn delete_ticket(&mut self, ticket_id: &str) -> Result<()> {
        self.storage.delete_ticket(ticket_id)?;
        let ticket_dir = self.data_dir.join("tickets").join(ticket_id);
        if ticket_dir.exists() {
            fs::remove_dir_all(ticket_dir)?;
        }
        Ok(())
    }
}
