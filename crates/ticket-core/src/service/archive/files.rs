use super::*;

pub(super) fn read_ticket_archive(
    source: &Path,
) -> Result<(TicketArchiveManifest, HashMap<String, Vec<u8>>)> {
    let file = fs::File::open(source)
    .with_context(|| format!("failed to open ticket archive at {}", source.display()))?;
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    let mut manifest_bytes = None;
    let mut asset_bytes = HashMap::new();
    for entry in archive.entries()? {
        let mut entry = entry?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let path = entry.path()?.to_string_lossy().replace('\\', "/");
        validate_archive_member_path(&path)?;
        let mut bytes = Vec::new();
        entry.read_to_end(&mut bytes)?;
        if path == TICKET_ARCHIVE_MANIFEST {
            manifest_bytes = Some(bytes);
        } else {
            asset_bytes.insert(path, bytes);
        }
    }
    let manifest_bytes = manifest_bytes
    .ok_or_else(|| anyhow!("ticket archive is missing {TICKET_ARCHIVE_MANIFEST}"))?;
    let manifest = serde_json::from_slice::<TicketArchiveManifest>(&manifest_bytes)
    .with_context(|| "ticket archive manifest is invalid")?;
    for asset in &manifest.assets {
        validate_archive_member_path(&asset.archive_path)?;
    }
    Ok((manifest, asset_bytes))
}

pub(super) fn validate_archive_member_path(path: &str) -> Result<()> {
    if path.trim().is_empty() {
        return Err(anyhow!("ticket archive contains an empty path"));
    }
    for component in Path::new(path).components() {
        match component {
            Component::Normal(_) => {}
            _ => return Err(anyhow!("ticket archive contains an unsafe path: {path}")),
        }
    }
    Ok(())
}

pub(super) fn imported_asset_path(
    asset_lookup: &HashMap<(String, String), String>,
    asset_bytes: &HashMap<String, Vec<u8>>,
    kind: &str,
    original_path: &str,
    target: PathBuf,
) -> Result<Option<String>> {
    let key = (kind.to_owned(), original_path.to_owned());
    let Some(archive_path) = asset_lookup.get(&key) else {
        return Ok(None);
    };
    let bytes = asset_bytes
    .get(archive_path)
    .ok_or_else(|| anyhow!("ticket archive is missing asset {archive_path}"))?;
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&target, bytes)
    .with_context(|| format!("failed to restore imported asset to {}", target.display()))?;
    Ok(Some(target.to_string_lossy().into_owned()))
}
