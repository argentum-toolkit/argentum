use argentum_log_business::LoggerTrait;
use argentum_openapi_infrastructure::data_type::SpecificationRoot;
use std::error::Error;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub struct OasLoader<L>
where
    L: LoggerTrait,
{
    logger: Arc<L>,
}

impl<L> OasLoader<L>
where
    L: LoggerTrait,
{
    pub fn new(logger: Arc<L>) -> Self {
        Self { logger }
    }

    pub fn load(&self, file_path: String) -> Result<(SpecificationRoot, PathBuf), Box<dyn Error>> {
        self.logger.debug(format!("Loading OAS from {file_path}"));

        let path = PathBuf::from(file_path.clone());

        let f = fs::File::open(path.clone())
            .map_err(|_| format!("Should have been able to read the file {file_path}"))?;

        let spec: SpecificationRoot = serde_yaml_ng::from_reader(f)
            .map_err(|_| format!("Could not read values from '{file_path}'."))?;

        Ok((spec, path))
    }
}
