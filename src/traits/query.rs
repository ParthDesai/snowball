pub trait Query {
    type Context: QueryContext;
    fn context(&self) -> &Self::Context;
}

pub trait QueryContext {
    type Error: crate::traits::error::Error;

    fn get(&self, key: String) -> String;
    fn set(&mut self, key: String, value: String) -> Result<(), Self::Error>;
    fn clear(&mut self);
}
