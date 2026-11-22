pub(crate) mod api_doc;
pub(crate) mod business;
pub(crate) mod component;
pub(crate) mod devops;
pub(crate) mod fe_app;
mod file_existence_checker;
mod file_generator;
pub(crate) mod infrastructure;
mod mod_file_writer;
pub(crate) mod ui;

pub(crate) use api_doc::ApiDocGenerator;
pub(crate) use business::BusinessGenerator;
pub(crate) use component::ComponentGenerator;
pub(crate) use devops::DevopsGenerator;
pub(crate) use file_existence_checker::FileExistenceChecker;
pub(crate) use file_generator::FileGenerator;
pub(crate) use infrastructure::{DbInfrastructureGenerator, InfrastructureGenerator};
pub(crate) use ui::UiGenerator;

#[cfg(test)]
pub(crate) mod test_util {
    use std::sync::{Mutex, MutexGuard};

    static CWD_MUTEX: Mutex<()> = Mutex::new(());

    /// Guards tests that change the process-wide current directory.
    pub(crate) fn lock_cwd() -> MutexGuard<'static, ()> {
        CWD_MUTEX.lock().unwrap_or_else(|e| e.into_inner())
    }
}
