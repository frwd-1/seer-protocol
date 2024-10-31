// primitive types for contracts

pub struct Contract<T: Terms> {
    pub terms: Vec<T>,
}
