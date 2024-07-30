use crate::generator::path_param::regex::RegexFactory;
use crate::template::Renderer;
use argentum_openapi_infrastructure::data_type::{
    InPlace, Operation, Parameter, SpecificationRoot,
};
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::Arc;

pub(crate) struct RouterGenerator {
    renderer: Arc<Renderer>,
    regex_factory: Arc<RegexFactory>,
}

const PATH: &str = "/src/server/router.rs";
const TEMPLATE: &str = "server/router";

#[derive(Debug, Clone, serde::Serialize)]
struct PathData {
    pub pattern: String,
    pub operations: BTreeMap<String, Operation>,
    pub params: Vec<Parameter>,
}

impl RouterGenerator {
    pub fn new(renderer: Arc<Renderer>, regex_factory: Arc<RegexFactory>) -> Self {
        Self {
            renderer,
            regex_factory,
        }
    }

    pub fn generate(
        &self,
        base_output_path: &str,
        spec: &SpecificationRoot,
    ) -> Result<(), Box<dyn Error>> {
        let mut paths_data: Vec<PathData> = vec![];

        for (url, path) in spec.clone().paths {
            let mut operations: BTreeMap<String, Operation> = BTreeMap::new();

            let mut path_params = match path.parameters {
                None => vec![],
                Some(params) => params
                    .into_iter()
                    .filter(|p| p.in_place == InPlace::Path)
                    .collect(),
            };

            for (method, operation) in path.operations.clone() {
                let mut operation_path_params = match operation.parameters.clone() {
                    None => vec![],
                    Some(params) => params
                        .into_iter()
                        .filter(|p| p.in_place == InPlace::Path)
                        .collect(),
                };
                path_params.append(&mut operation_path_params);

                operations.insert(
                    ("Method::".to_owned() + method.to_string().as_str()).to_string(),
                    operation,
                );
            }

            let mut pattern = url.replace('/', "\\/");
            for param in path_params.iter() {
                let find = format!("{{{}}}", param.name);

                let reg = self.regex_factory.create(param);

                pattern = pattern.replace(find.as_str(), reg.as_str());
            }

            let item = PathData {
                pattern,
                operations,
                params: path_params,
            };

            paths_data.push(item);
        }

        let data = HashMap::from([("paths", paths_data)]);
        self.renderer
            .render(base_output_path, TEMPLATE, data, PATH)?;

        Ok(())
    }
}
