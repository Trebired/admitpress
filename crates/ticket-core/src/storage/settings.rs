use super::*;

impl Storage {
    pub fn get_setting(&self, key: &str) -> Result<Option<String>> {
        self.connection
        .query_row(
            "SELECT value FROM app_settings WHERE key = ?",
            [key],
            |row| row.get(0),
        )
        .optional()
        .map_err(Into::into)
    }

    pub fn set_setting(&self, key: &str, value: &str) -> Result<()> {
        self.connection.execute(
            "INSERT INTO app_settings (key, value) VALUES (?, ?) \
            ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}
