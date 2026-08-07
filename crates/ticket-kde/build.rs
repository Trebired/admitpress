use cxx_qt_build::{CxxQtBuilder, QmlModule};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

const BACKEND_FILE: &str = "src/backend.rs";
const QML_MODULE_URI: &str = "app.ticket.native";
const QML_ROOT: &str = "src/qml";
const ASSETS_ROOT: &str = "src/qml/assets";
const WORKSPACE_MANIFEST: &str = "../../Cargo.toml";
const WORKSPACE_PACKAGE: &str = "../../package.json";

fn main() {
    let qml_files = discover_qml_files();
    let qmldir_files = discover_qmldir_files();
    let asset_files = discover_asset_files();
    let qmldir_qrc = write_resource_qrc(&qmldir_files, &asset_files);

    emit_application_metadata();
    emit_rebuild_inputs(&qml_files, &qmldir_files, &asset_files);

    unsafe {
        CxxQtBuilder::new_qml_module(QmlModule::new(QML_MODULE_URI).qml_files(qml_files))
        .qrc(qmldir_qrc)
        .files([BACKEND_FILE])
        .cc_builder(|compiler| {
                compiler.flag_if_supported("-Wno-sfinae-incomplete");
        })
        .build();
    }
}

fn emit_application_metadata() {
    let manifest =
    fs::read_to_string(WORKSPACE_MANIFEST).expect("failed to read workspace manifest");
    println!("cargo:rerun-if-changed={WORKSPACE_MANIFEST}");
    println!(
        "cargo:rustc-env=APP_DISPLAY_NAME={}",
        metadata_string(&manifest, "display_name")
    );
    println!(
        "cargo:rustc-env=APP_CONFIG_SLUG={}",
        metadata_string(&manifest, "config_slug")
    );
    emit_package_metadata();
}

fn emit_package_metadata() {
    let package = fs::read_to_string(WORKSPACE_PACKAGE).expect("failed to read workspace package");
    println!("cargo:rerun-if-changed={WORKSPACE_PACKAGE}");
    let root: serde_json::Value =
    serde_json::from_str(&package).expect("failed to parse the workspace package");
    let organization = &root["config"]["organization"];
    println!(
        "cargo:rustc-env=APP_ORGANIZATION={}",
        display_name(organization, "organization name")
    );
    println!(
        "cargo:rustc-env=APP_ORGANIZATION_URL={}",
        package_string(&organization["website"], "organization website")
    );
    println!(
        "cargo:rustc-env=APP_APP_URL={}",
        package_string(&root["config"]["app"]["website"], "app website")
    );
    println!(
        "cargo:rustc-env=APP_VERSION={}",
        package_string(&root["version"], "package version")
    );
    println!(
        "cargo:rustc-env=APP_REPOSITORY={}",
        package_string(&root["repository"]["url"], "repository url")
    );
}

fn display_name(section: &serde_json::Value, label: &str) -> String {
    if let Some(display) = section["displayName"].as_str() {
        return display.to_owned();
    }
    let name = package_string(&section["name"], label);
    let mut characters = name.chars();
    match characters.next() {
        Some(first) => first.to_uppercase().collect::<String>() + characters.as_str(),
        None => name,
    }
}

fn package_string(value: &serde_json::Value, label: &str) -> String {
    value
    .as_str()
    .unwrap_or_else(|| panic!("missing workspace package {label}"))
    .to_owned()
}

fn metadata_string(manifest: &str, key: &str) -> String {
    let prefix = format!("{key} = ");
    manifest
    .lines()
    .map(str::trim)
    .find_map(|line| line.strip_prefix(&prefix))
    .and_then(parse_quoted_value)
    .unwrap_or_else(|| panic!("missing workspace application metadata key {key}"))
}

fn parse_quoted_value(value: &str) -> Option<String> {
    let trimmed = value.trim();
    let inner = trimmed.strip_prefix('"')?.strip_suffix('"')?;
    Some(inner.to_owned())
}

fn discover_qml_files() -> Vec<String> {
    discover_files(QML_ROOT, |path| {
            path.extension().is_some_and(|extension| extension == "qml")
    })
}

