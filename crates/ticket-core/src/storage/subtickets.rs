use super::*;

impl Storage {
    pub fn replace_subtickets(
        &mut self,
        ticket_id: &str,
        subtickets: &[SubticketInput],
        recorded_at: &str,
    ) -> Result<()> {
        let transaction = self
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let existing_ids = {
            let mut statement =
            transaction.prepare("SELECT id FROM subtickets WHERE ticket_id = ?")?;
            let rows = statement.query_map([ticket_id], |row| row.get::<_, String>(0))?;
            rows.collect::<rusqlite::Result<Vec<_>>>()?
        };
        let mut active_ids = HashSet::new();

        for (index, input) in subtickets.iter().enumerate() {
            let id = if input.id.trim().is_empty() {
                Uuid::new_v4().to_string()
            } else {
                input.id.clone()
            };
            active_ids.insert(id.clone());
            let display_label = if input.display_label.trim().is_empty() {
                input.name.clone()
            } else {
                input.display_label.clone()
            };
            if existing_ids.iter().any(|existing_id| existing_id == &id) {
                transaction.execute(
                    r#"
                    UPDATE subtickets SET
                    name = ?, amount_minor = ?, currency = ?, display_label = ?,
                    keyboard_shortcut = ?, enabled = ?, sort_order = ?, updated_at = ?
                    WHERE ticket_id = ? AND id = ?
                    "#,
                    params![
                        input.name,
                        input.amount_minor,
                        input.currency,
                        display_label,
                        input.keyboard_shortcut,
                        input.enabled as i64,
                        index as i64,
                        recorded_at,
                        ticket_id,
                        id,
                    ],
                )?;
            } else {
                transaction.execute(
                    r#"
                    INSERT INTO subtickets (
                    id, ticket_id, name, amount_minor, currency, display_label,
                    keyboard_shortcut, enabled, sort_order, recorded_at, updated_at
                    ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                    "#,
                    params![
                        id,
                        ticket_id,
                        input.name,
                        input.amount_minor,
                        input.currency,
                        display_label,
                        input.keyboard_shortcut,
                        input.enabled as i64,
                        index as i64,
                        recorded_at,
                        recorded_at,
                    ],
                )?;
            }
        }

        for (offset, id) in existing_ids.iter().enumerate() {
            if active_ids.contains(id) {
                continue;
            }
            let has_history: bool = transaction.query_row(
                r#"
                SELECT EXISTS(SELECT 1 FROM daily_counters WHERE subticket_id = ?)
                OR EXISTS(SELECT 1 FROM print_records WHERE subticket_id = ?)
                OR EXISTS(SELECT 1 FROM counter_adjustments WHERE subticket_id = ?)
                "#,
                params![id, id, id],
                |row| row.get(0),
            )?;
            if has_history {
                transaction.execute(
                    "UPDATE subtickets SET enabled = 0, sort_order = ?, updated_at = ? \
                    WHERE ticket_id = ? AND id = ?",
                    params![
                        subtickets.len() as i64 + offset as i64,
                        recorded_at,
                        ticket_id,
                        id
                    ],
                )?;
            } else {
                transaction.execute(
                    "DELETE FROM subtickets WHERE ticket_id = ? AND id = ?",
                    params![ticket_id, id],
                )?;
            }
        }

        transaction.commit()?;
        Ok(())
    }

    pub fn list_subtickets(&self, ticket_id: &str) -> Result<Vec<Subticket>> {
        let mut statement = self.connection.prepare(
            "SELECT id, ticket_id, name, amount_minor, currency, display_label, \
            keyboard_shortcut, enabled, sort_order, recorded_at, updated_at \
            FROM subtickets WHERE ticket_id = ? ORDER BY sort_order, recorded_at",
        )?;
        let rows = statement.query_map([ticket_id], map_subticket)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }

    pub fn get_subticket(&self, subticket_id: &str) -> Result<Option<Subticket>> {
        self.connection
        .query_row(
            "SELECT id, ticket_id, name, amount_minor, currency, display_label, \
            keyboard_shortcut, enabled, sort_order, recorded_at, updated_at \
            FROM subtickets WHERE id = ?",
            [subticket_id],
            map_subticket,
        )
        .optional()
        .map_err(Into::into)
    }
}
