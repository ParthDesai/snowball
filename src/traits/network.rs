pub trait Node {
    type Error: crate::traits::error::Error;
    type Query: crate::traits::query::Query;

    fn id(&self) -> u64;
    fn query(&mut self, query: Self::Query) -> Result<(), Self::Error>;
}

pub trait Network {
    type Node: Node;
    fn nodes(&self) -> Vec<Self::Node>;
}
