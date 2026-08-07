use super::*;

pub(in crate::service) fn discover_printer_queues() -> Vec<String> {
    let output = run_printer_command("lpstat", &["-a"]);
    if !output.success {
        return Vec::new();
    }
    output
    .stdout
    .lines()
    .filter_map(|line| line.split_whitespace().next())
    .map(ToOwned::to_owned)
    .collect()
}

pub(in crate::service) fn default_printer_queue() -> Option<String> {
    let output = run_printer_command("lpstat", &["-d"]);
    if !output.success {
        return None;
    }
    output.stdout.lines().find_map(|line| {
            line.trim()
            .strip_prefix("system default destination:")
            .and_then(normalized_optional)
    })
}
