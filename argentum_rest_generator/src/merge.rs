use crate::generator::{Combiner, OasLoader};
use std::sync::Arc;

pub(crate) mod generator;
pub(crate) mod template;

fn main() {
    let input = "../argentum_user_account/api-doc/openapi.yaml".to_string();
    let loader = Arc::new(OasLoader::new());
    let combiner = Combiner::new(loader.clone());

    let res = combiner.combine(input);

    let f = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open("test.yml")
        .expect("Couldn't open file");
    serde_yaml::to_writer(f, &res).unwrap();
}
