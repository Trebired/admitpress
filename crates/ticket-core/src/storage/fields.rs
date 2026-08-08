use super::*;

impl Storage {
    pub fn replace_fields(
        &mut self,
        ticket_id: &str,
        fields: &[TicketFieldInput],
        recorded_at: &str,
    ) -> Result<()> {
        let transaction = self
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
        transaction.execute("DELETE FROM ticket_fields WHERE ticket_id = ?", [ticket_id])?;

        for (index, input) in fields.iter().enumerate() {
            let id = if input.id.trim().is_empty() {
                Uuid::new_v4().to_string()
            } else {
                input.id.clone()
            };
            transaction.execute(
                r#"
                INSERT INTO ticket_fields (
                id, ticket_id, template_kind, field_kind, dynamic_kind, text, x_mm, y_mm,
                width_mm, height_mm, font_family, font_size_pt, font_weight, font_italic,
                horizontal_alignment, vertical_alignment, date_format, prefix, suffix,
                sort_order, recorded_at, updated_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    id,
                    ticket_id,
                    normalize_template_kind(&input.template_kind),
                    field_kind_to_db(&input.field_kind),
                    input.dynamic_kind.as_ref().map(dynamic_kind_to_db),
                    input.text,
                    input.x_mm,
                    input.y_mm,
                    input.width_mm,
                    input.height_mm,
                    input.font_family,
                    input.font_size_pt,
                    input.font_weight,
                    input.font_italic,
                    input.horizontal_alignment,
                    input.vertical_alignment,
                    normalize_date_format(&input.date_format),
                    input.prefix,
                    input.suffix,
                    index as i64,
                    recorded_at,
                    recorded_at,
                ],
            )?;
        }

        transaction.commit()?;
        Ok(())
    }

    pub fn list_fields(&self, ticket_id: &str) -> Result<Vec<TicketField>> {
        let mut statement = self.connection.prepare(
            "SELECT id, ticket_id, template_kind, field_kind, dynamic_kind, text, x_mm, y_mm, width_mm, \
            height_mm, font_family, font_size_pt, font_weight, font_italic, horizontal_alignment, \
            vertical_alignment, date_format, prefix, suffix, sort_order, recorded_at, updated_at \
            FROM ticket_fields WHERE ticket_id = ? ORDER BY template_kind, sort_order, recorded_at",
        )?;
        let rows = statement.query_map([ticket_id], map_field)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }
}
