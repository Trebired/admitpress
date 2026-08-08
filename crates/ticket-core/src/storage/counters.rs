use super::*;

impl Storage {
    pub fn get_or_create_counter(
        &mut self,
        ticket: &Ticket,
        subticket_id: &str,
        local_date: &str,
        recorded_at: &str,
    ) -> Result<DailyCounter> {
        let transaction = self
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute(
            "INSERT OR IGNORE INTO daily_counters \
            (ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at) \
            VALUES (?, ?, ?, ?, ?, ?)",
            params![
                ticket.id,
                subticket_id,
                local_date,
                ticket.number_start,
                recorded_at,
                recorded_at
            ],
        )?;
        let counter = transaction.query_row(
            "SELECT ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at \
            FROM daily_counters WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?",
            params![ticket.id, subticket_id, local_date],
            map_counter,
        )?;
        transaction.commit()?;
        Ok(counter)
    }

    pub fn next_number_for_subticket_date(
        &self,
        ticket: &Ticket,
        subticket_id: &str,
        local_date: &str,
    ) -> Result<i64> {
        let next = self
        .connection
        .query_row(
            "SELECT next_number FROM daily_counters \
            WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?",
            params![ticket.id, subticket_id, local_date],
            |row| row.get(0),
        )
        .optional()?;
        Ok(next.unwrap_or(ticket.number_start))
    }

