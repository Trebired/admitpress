use super::*;

impl Storage {
    pub fn snapshot_database(&self, target: &Path) -> Result<()> {
        if target.exists() {
            fs::remove_file(target)?;
        }
        let destination = target.to_string_lossy().replace('\'', "''");
        self.connection
        .execute_batch(&format!("VACUUM INTO '{destination}'"))
        .with_context(|| format!("failed to snapshot the database into {}", target.display()))
    }
}
