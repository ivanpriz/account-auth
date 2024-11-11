use super::unit_of_work::UnitOfWork;
use crate::infrastructure::data::specifications::Specification;

pub trait Repository<'pool, 'tr, Entity, SpecificationType: Specification>:
    Send + Sync + 'static
{
    async fn create(entity: &Entity, uow: &mut UnitOfWork<'pool, 'tr>) -> Entity;

    async fn get_one_by(
        specification: SpecificationType,
        uow: &'pool mut UnitOfWork<'pool, 'tr>,
    ) -> Option<Entity>;
}
