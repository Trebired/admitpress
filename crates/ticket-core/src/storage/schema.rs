use super::*;

const REBUILD_DAILY_COUNTERS_PER_SUBTICKET_SQL: &str = r#"
ALTER TABLE daily_counters RENAME TO daily_counters_legacy;

CREATE TABLE daily_counters (
ticket_id TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
subticket_id TEXT NOT NULL REFERENCES subtickets(id) ON DELETE CASCADE,
local_date TEXT NOT NULL,
next_number INTEGER NOT NULL,
recorded_at TEXT NOT NULL,
updated_at TEXT NOT NULL,
PRIMARY KEY(ticket_id, subticket_id, local_date)
);

CREATE INDEX IF NOT EXISTS daily_counters_ticket_date
ON daily_counters(ticket_id, local_date);

INSERT OR REPLACE INTO daily_counters (
ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at
)
SELECT
records.ticket_id,
records.subticket_id,
records.local_date,
MAX(records.allocated_number) + tickets.number_increment,
MIN(records.recorded_at),
MAX(records.recorded_at)
FROM print_records records
JOIN tickets ON tickets.id = records.ticket_id
JOIN subtickets ON subtickets.id = records.subticket_id
GROUP BY records.ticket_id, records.subticket_id, records.local_date;

INSERT OR IGNORE INTO daily_counters (
ticket_id, subticket_id, local_date, next_number, recorded_at, updated_at
)
SELECT
legacy.ticket_id,
subtickets.id,
legacy.local_date,
tickets.number_start,
legacy.recorded_at,
legacy.updated_at
FROM daily_counters_legacy legacy
JOIN tickets ON tickets.id = legacy.ticket_id
JOIN subtickets ON subtickets.ticket_id = legacy.ticket_id;

DROP TABLE daily_counters_legacy;
"#;

const DEFAULT_NUMBER_PADDING_MIGRATION_KEY: &str = "migration.default_number_padding_zero";

impl Storage {
    pub fn open(path: &Path) -> Result<Self> {
        let connection = Connection::open(path)
        .with_context(|| format!("failed to open SQLite database at {}", path.display()))?;
        connection.execute_batch("PRAGMA foreign_keys = ON; PRAGMA journal_mode = WAL;")?;
        let storage = Self { connection };
        storage.migrate()?;
        Ok(storage)
    }

    fn migrate(&self) -> Result<()> {
        self.connection
        .execute_batch(include_str!("../../migrations/0001_initial.sql"))?;
        self.add_column_if_missing(
            "tickets",
            "border_style",
            "border_style TEXT NOT NULL DEFAULT 'none'",
        )?;
        self.add_column_if_missing(
            "tickets",
            "border_width_mm",
            "border_width_mm REAL NOT NULL DEFAULT 0.2",
        )?;
        self.add_column_if_missing(
            "ticket_fields",
            "date_format",
            "date_format TEXT NOT NULL DEFAULT 'date'",
        )?;
        self.add_column_if_missing(
            "ticket_fields",
            "template_kind",
            "template_kind TEXT NOT NULL DEFAULT 'classic'",
        )?;
        self.migrate_daily_counters_per_subticket()?;
        self.add_column_if_missing(
            "counter_adjustments",
            "subticket_id",
            "subticket_id TEXT NOT NULL DEFAULT ''",
        )?;
        self.add_column_if_missing("print_records", "operation_label", "operation_label TEXT")?;
        self.add_column_if_missing("print_records", "sold_currency", "sold_currency TEXT")?;
        self.add_column_if_missing(
            "print_records",
            "sold_amount_minor",
            "sold_amount_minor INTEGER",
        )?;
        self.create_exchange_rate_cache_table()?;
        self.migrate_default_number_padding()?;
        self.repair_centered_field_alignment()?;
        Ok(())
    }

    fn create_exchange_rate_cache_table(&self) -> Result<()> {
        self.connection.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS exchange_rate_cache (
            from_currency TEXT NOT NULL,
            to_currency TEXT NOT NULL,
            rate REAL NOT NULL,
            fetched_at TEXT NOT NULL,
            PRIMARY KEY (from_currency, to_currency)
            );
            "#,
        )?;
        Ok(())
    }

    fn add_column_if_missing(&self, table: &str, column: &str, definition: &str) -> Result<()> {
        if self.table_has_column(table, column)? {
            return Ok(());
        }

        self.connection
        .execute(&format!("ALTER TABLE {table} ADD COLUMN {definition}"), [])?;
        Ok(())
    }

    fn table_has_column(&self, table: &str, column: &str) -> Result<bool> {
        let mut statement = self
        .connection
        .prepare(&format!("PRAGMA table_info({table})"))?;
        let columns = statement.query_map([], |row| row.get::<_, String>(1))?;
        for existing_column in columns {
            if existing_column? == column {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn migrate_daily_counters_per_subticket(&self) -> Result<()> {
        if self.table_has_column("daily_counters", "subticket_id")? {
            self.create_daily_counter_date_index()?;
            return Ok(());
        }

        self.rebuild_daily_counters_per_subticket()
    }

    fn create_daily_counter_date_index(&self) -> Result<()> {
        self.connection.execute(
            "CREATE INDEX IF NOT EXISTS daily_counters_ticket_date \
            ON daily_counters(ticket_id, local_date)",
            [],
        )?;
        Ok(())
    }

    fn rebuild_daily_counters_per_subticket(&self) -> Result<()> {
        self.connection
        .execute_batch(REBUILD_DAILY_COUNTERS_PER_SUBTICKET_SQL)?;
        Ok(())
    }

    fn migrate_default_number_padding(&self) -> Result<()> {
        if self
        .get_setting(DEFAULT_NUMBER_PADDING_MIGRATION_KEY)?
        .as_deref()
        == Some("done")
        {
            return Ok(());
        }

        self.connection.execute(
            r#"
            UPDATE tickets
            SET number_padding = 0
            WHERE number_padding = 4
            AND number_prefix = ''
            AND number_suffix = ''
            "#,
            [],
        )?;
        self.set_setting(DEFAULT_NUMBER_PADDING_MIGRATION_KEY, "done")?;
        Ok(())
    }

    fn repair_centered_field_alignment(&self) -> Result<()> {
        self.connection.execute(
            r#"
            UPDATE ticket_fields
            SET horizontal_alignment = 'center'
            WHERE horizontal_alignment = 'left'
            AND x_mm > 0.001
            AND EXISTS (
            SELECT 1 FROM tickets
            WHERE tickets.id = ticket_fields.ticket_id
            AND ticket_fields.x_mm + ticket_fields.width_mm < tickets.width_mm - 0.001
            AND ABS((ticket_fields.x_mm + ticket_fields.width_mm / 2.0) - tickets.width_mm / 2.0) < 0.01
            )
            "#,
            [],
        )?;
        self.connection.execute(
            r#"
            UPDATE ticket_fields
            SET vertical_alignment = 'center'
            WHERE vertical_alignment = 'top'
            AND y_mm > 0.001
            AND EXISTS (
            SELECT 1 FROM tickets
            WHERE tickets.id = ticket_fields.ticket_id
            AND ticket_fields.y_mm + ticket_fields.height_mm < tickets.height_mm - 0.001
            AND ABS((ticket_fields.y_mm + ticket_fields.height_mm / 2.0) - tickets.height_mm / 2.0) < 0.01
            )
            "#,
            [],
        )?;
        Ok(())
    }
}
