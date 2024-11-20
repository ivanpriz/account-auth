use std::future::Future;

pub trait UnitOfWorkT {
    fn begin(&mut self) -> impl Future<Output = ()>;

    fn commit(&mut self) -> impl Future<Output = ()>;

    fn rollback(&mut self) -> impl Future<Output = ()>;
}
