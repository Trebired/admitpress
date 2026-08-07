use super::*;

impl qobject::TicketBackend {
    #[allow(clippy::too_many_arguments)]
    pub fn save_ticket(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        name: &QString,
        width_cm: f64,
        height_cm: f64,
        printer_queue: &QString,
        border_style: &QString,
        border_width_mm: f64,
        dpi: i32,
        number_start: i32,
        number_increment: i32,
        number_padding: i32,
        number_prefix: &QString,
        number_suffix: &QString,
        fields_json: &QString,
        subtickets_json: &QString,
    ) -> QString {
        let request = save_ticket_request(
            ticket_id,
            name,
            width_cm,
            height_cm,
            printer_queue,
            border_style,
            border_width_mm,
            dpi,
            number_start,
            number_increment,
            number_padding,
            number_prefix,
            number_suffix,
            fields_json,
            subtickets_json,
        );
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => save_ticket_response(service, request),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn save_ticket_async(
        self: Pin<&mut Self>,
        ticket_id: &QString,
        name: &QString,
        width_cm: f64,
        height_cm: f64,
        printer_queue: &QString,
        border_style: &QString,
        border_width_mm: f64,
        dpi: i32,
        number_start: i32,
        number_increment: i32,
        number_padding: i32,
        number_prefix: &QString,
        number_suffix: &QString,
        fields_json: &QString,
        subtickets_json: &QString,
    ) {
        let request = save_ticket_request(
            ticket_id,
            name,
            width_cm,
            height_cm,
            printer_queue,
            border_style,
            border_width_mm,
            dpi,
            number_start,
            number_increment,
            number_padding,
            number_prefix,
            number_suffix,
            fields_json,
            subtickets_json,
        );
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| save_ticket_response(service, request));
                queue_async_response(qt_thread, "save_ticket", response, false);
        });
    }

    pub fn import_background(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        source_path: &QString,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| {
                    service.import_background(&ticket_id.to_string(), &source_path.to_string())
            })
        };
        let response = match result {
            Ok(path) => success(json!({ "path": path })),
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn import_image(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        source_path: &QString,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| {
                    service.import_image(&ticket_id.to_string(), &source_path.to_string())
            })
        };
        QString::from(
            match result {
                Ok(image) => success(json!(image)),
                Err(error) => failure(error),
            }
            .as_str(),
        )
    }

    pub fn delete_ticket(mut self: Pin<&mut Self>, ticket_id: &QString) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| service.delete_ticket(&ticket_id.to_string()))
        };
        let response = match result {
            Ok(()) => {
                self.as_mut().refresh();
                success(json!({ "deleted": true }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn export_ticket_archive(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        destination_path: &QString,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| {
                    service.export_ticket_archive(
                        &ticket_id.to_string(),
                        &destination_path.to_string(),
                    )
            })
        };
        QString::from(
            match result {
                Ok(path) => success(json!({ "path": path })),
                Err(error) => failure(error),
            }
            .as_str(),
        )
    }

    pub fn import_ticket_archive(mut self: Pin<&mut Self>, source_path: &QString) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| service.import_ticket_archive(&source_path.to_string()))
        };
        let response = match result {
            Ok(ticket) => {
                self.as_mut().refresh();
                success(json!({ "ticket_id": ticket.id, "name": ticket.name }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn export_days(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        local_dates_json: &QString,
        destination_path: &QString,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .map(|service| {
                    export_days_response(
                        service,
                        &ticket_id.to_string(),
                        &local_dates_json.to_string(),
                        &destination_path.to_string(),
                    )
            })
        };
        QString::from(result.unwrap_or_else(failure).as_str())
    }

    pub fn preview_ticket(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        subticket_id: &QString,
        local_date: &QString,
        allocated_number: i32,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| {
                    service.preview_ticket(
                        &ticket_id.to_string(),
                        &subticket_id.to_string(),
                        &local_date.to_string(),
                        allocated_number as i64,
                    )
            })
        };
        QString::from(
            match result {
                Ok(path) => success(json!({ "path": path })),
                Err(error) => failure(error),
            }
            .as_str(),
        )
    }
}
