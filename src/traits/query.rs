pub trait Query<'a> {
    type Context: QueryContext<'a>;
    fn context(&self) -> &Self::Context;
}

pub trait QueryContext<'a> {
    type Error: crate::traits::error::Error<'a>;

    fn get(&self, key: &'a str) -> &'a str;
    fn set(&mut self, key: &'a str, value: &'a str) -> Result<(), Self::Error>;
    fn clear(&mut self);
}
