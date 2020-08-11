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

pub trait NetworkQueryExecutor {
    type Query: crate::traits::query::Query;
    type QueryResponse: crate::traits::query::QueryResponse;
    type Node: Node;
    type Error: crate::traits::error::Error;

    fn execute_query(
        sample_nodes: Vec<Self::Node>,
        query: Self::Query,
    ) -> Result<Option<Self::QueryResponse>, Self::Error>;
}
