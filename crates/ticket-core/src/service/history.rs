use super::*;

struct HistoryBuilder {
    entry: PrintHistoryEntry,
    item_indices: HashMap<String, usize>,
    item_sort_orders: HashMap<String, i64>,
}

impl TicketService {
    pub(super) fn print_operations_bundle(
        &mut self,
        ticket_id: &str,
        local_date: &str,
        recent_limit: usize,
    ) -> Result<(Vec<PrintHistoryEntry>, Vec<PrintHistoryEntry>)> {
        let entries = self.print_operations_all(ticket_id)?;
        let date_entries = entries
        .iter()
        .filter(|entry| entry.local_date == local_date)
        .cloned()
        .collect();
        let recent_entries = entries.into_iter().take(recent_limit).collect();
        Ok((date_entries, recent_entries))
    }

    fn print_operations_all(&mut self, ticket_id: &str) -> Result<Vec<PrintHistoryEntry>> {
        let subtickets = self
        .storage
        .list_subtickets(ticket_id)?
        .into_iter()
        .map(|subticket| (subticket.id.clone(), subticket))
        .collect::<HashMap<_, _>>();
        let records = self.storage.list_all_print_records(ticket_id)?;
        let mut order = Vec::new();
        let mut builders = HashMap::new();

        for record in records {
            let key = operation_key(&record);
            if !builders.contains_key(&key) {
                let print_number = order.len() as i64 + 1;
                builders.insert(
                    key.clone(),
                    HistoryBuilder::new(&key, &record, print_number),
                );
                order.push(key.clone());
            }
            let builder = builders
            .get_mut(&key)
            .expect("history builder should exist before adding records");
            let subticket = subtickets.get(&record.subticket_id);
            builder.add_record(record, subticket);
        }

        let mut entries = order
        .into_iter()
        .filter_map(|key| builders.remove(&key).map(HistoryBuilder::finish))
        .collect::<Vec<_>>();
        entries.sort_by_key(|entry| std::cmp::Reverse(entry.print_number));
        Ok(entries)
    }
}

impl HistoryBuilder {
    fn new(operation_id: &str, record: &PrintRecord, print_number: i64) -> Self {
        Self {
            entry: PrintHistoryEntry {
                id: operation_id.to_owned(),
                ticket_id: record.ticket_id.clone(),
                print_number,
                local_date: record.local_date.clone(),
                is_bulk: record.is_bulk,
                operation_label: record.operation_label.clone(),
                status: record.status.clone(),
                failure_reason: record.failure_reason.clone(),
                ticket_count: 0,
                total_minor: 0,
                currency: String::new(),
                recorded_at: record.recorded_at.clone(),
                started_at: record.started_at.clone(),
                ended_at: record.ended_at.clone(),
                rendered_output_path: record.rendered_output_path.clone(),
                printer_name: record.printer_name.clone(),
                printer_job_id: record.printer_job_id.clone(),
                items: Vec::new(),
            },
            item_indices: HashMap::new(),
            item_sort_orders: HashMap::new(),
        }
    }

    fn add_record(&mut self, record: PrintRecord, subticket: Option<&Subticket>) {
        let item_index = self.item_index(&record, subticket);
        let item = &mut self.entry.items[item_index];
        item.quantity += 1;
        item.total_minor += item.amount_minor;
        item.ticket_numbers.push(record.allocated_number);
        self.entry.ticket_count += 1;
        self.entry.total_minor += item.amount_minor;
        if self.entry.currency.is_empty() {
            self.entry.currency = item.currency.clone();
        }
        self.merge_record_metadata(&record);
    }

    fn item_index(&mut self, record: &PrintRecord, subticket: Option<&Subticket>) -> usize {
        if let Some(index) = self.item_indices.get(&record.subticket_id) {
            return *index;
        }
        let index = self.entry.items.len();
        self.entry.items.push(history_item(record, subticket));
        self.item_indices.insert(record.subticket_id.clone(), index);
        self.item_sort_orders.insert(
            record.subticket_id.clone(),
            subticket.map(|value| value.sort_order).unwrap_or(i64::MAX),
        );
        index
    }

    fn merge_record_metadata(&mut self, record: &PrintRecord) {
        self.entry.status = merged_status(&self.entry.status, &record.status);
        if self.entry.failure_reason.is_none() {
            self.entry.failure_reason = record.failure_reason.clone();
        }
        if self.entry.rendered_output_path.is_none() {
            self.entry.rendered_output_path = record.rendered_output_path.clone();
        }
        if self.entry.printer_name.is_none() {
            self.entry.printer_name = record.printer_name.clone();
        }
        if self.entry.printer_job_id.is_none() {
            self.entry.printer_job_id = record.printer_job_id.clone();
        }
        if self.entry.started_at.is_none() {
            self.entry.started_at = record.started_at.clone();
        }
        if self.entry.ended_at.is_none() {
            self.entry.ended_at = record.ended_at.clone();
        }
        self.entry.is_bulk = self.entry.is_bulk || record.is_bulk;
        if self.entry.operation_label.is_none() {
            self.entry.operation_label = record.operation_label.clone();
        }
    }

    fn finish(mut self) -> PrintHistoryEntry {
        for item in &mut self.entry.items {
            item.ticket_numbers.sort_unstable();
        }
        self.entry.items.sort_by_key(|item| {
                self.item_sort_orders
                .get(&item.subticket_id)
                .copied()
                .unwrap_or(i64::MAX)
        });
        self.entry
    }
}

fn operation_key(record: &PrintRecord) -> String {
    if record.is_bulk {
        return record
        .bulk_operation_id
        .clone()
        .unwrap_or_else(|| record.id.clone());
    }
    record.id.clone()
}

fn history_item(record: &PrintRecord, subticket: Option<&Subticket>) -> PrintHistoryItem {
    let amount_minor = record.sold_amount_minor.unwrap_or_else(|| {
            subticket
            .map(|value| value.amount_minor)
            .unwrap_or_default()
    });
    let currency = record
    .sold_currency
    .as_deref()
    .map(str::trim)
    .filter(|value| !value.is_empty())
    .map(ToOwned::to_owned)
    .unwrap_or_else(|| {
            subticket
            .map(|value| value.currency.clone())
            .unwrap_or_default()
    });
    match subticket {
        Some(subticket) => PrintHistoryItem {
            subticket_id: record.subticket_id.clone(),
            name: subticket.name.clone(),
            display_label: subticket.display_label.clone(),
            amount_minor,
            currency,
            quantity: 0,
            total_minor: 0,
            ticket_numbers: Vec::new(),
        },
        None => PrintHistoryItem {
            subticket_id: record.subticket_id.clone(),
            name: "Unknown price".to_owned(),
            display_label: String::new(),
            amount_minor,
            currency,
            quantity: 0,
            total_minor: 0,
            ticket_numbers: Vec::new(),
        },
    }
}

fn merged_status(left: &str, right: &str) -> String {
    if left == "failed" || right == "failed" {
        "failed".to_owned()
    } else if left == "pending" || right == "pending" {
        "pending".to_owned()
    } else if left == "queued" || right == "queued" {
        "queued".to_owned()
    } else {
        right.to_owned()
    }
}
