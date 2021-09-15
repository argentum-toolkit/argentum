use std::error::Error;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;

#[derive(thiserror::Error)]
pub enum InternalError {
    #[error("Internal Server Error")]
    Server(#[from] Box<dyn Error>),
}

fn error_chain_fmt(e: &impl Error, f: &mut Formatter<'_>) -> Result {
    writeln!(f, "InternalError => msg: {}", e)?;
    let mut current = e.source();
    while let Some(cause) = current {
        writeln!(f, "Caused by:\n\t{:?} => msg: {}", cause, cause)?;
        current = cause.source();
    }
    Ok(())
}

impl Debug for InternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        error_chain_fmt(self, f)
    }
}
