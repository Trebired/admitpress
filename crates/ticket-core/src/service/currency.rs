use super::*;
use crate::renderer::currency_exponent;
use iso_country::Country;
use iso_currency::{Currency as IsoCurrency, IntoEnumIterator};
use serde_json::Value;
use std::str::FromStr;
use std::time::Duration;

const EXCHANGE_RATE_TTL_SECONDS: i64 = 6 * 60 * 60;
const EXCHANGE_RATE_TIMEOUT: Duration = Duration::from_secs(6);

impl TicketService {
    pub fn main_currency(&self) -> &str {
        &self.main_currency
    }

    pub fn set_main_currency(&mut self, currency: &str) -> Result<()> {
        let normalized = normalized_currency_code(currency)?;
        self.storage.set_setting("main_currency", &normalized)?;
        self.main_currency = normalized;
        Ok(())
    }

    pub fn ensure_main_currency(&mut self, locale: &str) -> Result<String> {
        if !self.main_currency.is_empty() {
            return Ok(self.main_currency.clone());
        }
        let default = default_currency_for_locale(locale)
        .unwrap_or_else(|| IsoCurrency::USD.code().to_owned());
        self.set_main_currency(&default)?;
        Ok(self.main_currency.clone())
    }

    fn resolved_currency(&self, currency: &str) -> String {
        let trimmed = currency.trim();
        if trimmed.is_empty() {
            self.main_currency.clone()
        } else {
            trimmed.to_uppercase()
        }
    }

    pub(super) fn printed_currency(&self, sold: Option<&str>, priced: &str) -> String {
        self.resolved_currency(
            sold.map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or(priced),
        )
    }

    pub(super) fn effective_sale_currency(&self, sale_currency: Option<&str>) -> Option<String> {
        let resolved = self.resolved_currency(
            sale_currency
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .unwrap_or_default(),
        );
        (!resolved.is_empty()).then_some(resolved)
    }

    pub fn available_currencies(&self) -> Vec<CurrencyOption> {
        IsoCurrency::iter()
        .map(|value| CurrencyOption {
                code: value.code().to_owned(),
                name: value.name().to_owned(),
                symbol: value.symbol().to_string(),
        })
        .collect()
    }

    pub fn cached_convert_amount(
        &self,
        amount_minor: i64,
        currency: &str,
        target_currency: &str,
    ) -> Result<Option<i64>> {
        let from = self.resolved_currency(currency);
        let to = self.resolved_currency(target_currency);
        if from.is_empty() || to.is_empty() || from == to {
            return Ok(Some(amount_minor));
        }
        let Some(rate) = self
        .storage
        .cached_exchange_rate(&from, &to, EXCHANGE_RATE_TTL_SECONDS)?
        else {
            return Ok(None);
        };
        Ok(Some(apply_rate(amount_minor, &from, &to, rate)))
    }

    pub fn convert_amount(
        &mut self,
        amount_minor: i64,
        currency: &str,
        target_currency: &str,
    ) -> Result<i64> {
        let from = self.resolved_currency(currency);
        let to = self.resolved_currency(target_currency);
        if from.is_empty() || to.is_empty() || from == to {
            return Ok(amount_minor);
        }
        let rate = self.exchange_rate(&from, &to)?;
        Ok(apply_rate(amount_minor, &from, &to, rate))
    }

    pub fn exchange_rate(&mut self, from: &str, to: &str) -> Result<f64> {
        if from.eq_ignore_ascii_case(to) {
            return Ok(1.0);
        }
        if let Some(rate) = self
        .storage
        .cached_exchange_rate(from, to, EXCHANGE_RATE_TTL_SECONDS)?
        {
            return Ok(rate);
        }
        self.refresh_exchange_rate(from, to)
    }

    pub fn refresh_exchange_rate(&mut self, from: &str, to: &str) -> Result<f64> {
        let rate = fetch_exchange_rate(from, to)?;
        self.storage.store_exchange_rate(from, to, rate, &now_iso())?;
        Ok(rate)
    }

    pub(super) fn sold_amounts_for(
        &mut self,
        subtickets: &[Subticket],
        sale_currency: Option<&str>,
    ) -> Result<Option<HashMap<String, i64>>> {
        let Some(sale_currency) = sale_currency
        .map(str::trim)
        .filter(|value| !value.is_empty())
        else {
            return Ok(None);
        };
        let mut amounts = HashMap::with_capacity(subtickets.len());
        for subticket in subtickets {
            let converted =
            self.convert_amount(subticket.amount_minor, &subticket.currency, sale_currency)?;
            amounts.insert(subticket.id.clone(), converted);
        }
        Ok(Some(amounts))
    }
}

fn apply_rate(amount_minor: i64, currency: &str, target_currency: &str, rate: f64) -> i64 {
    let source_major = amount_minor as f64 / 10f64.powi(currency_exponent(currency) as i32);
    let target_major = source_major * rate;
    (target_major * 10f64.powi(currency_exponent(target_currency) as i32)).round() as i64
}

fn normalized_currency_code(currency: &str) -> Result<String> {
    let normalized = currency.trim().to_uppercase();
    if IsoCurrency::from_code(&normalized).is_none() {
        return Err(anyhow!("unknown currency code: {currency}"));
    }
    Ok(normalized)
}

fn default_currency_for_locale(locale: &str) -> Option<String> {
    let region = locale.split(['_', '-']).nth(1)?;
    let country = Country::from_str(region).ok()?;
    IsoCurrency::from_country(country)
    .into_iter()
    .next()
    .map(|value| value.code().to_owned())
}

fn fetch_exchange_rate(from: &str, to: &str) -> Result<f64> {
    let url = format!("https://open.er-api.com/v6/latest/{from}");
    let agent = ureq::Agent::config_builder()
    .timeout_global(Some(EXCHANGE_RATE_TIMEOUT))
    .build()
    .new_agent();
    let mut response = agent
    .get(&url)
    .call()
    .context("failed to reach the exchange rate service")?;
    let body: Value = response
    .body_mut()
    .read_json()
    .context("the exchange rate service returned an unexpected response")?;
    if body.get("result").and_then(Value::as_str) != Some("success") {
        return Err(anyhow!("the exchange rate service reported an error"));
    }
    body["rates"][to]
    .as_f64()
    .ok_or_else(|| anyhow!("no exchange rate is available for {to}"))
}
