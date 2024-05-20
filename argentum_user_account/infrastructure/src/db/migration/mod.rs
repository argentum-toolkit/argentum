mod migration_000_init;

pub fn up() -> Vec<String> {
    vec![migration_000_init::up()]
}
