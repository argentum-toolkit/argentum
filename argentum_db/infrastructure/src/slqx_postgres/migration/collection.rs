use std::collections::BTreeMap;

pub type MigrationCollection<'a> = BTreeMap<&'a str, Vec<String>>;
