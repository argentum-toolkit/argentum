pub struct FileExistenceChecker;

impl FileExistenceChecker {
    pub fn exists(&self, path: &str) -> bool {
        std::path::Path::new(path).exists()
    }
}
