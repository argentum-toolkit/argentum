use std::io::Write;

/// Appends non-duplicate lines to a mod file. Creates the file (and its parent
/// directories) if it does not exist yet.
pub(crate) fn append_mod(mod_path: &str, lines: Vec<String>) -> Result<(), String> {
    if let Some(parent) = std::path::Path::new(mod_path).parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let content = if std::path::Path::new(mod_path).exists() {
        std::fs::read_to_string(mod_path).map_err(|e| e.to_string())?
    } else {
        String::new()
    };

    let existing: Vec<&str> = content.lines().map(str::trim).collect();

    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(mod_path)
        .map_err(|e| e.to_string())?;

    for line in lines {
        if !existing.contains(&line.trim()) {
            writeln!(file, "{}", line).map_err(|e| e.to_string())?
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn append_mod_creates_file_and_writes_lines() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();

        append_mod(path_str, vec!["mod user_account_dto;".to_string()]).unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "mod user_account_dto;\n"
        );
    }

    #[test]
    fn append_mod_creates_parent_directories() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("nested/dir/mod.rs");
        let path_str = path.to_str().unwrap();

        append_mod(path_str, vec!["pub mod repository;".to_string()]).unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "pub mod repository;\n"
        );
    }

    #[test]
    fn append_mod_creates_empty_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();

        append_mod(path_str, Vec::new()).unwrap();

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "");
    }

    #[test]
    fn append_mod_does_not_duplicate_lines() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();
        let lines = vec![
            "mod user_account_dto;".to_string(),
            "pub use user_account_dto::UserAccountDto;".to_string(),
        ];

        append_mod(path_str, lines.clone()).unwrap();
        append_mod(path_str, lines).unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "mod user_account_dto;\npub use user_account_dto::UserAccountDto;\n"
        );
    }

    #[test]
    fn append_mod_does_not_skip_line_substring_of_existing_line() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();
        std::fs::write(&path, "pub use user_repository::UserRepositoryFactory;\n").unwrap();

        append_mod(
            path_str,
            vec!["pub use user_repository::UserRepository;".to_string()],
        )
        .unwrap();

        let content = std::fs::read_to_string(&path).unwrap();
        assert!(content.contains("pub use user_repository::UserRepositoryFactory;"));
        assert!(content.contains("pub use user_repository::UserRepository;\n"));
    }

    #[test]
    fn append_mod_tolerates_whitespace_differences() {
        let temp_dir = tempfile::tempdir().unwrap();
        let path = temp_dir.path().join("mod.rs");
        let path_str = path.to_str().unwrap();
        std::fs::write(&path, "  mod user_account_dto;  \n").unwrap();

        append_mod(path_str, vec!["mod user_account_dto;".to_string()]).unwrap();

        assert_eq!(
            std::fs::read_to_string(&path).unwrap(),
            "  mod user_account_dto;  \n"
        );
    }
}
