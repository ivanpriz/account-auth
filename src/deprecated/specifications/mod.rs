use uuid::Uuid;

pub enum CompType<T> {
    Equals(T),
    Gte(T),
    Lte(T),
    Lt(T),
    Gt(T),
}

pub enum UsersSpecification {
    Id(CompType<Uuid>),
    Email(CompType<String>),
}

pub trait Specification {}

impl Specification for UsersSpecification {}
