use std::sync::Arc;

use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::Mutex;

pub trait UnitOfWorkFactoryInfraT {
    fn new(pool: PgPool) -> Self
    where
        Self: Sized;
}

pub trait UnitOfWorkInfraT<'tr> {
    fn get_transaction(&mut self) -> &mut Transaction<'tr, Postgres>;

    fn new(pool: Arc<Mutex<PgPool>>) -> Self;
}
