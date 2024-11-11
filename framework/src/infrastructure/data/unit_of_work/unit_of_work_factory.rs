use std::sync::Arc;

use super::{
    traits::{UnitOfWorkFactoryInfraT, UnitOfWorkInfraT},
    UnitOfWork,
};
use crate::application::traits::UnitOfWorkFactoryT;
use sqlx::PgPool;
use tokio::sync::Mutex;

/* Ideally we want to depend only on UnitOfWorkPublic trait.
However now it's possible only by returning reference which is not what I want.
So currently we are binded to the implementation until Rust start supporting impl Trait as return type.
Update: No, as I can just use generic, what I actually did.
*/
pub struct UnitOfWorkFactory {
    pool: Arc<Mutex<PgPool>>,
}

impl<'tr> UnitOfWorkFactoryT<UnitOfWork<'tr>> for UnitOfWorkFactory {
    fn create_unit_of_work(&mut self) -> UnitOfWork<'tr> {
        UnitOfWork::new(self.pool.clone())
    }
}

impl UnitOfWorkFactoryInfraT for UnitOfWorkFactory {
    fn new(pool: PgPool) -> Self {
        Self {
            pool: Arc::new(Mutex::new(pool)),
        }
    }
}
