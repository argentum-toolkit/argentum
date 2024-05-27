use argentum_openapi_infrastructure::data_type::SpecificationRoot;
use std::fs;
use std::path::PathBuf;

pub struct OasLoader {}

impl OasLoader {
    pub fn new() -> Self {
        Self {}
    }

    pub fn load(&self, file_path: String) -> (SpecificationRoot, PathBuf) {
        let path = PathBuf::from(file_path.clone());

        let f = fs::File::open(path.clone()).expect(&format!(
            "Should have been able to read the file {}",
            file_path
        ));

        let spec: SpecificationRoot = serde_yaml::from_reader(f)
            .expect(&format!("Could not read values from '{}'.", file_path));

        (spec, path)
    }
}
