pub trait Node {
    type Error: crate::traits::error::Error;

    fn id(&self) -> u64;
    fn query(&mut self) -> Result<(), Self::Error>;
}

pub trait Network {
    type Node: Node;
    fn nodes(&self) -> Vec<Self::Node>;
}
