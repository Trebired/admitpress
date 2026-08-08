use super::*;

impl qobject::TicketBackend {
    pub fn print_one(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        subticket_id: &QString,
        local_date: &QString,
        sale_currency: &QString,
        sale_name: &QString,
    ) -> QString {
        let request =
        print_one_request(ticket_id, subticket_id, local_date, sale_currency, sale_name);
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => print_one_response(service, request),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn print_one_async(
        self: Pin<&mut Self>,
        ticket_id: &QString,
        subticket_id: &QString,
        local_date: &QString,
        sale_currency: &QString,
        sale_name: &QString,
    ) {
        let request =
        print_one_request(ticket_id, subticket_id, local_date, sale_currency, sale_name);
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| print_one_response(service, request));
                queue_async_response(qt_thread, "print_one", response, false);
        });
    }

    #[allow(clippy::too_many_arguments)]
    pub fn bulk_print(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        local_date: &QString,
        requests_json: &QString,
        label: &QString,
        count_only: bool,
        single_ticket: bool,
        sale_currency: &QString,
    ) -> QString {
        let request = bulk_print_request(
            ticket_id,
            local_date,
            requests_json,
            label,
            count_only,
            single_ticket,
            sale_currency,
        );
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => bulk_print_response(service, request),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn bulk_print_async(
        self: Pin<&mut Self>,
        ticket_id: &QString,
        local_date: &QString,
        requests_json: &QString,
        label: &QString,
        count_only: bool,
        single_ticket: bool,
        sale_currency: &QString,
    ) {
        let request = bulk_print_request(
            ticket_id,
            local_date,
            requests_json,
            label,
            count_only,
            single_ticket,
            sale_currency,
        );
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| bulk_print_response(service, request));
                queue_async_response(qt_thread, "bulk_print", response, false);
        });
    }

    pub fn preview_bulk_ticket(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        local_date: &QString,
        requests_json: &QString,
        label: &QString,
    ) -> QString {
        let request = bulk_print_request(
            ticket_id,
            local_date,
            requests_json,
            label,
            false,
            true,
            &QString::from(""),
        );
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => preview_bulk_ticket_response(service, request),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn update_subtickets(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        local_date: &QString,
        subtickets_json: &QString,
    ) -> QString {
        let ticket_id = ticket_id.to_string();
        let local_date = local_date.to_string();
        let subtickets_json = subtickets_json.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => {
                    update_subtickets_response(service, &ticket_id, &local_date, &subtickets_json)
                }
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn revert_last_sale(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        local_date: &QString,
    ) -> QString {
        let ticket_id = ticket_id.to_string();
        let local_date = local_date.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => revert_last_sale_response(service, &ticket_id, &local_date),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn adjust_counter(
        mut self: Pin<&mut Self>,
        ticket_id: &QString,
        subticket_id: &QString,
        local_date: &QString,
        new_next_number: i32,
        reason: &QString,
    ) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| {
                    service.adjust_counter(
                        &ticket_id.to_string(),
                        &subticket_id.to_string(),
                        &local_date.to_string(),
                        new_next_number as i64,
                        &reason.to_string(),
                    )
            })
        };
        let response = match result {
            Ok(next_number) => success(json!({ "next_number": next_number })),
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn reset_day(
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
                    service.reset_day(&ticket_id.to_string(), &local_date.to_string())
            })
        };
        let response = match result {
            Ok(()) => {
                self.as_mut().refresh();
                success(json!({ "reset": true }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn set_development_mode(mut self: Pin<&mut Self>, enabled: bool) -> QString {
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| service.set_development_mode(enabled))
        };
        let response = match result {
            Ok(()) => {
                self.as_mut().refresh();
                success(json!({ "development_mode": enabled }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn set_printer_queue(mut self: Pin<&mut Self>, printer_queue: &QString) -> QString {
        let queue = printer_queue.to_string();
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| service.set_selected_printer_queue(Some(&queue)))
        };
        let response = match result {
            Ok(()) => {
                self.as_mut().refresh();
                success(json!({ "selected_printer_queue": queue.trim() }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn printer_activity(mut self: Pin<&mut Self>) -> QString {
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => printer_activity_response(service),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn printer_diagnostics(mut self: Pin<&mut Self>) -> QString {
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => printer_diagnostics_response(service),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn printer_diagnostics_async(self: Pin<&mut Self>, request_id: &QString) {
        let request_id = request_id.to_string();
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(printer_diagnostics_response);
                queue_async_response_with_request(
                    qt_thread,
                    "printer_diagnostics",
                    response,
                    false,
                    Some(request_id),
                );
        });
    }

    pub fn fix_printer(mut self: Pin<&mut Self>, use_admin: bool) -> QString {
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => fix_printer_response(service, use_admin),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        if response_is_ok(&response) {
            self.as_mut().refresh();
        }
        QString::from(response.as_str())
    }

    pub fn fix_printer_async(self: Pin<&mut Self>, use_admin: bool) {
        let generation = PRINTER_FIX_GENERATION.fetch_add(1, Ordering::SeqCst) + 1;
        PRINTER_FIX_CANCELLED.store(false, Ordering::SeqCst);
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| {
                        fix_printer_response_cancelable(service, use_admin, || {
                                PRINTER_FIX_CANCELLED.load(Ordering::SeqCst)
                                || PRINTER_FIX_GENERATION.load(Ordering::SeqCst) != generation
                        })
                });
                if PRINTER_FIX_GENERATION.load(Ordering::SeqCst) == generation {
                    queue_async_response(qt_thread, "fix_printer", response, true);
                }
        });
    }

    pub fn cancel_printer_fix(self: Pin<&mut Self>) -> QString {
        let _ = self;
        PRINTER_FIX_CANCELLED.store(true, Ordering::SeqCst);
        PRINTER_FIX_GENERATION.fetch_add(1, Ordering::SeqCst);
        QString::from(success(json!({ "cancelled": true })).as_str())
    }
}
