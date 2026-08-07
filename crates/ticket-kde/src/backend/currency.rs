use super::*;

impl qobject::TicketBackend {
    pub fn ensure_main_currency(mut self: Pin<&mut Self>, locale: &QString) -> QString {
        let locale = locale.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => ensure_main_currency_response(service, &locale),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        if response_is_ok(&response) {
            self.as_mut().refresh();
        }
        QString::from(response.as_str())
    }

    pub fn set_main_currency(mut self: Pin<&mut Self>, currency: &QString) -> QString {
        let currency = currency.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => set_main_currency_response(service, &currency),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        if response_is_ok(&response) {
            self.as_mut().refresh();
        }
        QString::from(response.as_str())
    }

    pub fn format_money(&self, amount_minor: f64, currency: &QString) -> QString {
        let formatted =
        ticket_core::renderer::format_money(amount_minor.round() as i64, &currency.to_string());
        QString::from(formatted.as_str())
    }

    pub fn cached_convert_amount(
        mut self: Pin<&mut Self>,
        amount_minor: f64,
        currency: &QString,
        target_currency: &QString,
    ) -> QString {
        let currency = currency.to_string();
        let target_currency = target_currency.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => cached_convert_amount_response(
                    service,
                    amount_minor.round() as i64,
                    &currency,
                    &target_currency,
                ),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn refresh_exchange_rate_async(
        self: Pin<&mut Self>,
        from_currency: &QString,
        to_currency: &QString,
    ) {
        let from_currency = from_currency.to_string();
        let to_currency = to_currency.to_string();
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| {
                        refresh_exchange_rate_response(service, &from_currency, &to_currency)
                });
                queue_async_response(qt_thread, "refresh_exchange_rate", response, false);
        });
    }
}
