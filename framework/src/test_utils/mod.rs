use std::process::Command;

use super::infrastructure::data::unit_of_work::{
    traits::UnitOfWorkFactoryInfraT, UnitOfWorkFactory,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::runtime::{Builder, Runtime};

use rstest::fixture;

// As we need a way for fixtures to clean up stuff after a test has run,
// we will use this structure to store the return value, and then run some code on drop.
// Also found a way to test async code without using async tests, and with the ability to do
// cleanup on drop - we can create runtime fixture and drill it through all the fixtures.
// With such approach all the fixts and tests remains sync, and we can call async cleanups in
// drop.
pub struct WithCleanup<ValT> {
    pub closure: Box<dyn FnMut() -> ()>,
    pub _val: ValT,
}

impl<ValT> Drop for WithCleanup<ValT> {
    fn drop(&mut self) {
        (*self.closure)();
    }
}

#[fixture]
pub fn runtime() -> Runtime {
    Builder::new_current_thread().enable_all().build().unwrap()
}

#[fixture]
pub fn migrations() -> WithCleanup<()> {
    println!("Applying migrations...");
    Command::new("sqlx")
        .arg("migrate")
        .arg("run")
        .output()
        .expect("Error running migrations");
    println!("Migrations applied");

    WithCleanup {
        _val: (),
        closure: Box::new(|| {
            // Don't need to drop migrations now
            // println!("Reverting migrations...");
            // Command::new("sqlx")
            //     .arg("migrate")
            //     .arg("revert")
            //     .output()
            //     .expect("Error reverting migrations");
            // println!("Migrations reverted");
        }),
    }
}

#[fixture]
pub fn pg_pool(runtime: Runtime) -> (PgPool, Runtime) {
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL env var should be set");

    let pool = runtime
        .block_on(
            PgPoolOptions::new()
                .max_connections(30)
                .connect(&database_url),
        )
        .expect("Couldn't connect to db");

    (pool, runtime)
}

#[fixture]
pub fn uow_factory(pg_pool: (PgPool, Runtime)) -> (UnitOfWorkFactory, Runtime) {
    (UnitOfWorkFactory::new(pg_pool.0), pg_pool.1)
}
