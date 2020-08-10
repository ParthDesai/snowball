pub trait Node<'a> {
    type Error: crate::traits::error::Error<'a>;

    fn id(&self) -> u64;
    fn query(&mut self) -> Result<(), Self::Error>;
}

pub trait Network<'a> {
    type Node: Node<'a>;
    fn nodes(&self) -> &[Self::Node];
}