    pub fn adjust_counter(
        &mut self,
        ticket: &Ticket,
        subticket_id: &str,
        local_date: &str,
        new_next_number: i64,
        reason: &str,
        recorded_at: &str,
    ) -> Result<CounterAdjustment> {
        let transaction = self
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute(
            "INSERT OR IGNORE INTO daily_counters \
            (ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at) \
            VALUES (?, ?, ?, ?, ?, ?)",
            params![
                ticket.id,
                subticket_id,
                local_date,
                ticket.number_start,
                recorded_at,
                recorded_at
            ],
        )?;
        let previous_next_number: i64 = transaction.query_row(
            "SELECT next_number FROM daily_counters \
            WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?",
            params![ticket.id, subticket_id, local_date],
            |row| row.get(0),
        )?;
        transaction.execute(
            "UPDATE daily_counters SET next_number = ?, updated_at = ? \
            WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?",
            params![
                new_next_number,
                recorded_at,
                ticket.id,
                subticket_id,
                local_date
            ],
        )?;

        let adjustment = CounterAdjustment {
            id: Uuid::new_v4().to_string(),
            ticket_id: ticket.id.clone(),
            subticket_id: subticket_id.to_owned(),
            local_date: local_date.to_owned(),
            previous_next_number,
            new_next_number,
            reason: reason.to_owned(),
            recorded_at: recorded_at.to_owned(),
        };
        transaction.execute(
            "INSERT INTO counter_adjustments \
            (id, ticket_id, subticket_id, local_date, previous_next_number, new_next_number, \
            reason, recorded_at) \
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                adjustment.id,
                adjustment.ticket_id,
                adjustment.subticket_id,
                adjustment.local_date,
                adjustment.previous_next_number,
                adjustment.new_next_number,
                adjustment.reason,
                adjustment.recorded_at,
            ],
        )?;
        transaction.commit()?;
        Ok(adjustment)
    }

    pub fn sold_count_for_subticket_date(
        &self,
        subticket_id: &str,
        local_date: &str,
    ) -> Result<i64> {
        self.connection
        .query_row(
            "SELECT COUNT(*) FROM print_records \
            WHERE subticket_id = ? AND local_date = ? AND status IN ('completed', 'queued')",
            params![subticket_id, local_date],
            |row| row.get(0),
        )
        .map_err(Into::into)
    }

    pub fn day_has_sales(&self, ticket_id: &str, local_date: &str) -> Result<bool> {
        self.connection
        .query_row(
            "SELECT EXISTS(SELECT 1 FROM print_records \
            WHERE ticket_id = ? AND local_date = ? AND status IN ('completed', 'queued'))",
            params![ticket_id, local_date],
            |row| row.get::<_, bool>(0),
        )
        .map_err(Into::into)
    }

    pub fn list_sales_dates(&self, ticket_id: &str) -> Result<Vec<String>> {
        let mut statement = self.connection.prepare(
            "SELECT DISTINCT local_date FROM print_records \
            WHERE ticket_id = ? AND status IN ('completed', 'queued') \
            ORDER BY local_date DESC",
        )?;
        statement
        .query_map([ticket_id], |row| row.get(0))?
        .collect::<Result<Vec<String>, _>>()
        .map_err(Into::into)
    }

    pub fn list_day_sales(
        &self,
        ticket_id: &str,
        include_date: &str,
        limit: i64,
    ) -> Result<Vec<DaySalesSummary>> {
        let mut statement = self.connection.prepare(
            r#"
            SELECT
            records.local_date,
            subtickets.id,
            subtickets.name,
            subtickets.display_label,
            COALESCE(records.sold_amount_minor, subtickets.amount_minor) AS amount_minor,
            COALESCE(NULLIF(TRIM(records.sold_currency), ''), subtickets.currency) AS currency,
            COUNT(*) AS quantity,
            COALESCE(SUM(COALESCE(records.sold_amount_minor, subtickets.amount_minor)), 0)
            AS total_minor
            FROM print_records records
            JOIN subtickets ON subtickets.id = records.subticket_id
            WHERE records.ticket_id = ?
            AND records.status IN ('completed', 'queued')
            GROUP BY
            records.local_date,
            subtickets.id,
            subtickets.name,
            subtickets.display_label,
            COALESCE(records.sold_amount_minor, subtickets.amount_minor),
            COALESCE(NULLIF(TRIM(records.sold_currency), ''), subtickets.currency),
            subtickets.sort_order
            ORDER BY records.local_date DESC, subtickets.sort_order, subtickets.recorded_at
            "#,
        )?;

        let mut summaries: Vec<DaySalesSummary> = Vec::new();
        let mut rows = statement.query([ticket_id])?;
        while let Some(row) = rows.next()? {
            let local_date: String = row.get(0)?;
            if summaries.last().map(|summary| summary.local_date.as_str())
            != Some(local_date.as_str())
            {
                summaries.push(DaySalesSummary {
                        local_date: local_date.clone(),
                        ticket_count: 0,
                        total_minor: 0,
                        currency: String::new(),
                        subtickets: Vec::new(),
                });
            }

            let summary = summaries
            .last_mut()
            .expect("summary should be inserted before sales rows");
            let quantity: i64 = row.get(6)?;
            let total_minor: i64 = row.get(7)?;
            let currency: String = row.get(5)?;
            if summary.ticket_count == 0 {
                summary.currency = currency.clone();
            }
            summary.ticket_count += quantity;
            summary.total_minor += total_minor;
            summary.subtickets.push(DaySubticketSales {
                    subticket_id: row.get(1)?,
                    name: row.get(2)?,
                    display_label: row.get(3)?,
                    amount_minor: row.get(4)?,
                    currency,
                    quantity,
                    total_minor,
            });
        }

        retain_dates_on_or_before(&mut summaries, include_date);
        append_empty_calendar_days(&mut summaries, include_date, limit);
        summaries.sort_by(|left, right| right.local_date.cmp(&left.local_date));
        summaries.truncate(limit.max(0) as usize);
        Ok(summaries)
    }

    pub fn reset_day(&mut self, ticket_id: &str, local_date: &str) -> Result<()> {
        let transaction = self
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute(
            "DELETE FROM print_records WHERE ticket_id = ? AND local_date = ?",
            params![ticket_id, local_date],
        )?;
        transaction.execute(
            "DELETE FROM counter_adjustments WHERE ticket_id = ? AND local_date = ?",
            params![ticket_id, local_date],
        )?;
        transaction.execute(
            "DELETE FROM daily_counters WHERE ticket_id = ? AND local_date = ?",
            params![ticket_id, local_date],
        )?;
        transaction.commit()?;
        Ok(())
    }

    pub fn list_daily_counters(&self, ticket_id: &str) -> Result<Vec<DailyCounter>> {
        let mut statement = self.connection.prepare(
            "SELECT ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at \
            FROM daily_counters WHERE ticket_id = ? ORDER BY local_date, subticket_id",
        )?;
        let rows = statement.query_map([ticket_id], map_counter)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }
    pub fn list_counter_adjustments_for_date(
        &self,
        ticket_id: &str,
        local_date: &str,
    ) -> Result<Vec<CounterAdjustment>> {
        let mut statement = self.connection.prepare(
            "SELECT id, ticket_id, subticket_id, local_date, previous_next_number, \
            new_next_number, reason, recorded_at \
            FROM counter_adjustments WHERE ticket_id = ? AND local_date = ? \
            ORDER BY recorded_at DESC, id DESC",
        )?;
        let rows = statement.query_map(params![ticket_id, local_date], map_counter_adjustment)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }

    pub fn list_counter_adjustments(&self, ticket_id: &str) -> Result<Vec<CounterAdjustment>> {
        let mut statement = self.connection.prepare(
            "SELECT id, ticket_id, subticket_id, local_date, previous_next_number, \
            new_next_number, reason, recorded_at \
            FROM counter_adjustments WHERE ticket_id = ? ORDER BY recorded_at, id",
        )?;
        let rows = statement.query_map([ticket_id], map_counter_adjustment)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }
}

fn retain_dates_on_or_before(summaries: &mut Vec<DaySalesSummary>, include_date: &str) {
    let include_date = include_date.trim();
    if NaiveDate::parse_from_str(include_date, "%Y-%m-%d").is_err() {
        return;
    }
    summaries.retain(|summary| summary.local_date.as_str() <= include_date);
}

fn append_empty_calendar_days(
    summaries: &mut Vec<DaySalesSummary>,
    include_date: &str,
    limit: i64,
) {
    let Ok(anchor) = NaiveDate::parse_from_str(include_date.trim(), "%Y-%m-%d") else {
        return;
    };
    for offset in 0..limit.max(0) as u64 {
        let Some(date) = anchor.checked_sub_days(Days::new(offset)) else {
            continue;
        };
        let local_date = date.format("%Y-%m-%d").to_string();
        if summaries
        .iter()
        .any(|summary| summary.local_date == local_date)
        {
            continue;
        }
        summaries.push(DaySalesSummary {
                local_date,
                ticket_count: 0,
                total_minor: 0,
                currency: String::new(),
                subtickets: Vec::new(),
        });
    }
}
