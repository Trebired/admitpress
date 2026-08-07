mod backend;

rust_i18n::i18n!("i18n", fallback = "en");

use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QQuickStyle, QString, QUrl};
use cxx_qt_lib_extras::QApplication;
use std::env;
use std::path::PathBuf;

const APP_ID: &str = "app.ticket.native";
const APP_DISPLAY_NAME: &str = env!("APP_DISPLAY_NAME");
const APP_CONFIG_SLUG: &str = env!("APP_CONFIG_SLUG");
const DEFAULT_QT_LOGGING_RULES: &str = "qt.svg.warning=false";

fn main() {
    configure_qt_logging();
    configure_app_environment();

    if desktop_entry_available(APP_ID) {
        QGuiApplication::set_desktop_file_name(&QString::from(APP_ID));
    }

    let mut app = QApplication::new();
    if let Some(mut app) = app.as_mut() {
        app.as_mut()
        .set_application_name(&QString::from(APP_DISPLAY_NAME));
        app.as_mut()
        .set_application_display_name(&QString::from(APP_DISPLAY_NAME));
        app.as_mut()
        .set_organization_name(&QString::from(APP_DISPLAY_NAME));
        app.as_mut()
        .set_organization_domain(&QString::from("local.invalid"));
    }

    if env::var("QT_QUICK_CONTROLS_STYLE").is_err() {
        QQuickStyle::set_style(&QString::from("org.kde.desktop"));
    }

    let mut engine = QQmlApplicationEngine::new();
    if let Some(engine) = engine.as_mut() {
        engine.load(&QUrl::from(
                "qrc:/qt/qml/app/ticket/native/src/qml/app/main.qml",
        ));
    }

    if let Some(app) = app.as_mut() {
        app.exec();
    }
}

fn configure_app_environment() {
    if env::var_os("TICKET_APP_CONFIG_DIR_NAME").is_none() {
        unsafe {
            env::set_var("TICKET_APP_CONFIG_DIR_NAME", APP_CONFIG_SLUG);
        }
    }
}

fn configure_qt_logging() {
    if env::var_os("QT_LOGGING_RULES").is_none() {
        unsafe {
            env::set_var("QT_LOGGING_RULES", DEFAULT_QT_LOGGING_RULES);
        }
    }
}

fn desktop_entry_available(app_id: &str) -> bool {
    let desktop_file = format!("{app_id}.desktop");
    xdg_data_dirs()
    .into_iter()
    .any(|directory| directory.join("applications").join(&desktop_file).is_file())
}

fn xdg_data_dirs() -> Vec<PathBuf> {
    let mut directories = Vec::new();
    if let Some(data_home) = env::var_os("XDG_DATA_HOME") {
        directories.push(PathBuf::from(data_home));
    } else if let Some(home) = env::var_os("HOME") {
        directories.push(PathBuf::from(home).join(".local/share"));
    }

    if let Some(data_dirs) = env::var_os("XDG_DATA_DIRS") {
        directories.extend(env::split_paths(&data_dirs));
    } else {
        directories.push(PathBuf::from("/usr/local/share"));
        directories.push(PathBuf::from("/usr/share"));
    }
    directories
}
