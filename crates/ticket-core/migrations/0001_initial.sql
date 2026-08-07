CREATE TABLE IF NOT EXISTS tickets (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                width_mm REAL NOT NULL CHECK(width_mm > 0),
                height_mm REAL NOT NULL CHECK(height_mm > 0),
                background_svg_path TEXT,
                printer_queue TEXT,
                border_style TEXT NOT NULL DEFAULT 'none',
                border_width_mm REAL NOT NULL DEFAULT 0.2,
                dpi INTEGER NOT NULL DEFAULT 203,
                number_start INTEGER NOT NULL DEFAULT 1,
                number_increment INTEGER NOT NULL DEFAULT 1,
                number_padding INTEGER NOT NULL DEFAULT 0,
                number_prefix TEXT NOT NULL DEFAULT '',
                number_suffix TEXT NOT NULL DEFAULT '',
                recorded_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS ticket_fields (
                id TEXT PRIMARY KEY,
                ticket_id TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
                template_kind TEXT NOT NULL DEFAULT 'classic',
                field_kind TEXT NOT NULL,
                dynamic_kind TEXT,
                text TEXT NOT NULL DEFAULT '',
                x_mm REAL NOT NULL,
                y_mm REAL NOT NULL,
                width_mm REAL NOT NULL,
                height_mm REAL NOT NULL,
                font_family TEXT NOT NULL,
                font_size_pt REAL NOT NULL,
                font_weight INTEGER NOT NULL,
                horizontal_alignment TEXT NOT NULL,
                vertical_alignment TEXT NOT NULL,
                date_format TEXT NOT NULL DEFAULT 'date',
                prefix TEXT NOT NULL DEFAULT '',
                suffix TEXT NOT NULL DEFAULT '',
                sort_order INTEGER NOT NULL DEFAULT 0,
                recorded_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS ticket_fields_ticket_id
                ON ticket_fields(ticket_id, template_kind, sort_order);

            CREATE TABLE IF NOT EXISTS subtickets (
                id TEXT PRIMARY KEY,
                ticket_id TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
                name TEXT NOT NULL,
                amount_minor INTEGER NOT NULL,
                currency TEXT NOT NULL,
                display_label TEXT NOT NULL DEFAULT '',
                keyboard_shortcut TEXT,
                enabled INTEGER NOT NULL DEFAULT 1,
                sort_order INTEGER NOT NULL DEFAULT 0,
                recorded_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS subtickets_ticket_id
                ON subtickets(ticket_id, sort_order);

            CREATE TABLE IF NOT EXISTS daily_counters (
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

            CREATE TABLE IF NOT EXISTS print_records (
                id TEXT PRIMARY KEY,
                ticket_id TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
                subticket_id TEXT NOT NULL,
                local_date TEXT NOT NULL,
                allocated_number INTEGER NOT NULL,
                rendered_output_path TEXT,
                printer_name TEXT,
                printer_job_id TEXT,
                status TEXT NOT NULL,
                failure_reason TEXT,
                is_bulk INTEGER NOT NULL DEFAULT 0,
                bulk_operation_id TEXT,
                recorded_at TEXT NOT NULL,
                started_at TEXT,
                ended_at TEXT
            );

            CREATE INDEX IF NOT EXISTS print_records_ticket_date
                ON print_records(ticket_id, local_date, allocated_number DESC);

            CREATE TABLE IF NOT EXISTS counter_adjustments (
                id TEXT PRIMARY KEY,
                ticket_id TEXT NOT NULL REFERENCES tickets(id) ON DELETE CASCADE,
                subticket_id TEXT NOT NULL DEFAULT '',
                local_date TEXT NOT NULL,
                previous_next_number INTEGER NOT NULL,
                new_next_number INTEGER NOT NULL,
                reason TEXT NOT NULL,
                recorded_at TEXT NOT NULL
            );

            CREATE INDEX IF NOT EXISTS counter_adjustments_ticket_date
                ON counter_adjustments(ticket_id, local_date, recorded_at DESC);

            CREATE TABLE IF NOT EXISTS app_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );
