use core::pin::Pin;
use cxx_qt::{CxxQtThread, CxxQtType, Threading};
use cxx_qt_lib::QString;
use serde_json::{Value, json};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use ticket_core::{BulkPrintOptions, BulkRequest, SubticketInput, TicketFieldInput, TicketService};

static ASYNC_RESPONSE_ID: AtomicU64 = AtomicU64::new(1);
static PRINTER_FIX_GENERATION: AtomicU64 = AtomicU64::new(1);
static PRINTER_FIX_CANCELLED: AtomicBool = AtomicBool::new(false);
const APP_DISPLAY_NAME: &str = env!("APP_DISPLAY_NAME");
const APP_ORGANIZATION: &str = env!("APP_ORGANIZATION");
const APP_VERSION: &str = env!("APP_VERSION");
const APP_REPOSITORY: &str = env!("APP_REPOSITORY");
const APP_ORGANIZATION_URL: &str = env!("APP_ORGANIZATION_URL");
const APP_APP_URL: &str = env!("APP_APP_URL");
const WINDOW_ICON_RESOURCE_ROOT: &str = ":/qt/qml/app/ticket/native/src/qml/assets";

#[cxx_qt::bridge]
pub mod qobject {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;

        include!("window_icon.h");
        #[rust_name = "apply_window_icon"]
        fn applyWindowIcon(resource_path: &QString);
    }

    #[auto_cxx_name]
    unsafe extern "RustQt" {
        #[qobject]
        #[qml_element]
        #[qproperty(QString, state_json)]
        #[qproperty(QString, status_message)]
        #[qproperty(QString, async_response_json)]
        type TicketBackend = super::TicketBackendRust;

        #[qinvokable]
        fn initialize(self: Pin<&mut Self>);

        #[qinvokable]
        fn refresh(self: Pin<&mut Self>);

        #[qinvokable]
        fn today(self: &Self) -> QString;

        #[qinvokable]
        fn translate(self: &Self, key: &QString, locale: &QString) -> QString;

        #[qinvokable]
        fn create_ticket(
            self: Pin<&mut Self>,
            name: &QString,
            width_cm: f64,
            height_cm: f64,
        ) -> QString;

        #[qinvokable]
        fn load_ticket(self: Pin<&mut Self>, ticket_id: &QString, local_date: &QString) -> QString;

        #[qinvokable]
        fn save_ticket(
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
        ) -> QString;

        #[qinvokable]
        fn save_ticket_async(
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
        );

        #[qinvokable]
        fn import_background(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            source_path: &QString,
        ) -> QString;

        #[qinvokable]
        fn import_image(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            source_path: &QString,
        ) -> QString;

        #[qinvokable]
        fn delete_ticket(self: Pin<&mut Self>, ticket_id: &QString) -> QString;

        #[qinvokable]
        fn export_ticket_archive(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            destination_path: &QString,
        ) -> QString;

        #[qinvokable]
        fn import_ticket_archive(self: Pin<&mut Self>, source_path: &QString) -> QString;

        #[qinvokable]
        fn export_days(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            local_dates_json: &QString,
            destination_path: &QString,
        ) -> QString;

        #[qinvokable]
        fn preview_ticket(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            subticket_id: &QString,
            local_date: &QString,
            allocated_number: i32,
        ) -> QString;

        #[qinvokable]
        fn preview_bulk_ticket(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            local_date: &QString,
            requests_json: &QString,
            label: &QString,
        ) -> QString;

        #[qinvokable]
        fn print_one(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            subticket_id: &QString,
            local_date: &QString,
            sale_currency: &QString,
            sale_name: &QString,
        ) -> QString;

        #[qinvokable]
        fn print_one_async(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            subticket_id: &QString,
            local_date: &QString,
            sale_currency: &QString,
            sale_name: &QString,
        );

        #[qinvokable]
        fn bulk_print(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            local_date: &QString,
            requests_json: &QString,
            label: &QString,
            count_only: bool,
            single_ticket: bool,
            sale_currency: &QString,
        ) -> QString;

        #[qinvokable]
        fn bulk_print_async(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            local_date: &QString,
            requests_json: &QString,
            label: &QString,
            count_only: bool,
            single_ticket: bool,
            sale_currency: &QString,
        );

        #[qinvokable]
        fn update_subtickets(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            local_date: &QString,
            subtickets_json: &QString,
        ) -> QString;

        #[qinvokable]
        fn revert_last_sale(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            local_date: &QString,
        ) -> QString;

        #[qinvokable]
        fn adjust_counter(
            self: Pin<&mut Self>,
            ticket_id: &QString,
            subticket_id: &QString,
            local_date: &QString,
            new_next_number: i32,
            reason: &QString,
        ) -> QString;

        #[qinvokable]
        fn delete_day(self: Pin<&mut Self>, ticket_id: &QString, local_date: &QString) -> QString;

        #[qinvokable]
        fn set_development_mode(self: Pin<&mut Self>, enabled: bool) -> QString;

        #[qinvokable]
        fn set_printer_queue(self: Pin<&mut Self>, printer_queue: &QString) -> QString;

        #[qinvokable]
        fn printer_activity(self: Pin<&mut Self>) -> QString;

        #[qinvokable]
        fn printer_diagnostics(self: Pin<&mut Self>) -> QString;

        #[qinvokable]
        fn printer_diagnostics_async(self: Pin<&mut Self>, request_id: &QString);

        #[qinvokable]
        fn fix_printer(self: Pin<&mut Self>, use_admin: bool) -> QString;

        #[qinvokable]
        fn fix_printer_async(self: Pin<&mut Self>, use_admin: bool);

        #[qinvokable]
        fn cancel_printer_fix(self: Pin<&mut Self>) -> QString;

        #[qinvokable]
        fn set_data_directory(self: Pin<&mut Self>, source_path: &QString) -> QString;

        #[qinvokable]
        fn ensure_main_currency(self: Pin<&mut Self>, locale: &QString) -> QString;

        #[qinvokable]
        fn set_main_currency(self: Pin<&mut Self>, currency: &QString) -> QString;

        #[qinvokable]
        fn format_money(self: &Self, amount_minor: f64, currency: &QString) -> QString;

        #[qinvokable]
        fn cached_convert_amount(
            self: Pin<&mut Self>,
            amount_minor: f64,
            currency: &QString,
            target_currency: &QString,
        ) -> QString;

        #[qinvokable]
        fn refresh_exchange_rate_async(
            self: Pin<&mut Self>,
            from_currency: &QString,
            to_currency: &QString,
        );

        #[qinvokable]
        fn prefers_gnome_icons(self: &Self) -> bool;

        #[qinvokable]
        fn set_window_icon(self: &Self, dark: bool);

        #[qinvokable]
        fn release_status(self: &Self) -> QString;

        #[qinvokable]
        fn check_release_async(self: Pin<&mut Self>);

        #[qinvokable]
        fn backup_status(self: &Self) -> QString;

        #[qinvokable]
        fn backup_now_async(self: Pin<&mut Self>);

        #[qinvokable]
        fn backup_if_due_async(self: Pin<&mut Self>);
    }

    impl cxx_qt::Threading for TicketBackend {}
}

