#[allow(unused_imports)]
use diesel::{
    pg::PgConnection,
    r2d2::{self, ConnectionManager},
    sql_query,
};
use diesel_migrations::embed_migrations;
use log::info;

embed_migrations!();

pub type Connection = PgConnection;

pub type Pool = r2d2::Pool<ConnectionManager<Connection>>;

pub fn migrate_and_config(url: &str) -> Pool {
    info!("Migrating and configuring database...");
    let manager = ConnectionManager::<Connection>::new(url);
    let pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
    embedded_migrations::run(&pool.get().expect("Failed to migrate."));

    pool
}