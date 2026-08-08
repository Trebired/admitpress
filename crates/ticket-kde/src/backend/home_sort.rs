use super::*;

fn set_home_sort_mode_response(service: &mut TicketService, mode: &str) -> String {
    match service.set_home_sort_mode(mode) {
        Ok(()) => success(json!({ "home_sort_mode": mode })),
        Err(error) => failure(error),
    }
}

fn set_home_ticket_order_response(service: &mut TicketService, order_json: &str) -> String {
    let order: Vec<String> = match serde_json::from_str(order_json) {
        Ok(value) => value,
        Err(error) => return failure(anyhow::anyhow!("invalid ticket order: {error}")),
    };
    match service.set_home_ticket_order(&order) {
        Ok(()) => success(json!({ "home_ticket_order": order })),
        Err(error) => failure(error),
    }
}

impl qobject::TicketBackend {
    pub fn set_home_sort_mode(mut self: Pin<&mut Self>, mode: &QString) -> QString {
        let mode = mode.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => set_home_sort_mode_response(service, &mode),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        if response_is_ok(&response) {
            self.as_mut().refresh();
        }
        QString::from(response.as_str())
    }

    pub fn set_home_ticket_order(mut self: Pin<&mut Self>, order_json: &QString) -> QString {
        let order_json = order_json.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => set_home_ticket_order_response(service, &order_json),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        if response_is_ok(&response) {
            self.as_mut().refresh();
        }
        QString::from(response.as_str())
    }
}
