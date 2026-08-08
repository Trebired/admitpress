use super::*;

#[allow(clippy::too_many_arguments)]
pub(super) fn save_ticket_request(
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
) -> SaveTicketRequest {
    SaveTicketRequest {
        ticket_id: ticket_id.to_string(),
        name: name.to_string(),
        width_cm,
        height_cm,
        printer_queue: printer_queue.to_string(),
        border_style: border_style.to_string(),
        border_width_mm,
        dpi,
        number_start,
        number_increment,
        number_padding,
        number_prefix: number_prefix.to_string(),
        number_suffix: number_suffix.to_string(),
        fields_json: fields_json.to_string(),
        subtickets_json: subtickets_json.to_string(),
    }
}

pub(super) fn print_one_request(
    ticket_id: &QString,
    subticket_id: &QString,
    local_date: &QString,
    sale_currency: &QString,
    sale_name: &QString,
) -> PrintOneRequest {
    let base = print_request_base(ticket_id, local_date);
    PrintOneRequest {
        ticket_id: base.ticket_id,
        subticket_id: subticket_id.to_string(),
        local_date: base.local_date,
        sale_currency: sale_currency.to_string(),
        sale_name: sale_name.to_string(),
    }
}

pub(super) fn normalized_locale(locale: &str) -> String {
    let normalized = locale.to_lowercase().replace('-', "_");
    let language = normalized.split('_').next().unwrap_or("en");
    match language {
        "cs" => String::from("cs"),
        _ => String::from("en"),
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn bulk_print_request(
    ticket_id: &QString,
    local_date: &QString,
    requests_json: &QString,
    label: &QString,
    count_only: bool,
    single_ticket: bool,
    sale_currency: &QString,
) -> BulkPrintRequest {
    let base = print_request_base(ticket_id, local_date);
    BulkPrintRequest {
        ticket_id: base.ticket_id,
        local_date: base.local_date,
        requests_json: requests_json.to_string(),
        label: label.to_string(),
        count_only,
        single_ticket,
        sale_currency: sale_currency.to_string(),
    }
}

fn print_request_base(ticket_id: &QString, local_date: &QString) -> PrintRequestBase {
    PrintRequestBase {
        ticket_id: ticket_id.to_string(),
        local_date: local_date.to_string(),
    }
}

pub(super) fn save_ticket_response(
    service: &mut TicketService,
    request: SaveTicketRequest,
) -> String {
    let fields: Result<Vec<TicketFieldInput>, _> = serde_json::from_str(&request.fields_json);
    let subtickets: Result<Vec<SubticketInput>, _> = serde_json::from_str(&request.subtickets_json);
    match (fields, subtickets) {
        (Ok(fields), Ok(subtickets)) => match service.save_ticket(
            &request.ticket_id,
            &request.name,
            request.width_cm,
            request.height_cm,
            Some(&request.printer_queue),
            &request.border_style,
            request.border_width_mm,
            request.dpi as i64,
            request.number_start as i64,
            request.number_increment as i64,
            request.number_padding as i64,
            &request.number_prefix,
            &request.number_suffix,
            &fields,
            &subtickets,
        ) {
            Ok(()) => success(json!({ "saved": true })),
            Err(error) => failure(error),
        },
        (Err(error), _) => failure(anyhow::anyhow!("invalid field data: {error}")),
        (_, Err(error)) => failure(anyhow::anyhow!("invalid subticket data: {error}")),
    }
}

pub(super) fn print_one_response(service: &mut TicketService, request: PrintOneRequest) -> String {
    let sale_currency = non_empty(&request.sale_currency);
    let sale_name = non_empty(&request.sale_name);
    match service.print_one(
        &request.ticket_id,
        &request.subticket_id,
        &request.local_date,
        sale_currency,
        sale_name,
    ) {
        Ok(print_result) => success(json!(print_result)),
        Err(error) => failure(error),
    }
}

fn non_empty(value: &str) -> Option<&str> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then_some(trimmed)
}

pub(super) fn preview_bulk_ticket_response(
    service: &mut TicketService,
    request: BulkPrintRequest,
) -> String {
    let requests: Result<Vec<BulkRequest>, _> = serde_json::from_str(&request.requests_json);
    match requests {
        Ok(requests) => match service.preview_bulk_ticket(
            &request.ticket_id,
            &request.local_date,
            &requests,
            &request.label,
        ) {
            Ok(path) => success(json!({ "path": path })),
            Err(error) => failure(error),
        },
        Err(error) => failure(anyhow::anyhow!("invalid bulk print data: {error}")),
    }
}

pub(super) fn bulk_print_response(
    service: &mut TicketService,
    request: BulkPrintRequest,
) -> String {
    let requests: Result<Vec<BulkRequest>, _> = serde_json::from_str(&request.requests_json);
    match requests {
        Ok(requests) => {
            let trimmed = request.label.trim();
            let options = BulkPrintOptions {
                count_only: request.count_only,
                single_ticket: request.single_ticket,
                label: (!trimmed.is_empty()).then_some(trimmed),
                sale_currency: non_empty(&request.sale_currency),
            };
            match service.bulk_print(&request.ticket_id, &request.local_date, &requests, options) {
                Ok(result) => success(json!(result)),
                Err(error) => failure(error),
            }
        }
        Err(error) => failure(anyhow::anyhow!("invalid bulk print data: {error}")),
    }
}

pub(super) fn export_days_response(
    service: &mut TicketService,
    ticket_id: &str,
    local_dates_json: &str,
    destination_path: &str,
) -> String {
    let local_dates: Result<Vec<String>, _> = serde_json::from_str(local_dates_json);
    match local_dates {
        Ok(local_dates) => {
            match service.export_day_sales(ticket_id, &local_dates, destination_path) {
                Ok(path) => success(json!({ "path": path })),
                Err(error) => failure(error),
            }
        }
        Err(error) => failure(anyhow::anyhow!("invalid day export data: {error}")),
    }
}

pub(super) fn update_subtickets_response(
    service: &mut TicketService,
    ticket_id: &str,
    local_date: &str,
    subtickets_json: &str,
) -> String {
    let subtickets: Result<Vec<SubticketInput>, _> = serde_json::from_str(subtickets_json);
    match subtickets {
        Ok(subtickets) => match service.update_subtickets(ticket_id, local_date, &subtickets) {
            Ok(summary) => success(json!(summary)),
            Err(error) => failure(error),
        },
        Err(error) => failure(anyhow::anyhow!("invalid subticket data: {error}")),
    }
}

pub(super) fn ensure_main_currency_response(service: &mut TicketService, locale: &str) -> String {
    match service.ensure_main_currency(locale) {
        Ok(currency) => success(json!({ "main_currency": currency })),
        Err(error) => failure(error),
    }
}

pub(super) fn set_main_currency_response(service: &mut TicketService, currency: &str) -> String {
    match service.set_main_currency(currency) {
        Ok(()) => success(json!({ "main_currency": currency.trim().to_uppercase() })),
        Err(error) => failure(error),
    }
}

pub(super) fn set_ui_language_response(service: &mut TicketService, language: &str) -> String {
    match service.set_ui_language(language) {
        Ok(()) => success(json!({ "ui_language": service.ui_language() })),
        Err(error) => failure(error),
    }
}

pub(super) fn cached_convert_amount_response(
    service: &TicketService,
    amount_minor: i64,
    currency: &str,
    target_currency: &str,
) -> String {
    match service.cached_convert_amount(amount_minor, currency, target_currency) {
        Ok(converted) => success(json!({ "amount_minor": converted })),
        Err(error) => failure(error),
    }
}

pub(super) fn refresh_exchange_rate_response(
    service: &mut TicketService,
    from_currency: &str,
    to_currency: &str,
) -> String {
    match service.refresh_exchange_rate(from_currency, to_currency) {
        Ok(rate) => success(json!({ "rate": rate })),
        Err(error) => failure(error),
    }
}

pub(super) fn revert_last_sale_response(
    service: &mut TicketService,
    ticket_id: &str,
    local_date: &str,
) -> String {
    match service.revert_last_sale(ticket_id, local_date) {
        Ok(reverted) => success(json!({ "reverted": reverted })),
        Err(error) => failure(error),
    }
}

pub(super) fn printer_activity_response(service: &mut TicketService) -> String {
    match service.printer_activity() {
        Ok(activity) => success(json!(activity)),
        Err(error) => failure(error),
    }
}

pub(super) fn printer_diagnostics_response(service: &mut TicketService) -> String {
    match service.printer_diagnostics() {
        Ok(diagnostics) => success(json!(diagnostics)),
        Err(error) => failure(error),
    }
}

pub(super) fn fix_printer_response(service: &mut TicketService, use_admin: bool) -> String {
    match service.fix_printer(use_admin) {
        Ok(result) => success(json!(result)),
        Err(error) => failure(error),
    }
}

pub(super) fn fix_printer_response_cancelable<F>(
    service: &mut TicketService,
    use_admin: bool,
    is_cancelled: F,
) -> String
where
F: Fn() -> bool,
{
    match service.fix_printer_with_cancel(use_admin, is_cancelled) {
        Ok(result) => success(json!(result)),
        Err(error) => failure(error),
    }
}

pub(super) fn with_default_service<F>(operation: F) -> String
where
F: FnOnce(&mut TicketService) -> String,
{
    match TicketService::open_default() {
        Ok(mut service) => operation(&mut service),
        Err(error) => failure(error),
    }
}

pub(super) fn queue_async_response(
    qt_thread: CxxQtThread<qobject::TicketBackend>,
    operation: &'static str,
    response_json: String,
    reload_service: bool,
) {
    queue_async_response_with_request(qt_thread, operation, response_json, reload_service, None);
}

pub(super) fn queue_async_response_with_request(
    qt_thread: CxxQtThread<qobject::TicketBackend>,
    operation: &'static str,
    response_json: String,
    reload_service: bool,
    request_id: Option<String>,
) {
    let id = ASYNC_RESPONSE_ID.fetch_add(1, Ordering::Relaxed);
    let mut payload = json!({
            "id": id,
            "operation": operation,
            "response_json": response_json,
    });
    if let Some(value) = request_id {
        payload["request_id"] = json!(value);
    }
    let payload = payload.to_string();

    qt_thread
    .queue(move |mut backend| {
            if reload_service {
                if let Ok(service) = TicketService::open_default() {
                    backend.as_mut().rust_mut().service = Some(service);
                    backend.as_mut().refresh();
                }
            }
            backend
            .as_mut()
            .set_async_response_json(QString::from(payload.as_str()));
    })
    .ok();
}

pub(super) fn success(data: Value) -> String {
    json!({ "ok": true, "data": data }).to_string()
}

pub(super) fn failure(error: anyhow::Error) -> String {
    json!({ "ok": false, "error": format!("{error:#}") }).to_string()
}

pub(super) fn response_is_ok(response: &str) -> bool {
    serde_json::from_str::<Value>(response)
    .ok()
    .and_then(|value| value.get("ok").and_then(Value::as_bool))
    .unwrap_or(false)
}
