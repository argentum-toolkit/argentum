use argentum_log_business::LoggerTrait;
use argentum_openapi_infrastructure::data_type::SpecificationRoot;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;

pub struct OasLoader {
    logger: Arc<dyn LoggerTrait>,
}

impl OasLoader {
    pub fn new(logger: Arc<dyn LoggerTrait>) -> Self {
        Self { logger }
    }

    pub fn load(&self, file_path: String) -> (SpecificationRoot, PathBuf) {
        self.logger.debug(format!("Loading OAS from {}", file_path));

        let path = PathBuf::from(file_path.clone());

        let f = fs::File::open(path.clone())
            .unwrap_or_else(|_| panic!("Should have been able to read the file {}", file_path));

        let spec: SpecificationRoot = serde_yaml::from_reader(f)
            .unwrap_or_else(|_| panic!("Could not read values from '{}'.", file_path));

        (spec, path)
    }
}
