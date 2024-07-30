mod migration_000_init;

use argentum_standard_infrastructure::db::slqx_postgres::migration::MigrationCollection;

pub fn up(table_name_prefix: &str) -> MigrationCollection {
    MigrationCollection::from([("000_init", migration_000_init::up(table_name_prefix))])
}
