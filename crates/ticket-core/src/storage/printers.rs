use super::*;

impl Storage {
    pub fn list_printer_profiles(&self) -> Result<Vec<PrinterProfile>> {
        let mut statement = self.connection.prepare(
            "SELECT queue, width_mm, height_mm, offset_x_mm, offset_y_mm, \
            scale_x_percent, scale_y_percent, recorded_at, updated_at \
            FROM printer_profiles ORDER BY queue",
        )?;
        let rows = statement.query_map([], map_printer_profile)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }

    pub fn get_printer_profile(&self, queue: &str) -> Result<Option<PrinterProfile>> {
        let mut statement = self.connection.prepare(
            "SELECT queue, width_mm, height_mm, offset_x_mm, offset_y_mm, \
            scale_x_percent, scale_y_percent, recorded_at, updated_at \
            FROM printer_profiles WHERE queue = ?",
        )?;
        let mut rows = statement.query_map([queue], map_printer_profile)?;
        rows.next().transpose().map_err(Into::into)
    }

    pub fn upsert_printer_page_size(
        &self,
        queue: &str,
        width_mm: f64,
        height_mm: f64,
        recorded_at: &str,
    ) -> Result<()> {
        self.connection.execute(
            r#"
            INSERT INTO printer_profiles (
            queue, width_mm, height_mm, offset_x_mm, offset_y_mm,
            scale_x_percent, scale_y_percent, recorded_at, updated_at
            ) VALUES (?, ?, ?, 0, 0, 100, 100, ?, ?)
            ON CONFLICT(queue) DO UPDATE SET
            width_mm = excluded.width_mm,
            height_mm = excluded.height_mm,
            updated_at = excluded.updated_at
            "#,
            params![queue, width_mm, height_mm, recorded_at, recorded_at],
        )?;
        Ok(())
    }

    pub fn upsert_printer_calibration(
        &self,
        queue: &str,
        calibration: PrintCalibration,
        fallback_width_mm: f64,
        fallback_height_mm: f64,
        recorded_at: &str,
    ) -> Result<()> {
        let calibration = calibration.normalized();
        self.connection.execute(
            r#"
            INSERT INTO printer_profiles (
            queue, width_mm, height_mm, offset_x_mm, offset_y_mm,
            scale_x_percent, scale_y_percent, recorded_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(queue) DO UPDATE SET
            offset_x_mm = excluded.offset_x_mm,
            offset_y_mm = excluded.offset_y_mm,
            scale_x_percent = excluded.scale_x_percent,
            scale_y_percent = excluded.scale_y_percent,
            updated_at = excluded.updated_at
            "#,
            params![
                queue,
                fallback_width_mm,
                fallback_height_mm,
                calibration.offset_x_mm,
                calibration.offset_y_mm,
                calibration.scale_x_percent,
                calibration.scale_y_percent,
                recorded_at,
                recorded_at,
            ],
        )?;
        Ok(())
    }
}
