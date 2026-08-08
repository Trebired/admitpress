use super::*;

fn request_number(request: &Value, field: &'static str) -> Option<f64> {
    request.get(field).and_then(Value::as_f64)
}

fn print_calibration_sheet_response(
    service: &mut TicketService,
    queue: String,
    request_json: String,
) -> String {
    let request: Value = match serde_json::from_str(&request_json) {
        Ok(value) => value,
        Err(error) => {
            return failure(anyhow::anyhow!("invalid calibration sheet request: {error}"));
        }
    };
    let (Some(width_cm), Some(height_cm)) = (
        request_number(&request, "width_cm"),
        request_number(&request, "height_cm"),
    ) else {
        return failure(anyhow::anyhow!("calibration sheet request is missing page size"));
    };
    let page = PageSize {
        width_mm: width_cm * 10.0,
        height_mm: height_cm * 10.0,
    };
    let calibration = PrintCalibration {
        offset_x_mm: request_number(&request, "offset_x_mm").unwrap_or(0.0),
        offset_y_mm: request_number(&request, "offset_y_mm").unwrap_or(0.0),
        scale_x_percent: request_number(&request, "scale_x_percent").unwrap_or(100.0),
        scale_y_percent: request_number(&request, "scale_y_percent").unwrap_or(100.0),
    };
    match service.print_calibration_sheet(&queue, page, calibration) {
        Ok(output_path) => success(json!({ "output_path": output_path })),
        Err(error) => failure(error),
    }
}

impl qobject::TicketBackend {
    pub fn printer_profiles(mut self: Pin<&mut Self>) -> QString {
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => match service.printer_profiles() {
                    Ok(profiles) => success(json!({ "profiles": profiles })),
                    Err(error) => failure(error),
                },
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn set_printer_page_size(
        mut self: Pin<&mut Self>,
        printer_queue: &QString,
        width_cm: f64,
        height_cm: f64,
    ) -> QString {
        let queue = printer_queue.to_string();
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| service.set_printer_page_size(&queue, width_cm, height_cm))
        };
        let response = match result {
            Ok(profile) => {
                self.as_mut().refresh();
                success(json!({ "profile": profile }))
            }
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn set_printer_calibration(
        mut self: Pin<&mut Self>,
        printer_queue: &QString,
        offset_x_mm: f64,
        offset_y_mm: f64,
        scale_x_percent: f64,
        scale_y_percent: f64,
    ) -> QString {
        let queue = printer_queue.to_string();
        let calibration = PrintCalibration {
            offset_x_mm,
            offset_y_mm,
            scale_x_percent,
            scale_y_percent,
        };
        let result = {
            let mut rust = self.as_mut().rust_mut();
            rust.service
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("application service is not initialized"))
            .and_then(|service| service.set_printer_calibration(&queue, calibration))
        };
        let response = match result {
            Ok(profile) => success(json!({ "profile": profile })),
            Err(error) => failure(error),
        };
        QString::from(response.as_str())
    }

    pub fn print_calibration_sheet(
        mut self: Pin<&mut Self>,
        printer_queue: &QString,
        calibration_json: &QString,
    ) -> QString {
        let queue = printer_queue.to_string();
        let request_json = calibration_json.to_string();
        let response = {
            let mut rust = self.as_mut().rust_mut();
            match rust.service.as_mut() {
                Some(service) => print_calibration_sheet_response(service, queue, request_json),
                None => failure(anyhow::anyhow!("application service is not initialized")),
            }
        };
        QString::from(response.as_str())
    }

    pub fn print_calibration_sheet_async(
        self: Pin<&mut Self>,
        printer_queue: &QString,
        calibration_json: &QString,
    ) {
        let queue = printer_queue.to_string();
        let request_json = calibration_json.to_string();
        let qt_thread = self.qt_thread();
        std::thread::spawn(move || {
                let response = with_default_service(|service| {
                        print_calibration_sheet_response(service, queue, request_json)
                });
                queue_async_response(qt_thread, "print_calibration_sheet", response, false);
        });
    }
}
