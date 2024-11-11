/*
Actually this might be more properly called a transaction wrapper as we don't merge or optimize sql queries here.
*/

use std::sync::Arc;
use tokio::sync::Mutex;

use super::traits::UnitOfWorkInfraT;
use crate::application::traits::UnitOfWorkT;
use sqlx::{PgPool, Postgres, Transaction};

// Ok so the problem is most likely that transaction doesn't have to live as long as pool - separate lifetimes
pub struct UnitOfWork<'tr> {
    // The struct itself should never live less than the pool which is stored in factory
    pool: Arc<Mutex<PgPool>>,
    transaction: Option<Transaction<'tr, Postgres>>,
}

impl<'tr> UnitOfWorkT for UnitOfWork<'tr> {
    async fn begin(&mut self) {
        self.transaction = Some(
            self.pool
                .lock()
                .await
                //.expect("Failed to get mut ref for pool")
                .begin()
                .await
                .expect("Couln't begin the transaction"),
        );
    }

    async fn commit(&mut self) {
        let transaction = self.transaction.take();
        match transaction {
            None => panic!("No transaction when trying to commit"),
            Some(transaction) => transaction
                .commit()
                .await
                .expect("Failed to commit transaction"),
        };
    }

    async fn rollback(&mut self) {
        let transaction = self.transaction.take();
        match transaction {
            None => panic!("No transaction when trying to rollback"),
            Some(transaction) => transaction
                .rollback()
                .await
                .expect("Failed to rollback transaction"),
        };
    }
}

impl<'tr> UnitOfWorkInfraT<'tr> for UnitOfWork<'tr> {
    fn get_transaction(&mut self) -> &mut Transaction<'tr, Postgres> {
        match &mut self.transaction {
            None => panic!("No transaction yet"),
            Some(transaction) => transaction,
        }
    }

    fn new(pool: Arc<Mutex<PgPool>>) -> Self {
        Self {
            pool: pool,
            transaction: None,
        }
    }
}
