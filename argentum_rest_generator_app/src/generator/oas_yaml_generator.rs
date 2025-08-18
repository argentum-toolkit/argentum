use argentum_openapi_infrastructure::data_type::SpecificationRoot;

pub(crate) struct OasYamlGenerator {}

const PATH: &str = "api-spec/openapi.yaml";

impl OasYamlGenerator {
    pub fn new() -> Self {
        Self {}
    }

    pub fn generate(&self, base_output_path: &str, spec: &SpecificationRoot) -> Result<(), String> {
        let file_path = base_output_path.to_owned() + PATH;

        let path = std::path::Path::new(file_path.as_str());
        let prefix = path
            .parent()
            .ok_or(format!("Can't find parent path: {path:?}"))?;
        std::fs::create_dir_all(prefix).map_err(|e| format!("Can't create dir. Error: {e}"))?;

        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(file_path)
            .map_err(|e| format!("Can't open file. Error: {e}"))?;

        serde_yaml_ng::to_writer(f, &spec)
            .map_err(|e| format!("Can't write yaml file. Error: {e}"))?;

        Ok(())
    }
}
