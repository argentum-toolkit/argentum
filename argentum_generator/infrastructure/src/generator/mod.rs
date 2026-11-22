pub mod api_doc;
pub mod business;
pub mod component;
pub mod devops;
pub mod fe_app;
mod file_existence_checker;
mod file_generator;
pub mod infrastructure;
mod mod_file_writer;
pub mod rest;
pub mod ui;

pub use api_doc::ApiDocGenerator;
pub use business::BusinessGenerator;
pub use component::ComponentGenerator;
pub use devops::DevopsGenerator;
pub use file_existence_checker::FileExistenceChecker;
pub use file_generator::FileGenerator;
pub use infrastructure::{DbInfrastructureGenerator, InfrastructureGenerator};
pub use rest::RestGenerator;
pub use ui::UiGenerator;

#[cfg(test)]
pub mod test_util {
    use std::sync::{Mutex, MutexGuard};

    static CWD_MUTEX: Mutex<()> = Mutex::new(());

    /// Guards tests that change the process-wide current directory.
    pub fn lock_cwd() -> MutexGuard<'static, ()> {
        CWD_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
    }
}
