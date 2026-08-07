use super::*;

impl Storage {
    pub fn reserve_print_records(
        &mut self,
        reservation: PrintReservation<'_>,
    ) -> Result<Vec<PrintRecord>> {
        let PrintReservation {
            ticket,
            local_date,
            subticket_ids,
            printer_name,
            is_bulk,
            bulk_operation_id,
            operation_label,
            recorded_at,
            sold_currency,
            sold_amounts_minor,
        } = reservation;
        let transaction = self
        .connection
        .transaction_with_behavior(TransactionBehavior::Immediate)?;
        let mut next_numbers: HashMap<String, i64> = HashMap::new();
        let mut records = Vec::with_capacity(subticket_ids.len());
        for subticket_id in subticket_ids {
            let next_number = match next_numbers.get(subticket_id) {
                Some(value) => *value,
                None => {
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
                    let value = transaction.query_row(
                        "SELECT next_number FROM daily_counters \
                        WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?",
                        params![ticket.id, subticket_id, local_date],
                        |row| row.get(0),
                    )?;
                    next_numbers.insert(subticket_id.clone(), value);
                    value
                }
            };
            let record = PrintRecord {
                id: Uuid::new_v4().to_string(),
                ticket_id: ticket.id.clone(),
                subticket_id: subticket_id.clone(),
                local_date: local_date.to_owned(),
                allocated_number: next_number,
                rendered_output_path: None,
                printer_name: printer_name.map(str::to_owned),
                printer_job_id: None,
                status: "pending".to_owned(),
                failure_reason: None,
                is_bulk,
                bulk_operation_id: bulk_operation_id.map(ToOwned::to_owned),
                operation_label: operation_label.map(ToOwned::to_owned),
                recorded_at: recorded_at.to_owned(),
                started_at: None,
                ended_at: None,
                sold_currency: sold_currency.map(ToOwned::to_owned),
                sold_amount_minor: sold_amounts_minor
                .and_then(|amounts| amounts.get(subticket_id))
                .copied(),
            };
            transaction.execute(
                r#"
                INSERT INTO print_records (
                id, ticket_id, subticket_id, local_date, allocated_number,
                rendered_output_path, printer_name, printer_job_id, status,
                failure_reason, is_bulk, bulk_operation_id, recorded_at,
                started_at, ended_at, operation_label, sold_currency, sold_amount_minor
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
                "#,
                params![
                    record.id,
                    record.ticket_id,
                    record.subticket_id,
                    record.local_date,
                    record.allocated_number,
                    record.rendered_output_path,
                    record.printer_name,
                    record.printer_job_id,
                    record.status,
                    record.failure_reason,
                    record.is_bulk as i64,
                    record.bulk_operation_id,
                    record.recorded_at,
                    record.started_at,
                    record.ended_at,
                    record.operation_label,
                    record.sold_currency,
                    record.sold_amount_minor,
                ],
            )?;
            records.push(record);
            next_numbers.insert(subticket_id.clone(), next_number + ticket.number_increment);
        }

        for (subticket_id, next_number) in next_numbers {
            transaction.execute(
                "UPDATE daily_counters SET next_number = ?, updated_at = ? \
                WHERE ticket_id = ? AND subticket_id = ? AND local_date = ?",
                params![
                    next_number,
                    recorded_at,
                    ticket.id,
                    subticket_id,
                    local_date
                ],
            )?;
        }
        transaction.commit()?;
        Ok(records)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update_print_record(
        &self,
        record_id: &str,
        status: &str,
        rendered_output_path: Option<&str>,
        printer_job_id: Option<&str>,
        failure_reason: Option<&str>,
        started_at: Option<&str>,
        ended_at: Option<&str>,
    ) -> Result<()> {
        self.connection.execute(
            r#"
            UPDATE print_records SET
            status = ?, rendered_output_path = ?, printer_job_id = ?,
            failure_reason = ?, started_at = ?, ended_at = ?
            WHERE id = ?
            "#,
            params![
                status,
                rendered_output_path,
                printer_job_id,
                failure_reason,
                started_at,
                ended_at,
                record_id,
            ],
        )?;
        Ok(())
    }

    pub fn get_print_record(&self, record_id: &str) -> Result<Option<PrintRecord>> {
        self.connection
        .query_row(
            "SELECT id, ticket_id, subticket_id, local_date, allocated_number, \
            rendered_output_path, printer_name, printer_job_id, status, failure_reason, \
            is_bulk, bulk_operation_id, recorded_at, started_at, ended_at, operation_label, \
            sold_currency, sold_amount_minor \
            FROM print_records WHERE id = ?",
            [record_id],
            map_print_record,
        )
        .optional()
        .map_err(Into::into)
    }

    pub fn list_recent_prints(&self, ticket_id: &str, limit: i64) -> Result<Vec<PrintRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT id, ticket_id, subticket_id, local_date, allocated_number, \
            rendered_output_path, printer_name, printer_job_id, status, failure_reason, \
            is_bulk, bulk_operation_id, recorded_at, started_at, ended_at, operation_label, \
            sold_currency, sold_amount_minor \
            FROM print_records WHERE ticket_id = ? ORDER BY recorded_at DESC LIMIT ?",
        )?;
        let rows = statement.query_map(params![ticket_id, limit], map_print_record)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }

    pub fn list_all_print_records(&self, ticket_id: &str) -> Result<Vec<PrintRecord>> {
        let mut statement = self.connection.prepare(
            "SELECT id, ticket_id, subticket_id, local_date, allocated_number, \
            rendered_output_path, printer_name, printer_job_id, status, failure_reason, \
            is_bulk, bulk_operation_id, recorded_at, started_at, ended_at, operation_label, \
            sold_currency, sold_amount_minor \
            FROM print_records WHERE ticket_id = ? ORDER BY recorded_at, allocated_number, id",
        )?;
        let rows = statement.query_map([ticket_id], map_print_record)?;
        rows.collect::<rusqlite::Result<Vec<_>>>()
        .map_err(Into::into)
    }
}
