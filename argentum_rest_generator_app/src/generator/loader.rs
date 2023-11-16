use argentum_openapi_infrastructure::data_type::SpecificationRoot;
use std::fs;
use std::path::PathBuf;

pub struct OasLoader {}

impl OasLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load(&self, file_path: String) -> (SpecificationRoot, PathBuf) {
        let path = PathBuf::from(file_path);

        let f = fs::File::open(path.clone())
            .expect("LogRocket: Should have been able to read the file");

        let spec: SpecificationRoot = serde_yaml::from_reader(f).expect("Could not read values.");

        (spec, path)
    }
}
