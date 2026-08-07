use super::*;

impl qobject::TicketBackend {
    pub fn release_status(&self) -> QString {
        let response = match self.service.as_ref() {
            Some(service) => match service.release_status(APP_VERSION) {
                Ok(status) => success(release_value(&status)),
                Err(error) => failure(error),
            },
            None => failure(anyhow::anyhow!("application service is not initialized")),
        };
        QString::from(response.as_str())
    }

    pub fn check_release_async(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| {
                        match service.refresh_release_status(APP_VERSION, APP_REPOSITORY) {
                            Ok(status) => success(release_value(&status)),
                            Err(error) => failure(error),
                        }
                });
                queue_async_response(qt_thread, "check_release", response, false);
        });
    }
}

fn release_value(status: &ticket_core::ReleaseStatus) -> Value {
    json!({
            "organization": APP_ORGANIZATION,
            "organization_url": APP_ORGANIZATION_URL,
            "app_url": APP_APP_URL,
            "repository": APP_REPOSITORY,
            "current_version": status.current_version,
            "latest_version": status.latest_version,
            "update_available": status.update_available,
            "checked_at": status.checked_at,
            "release_url": status.release_url,
    })
}
