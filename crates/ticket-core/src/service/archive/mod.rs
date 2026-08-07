use super::*;

mod assets;
mod files;
mod paths;

use assets::*;
use files::*;
use paths::*;

impl TicketService {
    pub fn export_ticket_archive(
        &mut self,
        ticket_id: &str,
        destination_path: &str,
    ) -> Result<String> {
        let ticket = self
        .storage
        .get_ticket(ticket_id)?
        .ok_or(TicketError::TicketNotFound)?;
        let fields = self.storage.list_fields(ticket_id)?;
        let subtickets = self.storage.list_subtickets(ticket_id)?;
        let daily_counters = self.storage.list_daily_counters(ticket_id)?;
        let print_records = self.storage.list_all_print_records(ticket_id)?;
        let counter_adjustments = self.storage.list_counter_adjustments(ticket_id)?;
        let assets = collect_ticket_archive_assets(&ticket, &fields, &print_records);
        let manifest = TicketArchiveManifest {
            format: TICKET_ARCHIVE_FORMAT.to_owned(),
            version: TICKET_ARCHIVE_VERSION,
            exported_at: now_iso(),
            ticket,
            fields,
            subtickets,
            daily_counters,
            print_records,
            counter_adjustments,
            assets,
        };

        let destination = archive_destination_path(destination_path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let file = fs::File::create(&destination)
        .with_context(|| format!("failed to create archive at {}", destination.display()))?;
        let encoder = GzEncoder::new(file, Compression::default());
        let mut builder = Builder::new(encoder);
        let manifest_bytes = serde_json::to_vec_pretty(&manifest)?;
        append_archive_bytes(&mut builder, TICKET_ARCHIVE_MANIFEST, &manifest_bytes)?;
        for asset in &manifest.assets {
            builder
            .append_path_with_name(&asset.original_path, &asset.archive_path)
            .with_context(|| {
                    format!("failed to add {} to ticket archive", asset.original_path)
            })?;
        }
        let encoder = builder.into_inner()?;
        encoder.finish()?;
        Ok(destination.to_string_lossy().into_owned())
    }

    pub fn import_ticket_archive(&mut self, source_path: &str) -> Result<Ticket> {
        let source = PathBuf::from(decode_file_path(source_path));
        if !source.is_file() {
            return Err(anyhow!("selected ticket archive does not exist"));
        }
        let (manifest, asset_bytes) = read_ticket_archive(&source)?;
        if manifest.format != TICKET_ARCHIVE_FORMAT {
            return Err(anyhow!("selected file is not a supported ticket archive"));
        }
        if manifest.version != TICKET_ARCHIVE_VERSION {
            return Err(anyhow!(
                    "unsupported ticket archive version {}",
                    manifest.version
            ));
        }

        let imported = self.build_imported_ticket(manifest, asset_bytes)?;
        self.storage.insert_ticket_archive(
            &imported.ticket,
            &imported.fields,
            &imported.subtickets,
            &imported.daily_counters,
            &imported.print_records,
            &imported.counter_adjustments,
        )?;
        Ok(imported.ticket)
    }

    fn build_imported_ticket(
        &self,
        manifest: TicketArchiveManifest,
        asset_bytes: HashMap<String, Vec<u8>>,
    ) -> Result<ImportedTicketArchive> {
        let now = now_iso();
        let old_ticket_id = manifest.ticket.id.clone();
        let new_ticket_id = Uuid::new_v4().to_string();
        let asset_lookup = manifest
        .assets
        .iter()
        .map(|asset| {
                (
                    (asset.kind.clone(), asset.original_path.clone()),
                    asset.archive_path.clone(),
                )
        })
        .collect::<HashMap<_, _>>();

        let mut subticket_id_map = HashMap::new();
        let subtickets = manifest
        .subtickets
        .into_iter()
        .map(|mut subticket| {
                let old_id = subticket.id;
                let new_id = Uuid::new_v4().to_string();
                subticket_id_map.insert(old_id, new_id.clone());
                subticket.id = new_id;
                subticket.ticket_id = new_ticket_id.clone();
                subticket
        })
        .collect::<Vec<_>>();

        let mut ticket = manifest.ticket;
        ticket.id = new_ticket_id.clone();
        ticket.recorded_at = now.clone();
        ticket.updated_at = now;
        ticket.background_svg_path = if let Some(path) = ticket.background_svg_path.as_deref() {
            imported_asset_path(
                &asset_lookup,
                &asset_bytes,
                "background",
                path,
                self.data_dir
                .join("tickets")
                .join(&new_ticket_id)
                .join("background.svg"),
            )?
        } else {
            None
        };

        let fields = manifest
        .fields
        .into_iter()
        .map(|mut field| {
                field.id = Uuid::new_v4().to_string();
                field.ticket_id = new_ticket_id.clone();
                if field.field_kind == FieldKind::Image && !field.text.trim().is_empty() {
                    if let Some(extension) = path_extension(&field.text) {
                        let target = self
                        .data_dir
                        .join("tickets")
                        .join(&new_ticket_id)
                        .join("images")
                        .join(format!("{}.{}", Uuid::new_v4(), extension));
                        field.text = imported_asset_path(
                            &asset_lookup,
                            &asset_bytes,
                            "image",
                            &field.text,
                            target,
                        )?
                        .unwrap_or_default();
                    } else {
                        field.text.clear();
                    }
                }
                Ok(field)
        })
        .collect::<Result<Vec<_>>>()?;

        let daily_counters = manifest
        .daily_counters
        .into_iter()
        .filter_map(|mut counter| {
                let subticket_id = subticket_id_map.get(&counter.subticket_id)?.clone();
                counter.ticket_id = new_ticket_id.clone();
                counter.subticket_id = subticket_id;
                Some(counter)
        })
        .collect::<Vec<_>>();

        let print_records = manifest
        .print_records
        .into_iter()
        .filter_map(|mut record| {
                let subticket_id = subticket_id_map.get(&record.subticket_id)?.clone();
                let old_rendered_output_path = record.rendered_output_path.clone();
                record.id = Uuid::new_v4().to_string();
                record.ticket_id = new_ticket_id.clone();
                record.subticket_id = subticket_id;
                if let Some(path) = old_rendered_output_path.as_deref() {
                    let target = imported_print_output_path(
                        &self.data_dir,
                        &new_ticket_id,
                        &record.id,
                        &record.local_date,
                        path,
                    );
                    match imported_asset_path(&asset_lookup, &asset_bytes, "output", path, target) {
                        Ok(path) => record.rendered_output_path = path,
                        Err(error) => return Some(Err(error)),
                    }
                } else {
                    record.rendered_output_path = None;
                }
                Some(Ok(record))
        })
        .collect::<Result<Vec<_>>>()?;

        let counter_adjustments = manifest
        .counter_adjustments
        .into_iter()
        .filter_map(|mut adjustment| {
                let subticket_id = subticket_id_map.get(&adjustment.subticket_id)?.clone();
                adjustment.id = Uuid::new_v4().to_string();
                adjustment.ticket_id = new_ticket_id.clone();
                adjustment.subticket_id = subticket_id;
                Some(adjustment)
        })
        .collect::<Vec<_>>();

        if old_ticket_id == new_ticket_id {
            return Err(anyhow!("failed to allocate a new imported ticket id"));
        }

        Ok(ImportedTicketArchive {
                ticket,
                fields,
                subtickets,
                daily_counters,
                print_records,
                counter_adjustments,
        })
    }
}
