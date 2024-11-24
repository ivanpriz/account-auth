use framework::application::specifications::{CompType, SpecificationT};
use uuid::Uuid;

pub enum UsersSpecification {
    Id(CompType<Uuid>),
    Username(CompType<String>),
}

impl SpecificationT for UsersSpecification {}
