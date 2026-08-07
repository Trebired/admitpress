use super::*;

impl qobject::TicketBackend {
    pub fn backup_status(&self) -> QString {
        let response = match self.service.as_ref() {
            Some(service) => backup_status_response(service),
            None => failure(anyhow::anyhow!("application service is not initialized")),
        };
        QString::from(response.as_str())
    }

    pub fn backup_now_async(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| match service.create_backup() {
                        Ok(info) => success(json!({
                                    "backup": info,
                                    "status": backup_status_value(service),
                        })),
                        Err(error) => failure(error),
                });
                queue_async_response(qt_thread, "backup_now", response, false);
        });
    }

    pub fn backup_if_due_async(self: Pin<&mut Self>) {
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(backup_if_due_response);
                queue_async_response(qt_thread, "backup_if_due", response, false);
        });
    }
}

fn backup_if_due_response(service: &mut TicketService) -> String {
    match service.backup_due() {
        Ok(false) => success(json!({ "skipped": true, "status": backup_status_value(service) })),
        Ok(true) => match service.create_backup() {
            Ok(info) => success(json!({
                        "skipped": false,
                        "backup": info,
                        "status": backup_status_value(service),
            })),
            Err(error) => failure(error),
        },
        Err(error) => failure(error),
    }
}

fn backup_status_response(service: &TicketService) -> String {
    success(backup_status_value(service))
}

fn backup_status_value(service: &TicketService) -> Value {
    let backups = service.list_backups().unwrap_or_default();
    json!({
            "last_backup_at": service.last_backup_at().ok().flatten(),
            "count": backups.len(),
            "total_bytes": backups.iter().map(|entry| entry.size_bytes).sum::<u64>(),
            "directory": service.backups_dir().to_string_lossy(),
    })
}
