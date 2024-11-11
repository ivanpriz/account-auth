use framework::application::specifications::{CompType, SpecificationT};
use uuid::Uuid;

pub enum UsersSpecification {
    Id(CompType<Uuid>),
    Email(CompType<String>),
}

impl SpecificationT for UsersSpecification {}
