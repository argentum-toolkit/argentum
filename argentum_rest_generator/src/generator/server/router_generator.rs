use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{Operation, SpecificationRoot};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;

pub(crate) struct RouterGenerator {
    renderer: Arc<Renderer>,
}

const PATH: &str = "/src/server/router.rs";
const TEMPLATE: &str = "server/router";

#[derive(Debug, Clone, serde::Serialize)]
struct PathData {
    pub segments: Vec<String>,
    pub operations: HashMap<String, Operation>,
}

impl RouterGenerator {
    pub fn new(renderer: Arc<Renderer>) -> Self {
        Self { renderer }
    }

    pub fn generate(&self, spec: &SpecificationRoot) -> Result<(), Box<dyn Error>> {
        let mut paths_data: Vec<PathData> = vec![];

        for (url, path) in spec.clone().paths {
            let segments: Vec<_> = url
                .split('/')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();

            let mut operations: HashMap<String, Operation> = HashMap::new();

            for (method, operation) in path.operations.clone() {
                operations.insert(
                    ("Method::".to_owned() + method.to_string().as_str()).to_string(),
                    operation,
                );
            }
            let item = PathData {
                segments,
                operations,
            };

            paths_data.push(item);
        }

        let data = HashMap::from([("paths", paths_data)]);
        self.renderer.render(TEMPLATE, data, PATH)?;

        Ok(())
    }
}
