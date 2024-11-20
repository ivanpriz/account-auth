use super::UnitOfWorkT;

pub trait UnitOfWorkFactoryT<UnitOfWorkType: UnitOfWorkT> {
    // We need to specify lifetime param for easier realizations.
    fn create_unit_of_work(&mut self) -> UnitOfWorkType;
}
