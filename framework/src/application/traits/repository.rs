use std::future::Future;

use super::super::specifications::SpecificationT;
use super::unit_of_work::UnitOfWorkT;

pub trait RepositoryT<Entity, SpecificationType: SpecificationT, UnitOfWorkType: UnitOfWorkT>:
    Send + Sync + 'static
{
    fn create(&self, entity: &Entity, uow: &mut UnitOfWorkType) -> impl Future<Output = Entity>;

    fn get_one_by(
        &self,
        specification: SpecificationType,
        uow: &mut UnitOfWorkType,
    ) -> impl Future<Output = Option<Entity>>;
}
