use super::*;

const DEVELOPMENT_MODE_DEFAULT_ENV: &str = "TICKET_APP_DEVELOPMENT_MODE_DEFAULT";

impl TicketService {
    pub fn open_default() -> Result<Self> {
        let data_dir = std::env::var_os("TICKET_APP_DATA_DIR")
        .map(PathBuf::from)
        .or_else(read_configured_data_dir)
        .unwrap_or_else(default_data_dir);
        Self::open_at(data_dir)
    }

    pub fn select_data_dir(source_path: &str) -> Result<Self> {
        let data_dir = PathBuf::from(decode_file_path(source_path));
        let service = Self::open_at(data_dir.clone())?;
        write_configured_data_dir(&data_dir)?;
        Ok(service)
    }

    pub fn open_at(data_dir: PathBuf) -> Result<Self> {
        Self::open_at_with_development_default(data_dir, default_development_mode())
    }

    pub(super) fn open_at_with_development_default(
        data_dir: PathBuf,
        development_mode_default: bool,
    ) -> Result<Self> {
        fs::create_dir_all(&data_dir).map_err(|_| TicketError::DataDirectoryNotWritable)?;
        let data_dir = fs::canonicalize(&data_dir).unwrap_or(data_dir);
        fs::create_dir_all(data_dir.join("tickets"))?;
        fs::create_dir_all(data_dir.join("development-prints"))?;
        fs::create_dir_all(data_dir.join("print-jobs"))?;
        fs::create_dir_all(data_dir.join("previews"))?;
        fs::create_dir_all(data_dir.join("exports"))?;
        fs::create_dir_all(data_dir.join("backups"))?;

        let storage = Storage::open(&data_dir.join("application.sqlite"))?;
        let development_mode = storage
        .get_setting("development_mode")?
        .map(|value| value != "false")
        .unwrap_or(development_mode_default);
        let selected_printer_queue = match storage.get_setting("selected_printer_queue")? {
            Some(value) => normalized_optional(value),
            None => default_printer_queue(),
        };
        let main_currency = storage.get_setting("main_currency")?.unwrap_or_default();
        let ui_language = storage
        .get_setting("ui_language")?
        .and_then(|value| normalize_ui_language(&value).ok())
        .unwrap_or_else(|| "en".to_owned());

        Ok(Self {
                storage,
                data_dir,
                development_mode,
                selected_printer_queue,
                cached_printer_queues: Vec::new(),
                main_currency,
                ui_language,
        })
    }

    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    pub fn development_mode(&self) -> bool {
        self.development_mode
    }

    pub fn set_development_mode(&mut self, enabled: bool) -> Result<()> {
        self.development_mode = enabled;
        self.storage
        .set_setting("development_mode", if enabled { "true" } else { "false" })
    }

    pub fn ui_language(&self) -> &str {
        &self.ui_language
    }

    pub fn set_ui_language(&mut self, language: &str) -> Result<()> {
        let normalized = normalize_ui_language(language)?;
        self.storage.set_setting("ui_language", &normalized)?;
        self.ui_language = normalized;
        Ok(())
    }

    pub fn selected_printer_queue(&self) -> Option<&str> {
        self.selected_printer_queue.as_deref()
    }

    pub fn set_selected_printer_queue(&mut self, printer_queue: Option<&str>) -> Result<()> {
        self.selected_printer_queue = printer_queue.and_then(normalized_optional);
        self.storage.set_setting(
            "selected_printer_queue",
            self.selected_printer_queue.as_deref().unwrap_or_default(),
        )
    }

    pub fn application_state(&mut self) -> Result<ApplicationState> {
        let today = today_local();
        let mut summaries = Vec::new();
        for ticket in self.storage.list_tickets()? {
            let fields = self.storage.list_fields(&ticket.id)?;
            let raw_subtickets = self.storage.list_subtickets(&ticket.id)?;
            let subtickets = self.subticket_summaries(&ticket, raw_subtickets, &today)?;
            let next_number_today = subtickets
            .first()
            .map(|subticket| subticket.next_number_today)
            .unwrap_or(ticket.number_start);
            let day_summaries = self.storage.list_day_sales(&ticket.id, &today, 31)?;
            let sales_dates = self.storage.list_sales_dates(&ticket.id)?;
            let recent_prints = self.storage.list_recent_prints(&ticket.id, 12)?;
            let (date_print_operations, recent_print_operations) =
            self.print_operations_bundle(&ticket.id, &today, 12)?;
            let date_counter_adjustments = self
            .storage
            .list_counter_adjustments_for_date(&ticket.id, &today)?;
            summaries.push(TicketSummary {
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
            });
        }

        Ok(ApplicationState {
                tickets: summaries,
                printer_queues: self.cached_printer_queues.clone(),
                selected_printer_queue: self.selected_printer_queue.clone(),
                development_mode: self.development_mode,
                data_dir: self.data_dir.to_string_lossy().into_owned(),
                main_currency: self.main_currency.clone(),
                available_currencies: self.available_currencies(),
                ui_language: self.ui_language.clone(),
        })
    }

    pub(super) fn subticket_summaries(
        &self,
        ticket: &Ticket,
        subtickets: Vec<Subticket>,
        local_date: &str,
    ) -> Result<Vec<SubticketSummary>> {
        subtickets
        .into_iter()
        .filter(|subticket| subticket.enabled)
        .map(|subticket| {
                let next_number_today = self.storage.next_number_for_subticket_date(
                    ticket,
                    &subticket.id,
                    local_date,
                )?;
                let sold_count_today = self
                .storage
                .sold_count_for_subticket_date(&subticket.id, local_date)?;
                Ok(SubticketSummary::from_subticket(
                        subticket,
                        next_number_today,
                        sold_count_today,
                ))
        })
        .collect()
    }

    pub(super) fn counter_states(
        &self,
        ticket: &Ticket,
        subtickets: &[Subticket],
        local_date: &str,
    ) -> Result<Vec<SubticketCounterState>> {
        subtickets
        .iter()
        .map(|subticket| {
                Ok(SubticketCounterState {
                        subticket_id: subticket.id.clone(),
                        next_number: self.storage.next_number_for_subticket_date(
                            ticket,
                            &subticket.id,
                            local_date,
                        )?,
                })
        })
        .collect()
    }
}

fn default_development_mode() -> bool {
    std::env::var(DEVELOPMENT_MODE_DEFAULT_ENV)
    .map(|value| {
            matches!(
                value.trim().to_ascii_lowercase().as_str(),
                "1" | "true" | "yes" | "on"
            )
    })
    .unwrap_or(false)
}

fn normalize_ui_language(language: &str) -> Result<String> {
    let normalized = language.trim().to_lowercase().replace('-', "_");
    match normalized.split('_').next().unwrap_or("en") {
        "en" => Ok("en".to_owned()),
        "cs" => Ok("cs".to_owned()),
        _ => Err(anyhow!("unsupported language")),
    }
}
