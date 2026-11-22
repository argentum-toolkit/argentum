pub(crate) struct FileExistenceChecker;

impl FileExistenceChecker {
    pub(crate) fn ensure_absent(&self, path: &str) -> Result<(), String> {
        if std::path::Path::new(path).exists() {
            return Err(format!("File already exists: {path}"));
        }

        Ok(())
    }
}
