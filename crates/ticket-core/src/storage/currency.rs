use super::*;
use chrono::{DateTime, Utc};

impl Storage {
    pub fn cached_exchange_rate(
        &self,
        from: &str,
        to: &str,
        max_age_seconds: i64,
    ) -> Result<Option<f64>> {
        let row: Option<(f64, String)> = self
        .connection
        .query_row(
            "SELECT rate, fetched_at FROM exchange_rate_cache \
            WHERE from_currency = ? AND to_currency = ?",
            params![from, to],
            |row| Ok((row.get(0)?, row.get(1)?)),
        )
        .optional()?;
        let Some((rate, fetched_at)) = row else {
            return Ok(None);
        };
        let is_fresh = DateTime::parse_from_rfc3339(&fetched_at)
        .map(|value| (Utc::now() - value.with_timezone(&Utc)).num_seconds() < max_age_seconds)
        .unwrap_or(false);
        Ok(is_fresh.then_some(rate))
    }

    pub fn store_exchange_rate(&self, from: &str, to: &str, rate: f64, fetched_at: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO exchange_rate_cache (from_currency, to_currency, rate, fetched_at) \
            VALUES (?, ?, ?, ?) \
            ON CONFLICT(from_currency, to_currency) DO UPDATE SET \
            rate = excluded.rate, fetched_at = excluded.fetched_at",
            params![from, to, rate, fetched_at],
        )?;
        Ok(())
    }
}
