use super::{Connector, DataSource, DataSourcePool::Postgres};
use crate::{extend::TomlTableExt, state::State, BoxError};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use toml::Table;

impl Connector for PgPool {
    fn new_data_source(config: &'static Table) -> Result<DataSource, BoxError> {
        let name = config.get_str("name").unwrap_or("postgres");
        let database = config.get_str("database").unwrap_or("postgres");
        let authority = State::format_authority(config, Some(5432));
        let dsn = format!("postgres://{authority}/{database}");

        let max_connections = config.get_u32("max-connections").unwrap_or(16);
        let min_connections = config.get_u32("min-connections").unwrap_or(2);
        let max_lifetime = config.get_u64("max-lifetime").unwrap_or(60 * 60);
        let idle_timeout = config.get_u64("idle-timeout").unwrap_or(10 * 60);
        let acquire_timeout = config.get_u64("acquire-timeout").unwrap_or(30);
        let pool_options = PgPoolOptions::new()
            .max_connections(max_connections)
            .min_connections(min_connections)
            .max_lifetime(Duration::from_secs(max_lifetime))
            .idle_timeout(Duration::from_secs(idle_timeout))
            .acquire_timeout(Duration::from_secs(acquire_timeout));
        let pool = pool_options.connect_lazy(&dsn)?;
        let data_source = DataSource::new("postgres", name, database, Postgres(pool));
        Ok(data_source)
    }

    super::impl_sqlx_connector!(PgPool);
}