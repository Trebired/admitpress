use super::*;

impl qobject::TicketBackend {
    pub fn initialize(mut self: Pin<&mut Self>) {
        match TicketService::open_default() {
            Ok(service) => {
                self.as_mut().rust_mut().service = Some(service);
                self.as_mut().set_status_message(QString::from("Ready"));
                self.as_mut().refresh();
                self.backup_if_due_async();
            }
            Err(error) => {
                self.as_mut().set_status_message(QString::from(
                        format!("Initialization failed: {error:#}").as_str(),
                ));
            }
        }
    }

    pub fn refresh(mut self: Pin<&mut Self>) {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(TicketService::application_state)
        };
        match result {
            Ok(state) => {
                let text = serde_json::to_string(&state).unwrap_or_else(|_| "{}".to_owned());
                self.as_mut().set_state_json(QString::from(text.as_str()));
                self.as_mut().set_status_message(QString::from("Ready"));
            }
            Err(error) => {
                self.as_mut().set_status_message(QString::from(
                        format!("Refresh failed: {error:#}").as_str(),
                ));
            }
        }
    }

    pub fn set_window_icon(&self, dark: bool) {
        let asset = if dark { "icon-dark.svg" } else { "icon-light.svg" };
        let path = format!("{WINDOW_ICON_RESOURCE_ROOT}/{asset}");
        qobject::apply_window_icon(&QString::from(path.as_str()));
    }

    pub fn prefers_gnome_icons(&self) -> bool {
        let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase();
        if desktop.is_empty() {
            return false;
        }
        !desktop.split(':').any(|entry| matches!(entry, "kde" | "lxqt" | "razor"))
    }

    pub fn today(&self) -> QString {
        QString::from(
            chrono::Local::now()
            .date_naive()
            .format("%Y-%m-%d")
            .to_string()
            .as_str(),
        )
    }

    pub fn translate(&self, key: &QString, locale: &QString) -> QString {
        let locale = normalized_locale(&locale.to_string());
        QString::from(rust_i18n::t!(key.to_string(), locale = locale.as_str()).as_ref())
    }

    pub fn create_ticket(
        mut self: Pin<&mut Self>,
        name: &QString,
        width_cm: f64,
        height_cm: f64,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| service.create_ticket(&name.to_string(), width_cm, height_cm))
        };
        let response = match result {
            Ok(ticket) => {
                self.as_mut().refresh();
                success(json!({ "ticket_id": ticket.id }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn load_ticket(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        local_date: &QString,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| {
                    service.ticket_summary(&ticket_id.to_string(), &local_date.to_string())
            })
        };
        QString::from(
            match result {
                Ok(summary) => success(json!(summary)),
                Err(error) => failure(error),
            }
            .as_str(),
        )
    }

    pub fn set_data_directory(mut self: Pin<&mut Self>, source_path: &QString) -> QString {
        let result = TicketService::select_data_dir(&source_path.to_string());
        let response = match result {
            Ok(service) => {
                let data_dir = service.data_dir().to_string_lossy().into_owned();
                self.as_mut().rust_mut().service = Some(service);
                self.as_mut().refresh();
                success(json!({ "data_dir": data_dir }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }
}
