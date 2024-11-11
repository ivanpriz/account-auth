pub enum CompType<T> {
    Equals(T),
    Gte(T),
    Lte(T),
    Lt(T),
    Gt(T),
}

pub trait SpecificationT {}
