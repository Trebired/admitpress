use super::*;

impl Storage {
    pub fn create_ticket(
        &self,
        name: &str,
        width_mm: f64,
        height_mm: f64,
        recorded_at: &str,
    ) -> Result<Ticket> {
        let ticket = Ticket {
            id: Uuid::new_v4().to_string(),
            name: name.to_owned(),
            width_mm,
            height_mm,
            background_svg_path: None,
            printer_queue: None,
            border_style: "none".to_owned(),
            border_width_mm: 0.2,
            dpi: 203,
            number_start: 1,
            number_increment: 1,
            number_padding: 0,
            number_prefix: String::new(),
            number_suffix: String::new(),
            recorded_at: recorded_at.to_owned(),
            updated_at: recorded_at.to_owned(),
        };

        self.connection.execute(
            r#"
            INSERT INTO tickets (
            id, name, width_mm, height_mm, background_svg_path, printer_queue,
            border_style, border_width_mm, dpi, number_start, number_increment,
            number_padding, number_prefix, number_suffix, recorded_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            params![
                ticket.id,
                ticket.name,
                ticket.width_mm,
                ticket.height_mm,
                ticket.background_svg_path,
                ticket.printer_queue,
                ticket.border_style,
                ticket.border_width_mm,
                ticket.dpi,
                ticket.number_start,
                ticket.number_increment,
                ticket.number_padding,
                ticket.number_prefix,
                ticket.number_suffix,
                ticket.recorded_at,
                ticket.updated_at,
            ],
        )?;

        Ok(ticket)
    }

    pub fn list_tickets(&self) -> Result<Vec<Ticket>> {
        let mut statement = self.connection.prepare(
            "SELECT id, name, width_mm, height_mm, background_svg_path, printer_queue, \
            border_style, border_width_mm, dpi, number_start, number_increment, \
            number_padding, number_prefix, number_suffix, recorded_at, updated_at \
            FROM tickets ORDER BY updated_at DESC, name ASC",
        )?;
        let rows = statement.query_map([], map_ticket)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }

    pub fn get_ticket(&self, ticket_id: &str) -> Result<Option<Ticket>> {
        self.connection
        .query_row(
            "SELECT id, name, width_mm, height_mm, background_svg_path, printer_queue, \
            border_style, border_width_mm, dpi, number_start, number_increment, \
            number_padding, number_prefix, number_suffix, recorded_at, updated_at \
            FROM tickets WHERE id = ?",
            [ticket_id],
            map_ticket,
        )
        .optional()
        .map_err(Into::into)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_ticket(
        &self,
        ticket_id: &str,
        name: &str,
        width_mm: f64,
        height_mm: f64,
        background_svg_path: Option<&str>,
        printer_queue: Option<&str>,
        border_style: &str,
        border_width_mm: f64,
        dpi: i64,
        number_start: i64,
        number_increment: i64,
        number_padding: i64,
        number_prefix: &str,
        number_suffix: &str,
        updated_at: &str,
    ) -> Result<()> {
        self.connection.execute(
            r#"
            UPDATE tickets SET
            name = ?, width_mm = ?, height_mm = ?, background_svg_path = ?,
            printer_queue = ?, border_style = ?, border_width_mm = ?, dpi = ?,
            number_start = ?, number_increment = ?, number_padding = ?,
            number_prefix = ?, number_suffix = ?, updated_at = ?
            WHERE id = ?
            "#,
            params![
                name,
                width_mm,
                height_mm,
                background_svg_path,
                printer_queue,
                border_style,
                border_width_mm,
                dpi,
                number_start,
                number_increment,
                number_padding,
                number_prefix,
                number_suffix,
                updated_at,
                ticket_id,
            ],
        )?;
        Ok(())
    }

    pub fn delete_ticket(&self, ticket_id: &str) -> Result<()> {
        self.connection
        .execute("DELETE FROM tickets WHERE id = ?", [ticket_id])?;
        Ok(())
    }
}
