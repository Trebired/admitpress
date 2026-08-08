use super::*;

const HOME_SORT_MODES: &[&str] = &["activity", "alphabetical", "created", "manual"];
const DEFAULT_HOME_SORT_MODE: &str = "activity";

fn normalize_home_sort_mode(mode: &str) -> Result<String> {
    let trimmed = mode.trim();
    if HOME_SORT_MODES.contains(&trimmed) {
        Ok(trimmed.to_owned())
    } else {
        Err(anyhow!("unsupported home sort mode: {trimmed}"))
    }
}

fn activity_timestamp(summary: &TicketSummary) -> &str {
    summary
    .recent_prints
    .first()
    .map(|record| record.recorded_at.as_str())
    .unwrap_or(summary.ticket.updated_at.as_str())
}

pub(super) fn sort_ticket_summaries(
    summaries: &mut [TicketSummary],
    mode: &str,
    manual_order: &[String],
) {
    match mode {
        "alphabetical" => {
            summaries.sort_by(|a, b| {
                    a.ticket
                    .name
                    .to_lowercase()
                    .cmp(&b.ticket.name.to_lowercase())
            });
        }
        "created" => {
            summaries.sort_by(|a, b| b.ticket.recorded_at.cmp(&a.ticket.recorded_at));
        }
        "manual" => {
            summaries.sort_by_key(|summary| {
                    manual_order
                    .iter()
                    .position(|entry| entry == &summary.ticket.id)
                    .unwrap_or(usize::MAX)
            });
        }
        _ => {
            summaries.sort_by(|a, b| activity_timestamp(b).cmp(activity_timestamp(a)));
        }
    }
}

impl TicketService {
    pub fn home_sort_mode(&self) -> Result<String> {
        Ok(self
            .storage
            .get_setting("home_sort_mode")?
            .and_then(|value| normalize_home_sort_mode(&value).ok())
            .unwrap_or_else(|| DEFAULT_HOME_SORT_MODE.to_owned()))
    }

    pub fn set_home_sort_mode(&mut self, mode: &str) -> Result<()> {
        let normalized = normalize_home_sort_mode(mode)?;
        self.storage.set_setting("home_sort_mode", &normalized)
    }

    pub fn home_ticket_order(&self) -> Result<Vec<String>> {
        match self.storage.get_setting("home_ticket_order")? {
            Some(value) => Ok(serde_json::from_str(&value).unwrap_or_default()),
            None => Ok(Vec::new()),
        }
    }

    pub fn set_home_ticket_order(&mut self, order: &[String]) -> Result<()> {
        let json = serde_json::to_string(order)?;
        self.storage.set_setting("home_ticket_order", &json)
    }
}