mod backup;
mod currency;
mod helpers;
mod lifecycle;
mod printing;
mod release;
mod ticket;

use helpers::*;

pub struct TicketBackendRust {
    state_json: QString,
    status_message: QString,
    async_response_json: QString,
    service: Option<TicketService>,
}

impl Default for TicketBackendRust {
    fn default() -> Self {
        Self {
            state_json: QString::from("{}"),
            status_message: QString::from(format!("Starting {APP_DISPLAY_NAME}").as_str()),
            async_response_json: QString::from(""),
            service: None,
        }
    }
}

struct SaveTicketRequest {
    ticket_id: String,
    name: String,
    width_cm: f64,
    height_cm: f64,
    printer_queue: String,
    border_style: String,
    border_width_mm: f64,
    dpi: i32,
    number_start: i32,
    number_increment: i32,
    number_padding: i32,
    number_prefix: String,
    number_suffix: String,
    fields_json: String,
    subtickets_json: String,
}

struct PrintOneRequest {
    ticket_id: String,
    subticket_id: String,
    local_date: String,
    sale_currency: String,
    sale_name: String,
}

struct BulkPrintRequest {
    ticket_id: String,
    local_date: String,
    requests_json: String,
    label: String,
    count_only: bool,
    single_ticket: bool,
    sale_currency: String,
}

struct PrintRequestBase {
    ticket_id: String,
    local_date: String,
}
