use argentum_openapi_infrastructure::data_type::SpecificationRoot;
use std::error::Error;

pub(crate) struct OasYamlGenerator {
    base_output_path: String,
}

const PATH: &str = "api-spec/openapi.yaml";

impl OasYamlGenerator {
    pub fn new(base_output_path: String) -> Self {
        Self { base_output_path }
    }

    pub fn generate(&self, spec: &SpecificationRoot) -> Result<(), Box<dyn Error>> {
        let file_path = self.base_output_path.to_owned() + PATH;

        let path = std::path::Path::new(file_path.as_str());
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).unwrap();

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        serde_yaml::to_writer(f, &spec)?;

        Ok(())
    }
}
