pub trait Node {
    type Error: crate::traits::error::Error;
    type Query: crate::traits::query::Query;
    type QueryResponse: crate::traits::query::QueryResponse;

    fn id(&self) -> u64;
    fn query(&mut self, query: Self::Query) -> Result<Self::QueryResponse, Self::Error>;
}

pub trait Network {
    type Node: Node;
    fn nodes(&self) -> Vec<Self::Node>;
}

pub trait NetworkQueryExecutor {
    type Node: Node;

    fn execute_query(
        &mut self,
        sample_nodes: Vec<Self::Node>,
        query: <<Self as NetworkQueryExecutor>::Node as Node>::Query,
    ) -> Result<
        Vec<<<Self as NetworkQueryExecutor>::Node as Node>::QueryResponse>,
        <<Self as NetworkQueryExecutor>::Node as Node>::Error,
    >;

    fn register_query_handler(
        &mut self,
        handler: fn(
            query: <<Self as NetworkQueryExecutor>::Node as Node>::Query,
            originating_node: &Self::Node,
        ) -> <<Self as NetworkQueryExecutor>::Node as Node>::QueryResponse,
    );
}