fn discover_qmldir_files() -> Vec<String> {
    discover_files(QML_ROOT, |path| {
            path.file_name().is_some_and(|name| name == "qmldir")
    })
}

fn discover_asset_files() -> Vec<String> {
    discover_files(ASSETS_ROOT, |path| {
            path.extension().is_some_and(|extension| extension == "svg")
    })
}

fn discover_files(root: &str, accept: impl Fn(&Path) -> bool) -> Vec<String> {
    let mut files = Vec::new();
    collect_files(Path::new(root), &accept, &mut files);
    files.sort();
    files
}

fn collect_files(directory: &Path, accept: &impl Fn(&Path) -> bool, files: &mut Vec<String>) {
    println!("cargo:rerun-if-changed={}", directory.display());

    for entry in fs::read_dir(directory).expect("failed to read QML directory") {
        let path = entry.expect("failed to read QML directory entry").path();
        if path.is_dir() {
            collect_files(&path, accept, files);
        } else if accept(&path) {
            files.push(path_to_build_path(&path));
        }
    }
}

fn path_to_build_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}

fn emit_rebuild_inputs(qml_files: &[String], qmldir_files: &[String], asset_files: &[String]) {
    for qml_file in qml_files {
        println!("cargo:rerun-if-changed={qml_file}");
    }

    for qmldir_file in qmldir_files {
        println!("cargo:rerun-if-changed={qmldir_file}");
    }

    for asset_file in asset_files {
        println!("cargo:rerun-if-changed={asset_file}");
    }

    println!("cargo:rerun-if-changed={BACKEND_FILE}");
    println!("cargo:rerun-if-changed=src/window_icon.h");
}

fn write_resource_qrc(qmldir_files: &[String], asset_files: &[String]) -> PathBuf {
    let qrc_path = generated_qrc_path();
    let mut content = String::from("<RCC>\n");

    for qmldir_file in qmldir_files {
        content.push_str(&qresource_block(
                &module_resource_prefix(qmldir_file),
                "qmldir",
                qmldir_file,
        ));
    }

    if !asset_files.is_empty() {
        let prefix = format!("/qt/qml/{QML_MODULE_URI}/{ASSETS_ROOT}").replace('.', "/");
        for asset_file in asset_files {
            let alias = asset_file_name(asset_file);
            content.push_str(&qresource_block(&prefix, &alias, asset_file));
        }
    }

    content.push_str("</RCC>\n");
    write_if_changed(&qrc_path, &content);
    qrc_path
}

fn write_if_changed(path: &Path, content: &str) {
    if fs::read_to_string(path).is_ok_and(|existing| existing == content) {
        return;
    }
    fs::write(path, content).expect("failed to write generated QML resource file");
}

fn qresource_block(prefix: &str, alias: &str, source_file: &str) -> String {
    format!(
        "  <qresource prefix=\"{}\">\n    <file alias=\"{}\">{}</file>\n  </qresource>\n",
        xml_escape(prefix),
        xml_escape(alias),
        xml_escape(&absolute_build_path(source_file)),
    )
}

fn asset_file_name(asset_file: &str) -> String {
    Path::new(asset_file)
    .file_name()
    .expect("asset file must have a file name")
    .to_string_lossy()
    .into_owned()
}

fn generated_qrc_path() -> PathBuf {
    PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR must be set")).join("qml_imports.qrc")
}

fn absolute_build_path(path: &str) -> String {
    path_to_build_path(
        &Path::new(path)
        .canonicalize()
        .expect("failed to canonicalize QML resource path"),
    )
}

fn module_resource_prefix(qmldir_file: &str) -> String {
    let directory = PathBuf::from(qmldir_file)
    .parent()
    .expect("qmldir file must have a parent directory")
    .to_string_lossy()
    .replace('\\', "/");

    format!("/qt/qml/{QML_MODULE_URI}/{directory}").replace('.', "/")
}

fn xml_escape(value: &str) -> String {
    value
    .replace('&', "&amp;")
    .replace('"', "&quot;")
    .replace('<', "&lt;")
    .replace('>', "&gt;")
}
