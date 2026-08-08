use super::*;

fn profile_page_size(profile: PrinterProfile) -> PageSize {
    PageSize {
        width_mm: profile.width_mm,
        height_mm: profile.height_mm,
    }
}

impl TicketService {
    pub fn printer_profiles(&self) -> Result<Vec<PrinterProfile>> {
        self.storage.list_printer_profiles()
    }

    pub fn printer_profile(&self, queue: &str) -> Result<Option<PrinterProfile>> {
        let queue = queue.trim();
        if queue.is_empty() {
            return Ok(None);
        }
        self.storage.get_printer_profile(queue)
    }

    pub fn printer_page_size(&self, queue: &str) -> Result<PageSize> {
        Ok(self
            .printer_profile(queue)?
            .map(profile_page_size)
            .unwrap_or_else(PageSize::fallback))
    }

    pub(crate) fn ticket_calibration(&self, ticket: &Ticket) -> Result<PrintCalibration> {
        let Some(queue) = ticket
        .printer_queue
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        else {
            return Ok(PrintCalibration::none());
        };
        Ok(self
            .printer_profile(queue)?
            .map(|profile| profile.calibration())
            .unwrap_or_else(PrintCalibration::none))
    }

    pub fn set_printer_page_size(
        &mut self,
        queue: &str,
        width_cm: f64,
        height_cm: f64,
    ) -> Result<PrinterProfile> {
        let queue = queue.trim();
        if queue.is_empty() {
            return Err(TicketError::PrinterNotConfigured.into());
        }
        if width_cm <= 0.0 || height_cm <= 0.0 {
            return Err(TicketError::InvalidDimensions.into());
        }
        let recorded_at = now_iso();
        self.storage.upsert_printer_page_size(
            queue,
            width_cm * 10.0,
            height_cm * 10.0,
            &recorded_at,
        )?;
        self.require_printer_profile(queue)
    }

    pub fn set_printer_calibration(
        &mut self,
        queue: &str,
        calibration: PrintCalibration,
    ) -> Result<PrinterProfile> {
        let queue = queue.trim();
        if queue.is_empty() {
            return Err(TicketError::PrinterNotConfigured.into());
        }
        let recorded_at = now_iso();
        let fallback = PageSize::fallback();
        self.storage.upsert_printer_calibration(
            queue,
            calibration,
            fallback.width_mm,
            fallback.height_mm,
            &recorded_at,
        )?;
        self.require_printer_profile(queue)
    }

    fn require_printer_profile(&self, queue: &str) -> Result<PrinterProfile> {
        self.storage
        .get_printer_profile(queue)?
        .ok_or_else(|| anyhow!("printer profile for {queue} could not be stored"))
    }

    pub fn print_calibration_sheet(
        &mut self,
        queue: &str,
        page: PageSize,
        calibration: PrintCalibration,
    ) -> Result<String> {
        let queue = queue.trim();
        if queue.is_empty() {
            return Err(TicketError::PrinterNotConfigured.into());
        }
        if page.width_mm <= 0.0 || page.height_mm <= 0.0 {
            return Err(TicketError::InvalidDimensions.into());
        }
        let calibration = calibration.normalized();
        let svg = render_calibration_svg(page, queue, calibration);
        let svg_path = self.calibration_output_path(queue);
        save_svg(&svg_path, &svg)?;
        if self.development_mode {
            return Ok(svg_path.to_string_lossy().into_owned());
        }
        ensure_printer_ready(queue)?;
        let pdf_path = svg_path.with_extension("pdf");
        render_print_pdf(&svg_path, &pdf_path, page)?;
        submit_cups_job(queue, page, &pdf_path)?;
        Ok(pdf_path.to_string_lossy().into_owned())
    }

    fn calibration_output_path(&self, queue: &str) -> PathBuf {
        let root = if self.development_mode {
            self.data_dir.join("development-prints")
        } else {
            self.data_dir.join("print-jobs")
        };
        let stem = preview_path_component(queue);
        root.join("calibration").join(format!("{stem}.svg"))
    }
}
