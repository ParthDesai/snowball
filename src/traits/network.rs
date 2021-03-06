use std::net::IpAddr;

/// Remote node discovered by this node.
pub trait Node {
    type Error: crate::traits::error::Error;
    type Query: crate::traits::query::Query;
    type QueryResponse: crate::traits::query::QueryResponse;
    type NodeId: Eq;

    /// Id of the node
    fn id(&self) -> Self::NodeId;

    /// Fire a query to the node
    fn query(&mut self, query: Self::Query) -> Result<Self::QueryResponse, Self::Error>;

    /// Returns IP address of the node
    fn ip(&self) -> IpAddr;
}

/// Object that provides view of the network
pub trait Network {
    type Node: Node;
    /// Returns list of node ids we have discovered
    /// this list can be constantly updated as nodes comes up or shuts down.
    fn node_ids(&self) -> Vec<<Self::Node as Node>::NodeId>;

    /// Returns node object against the id passed
    fn node(&self, id: <Self::Node as Node>::NodeId) -> Option<&Self::Node>;

    /// Executes our query on all sample nodes and returns array of response for the same.
    fn execute_query(
        &mut self,
        sample_nodes: Vec<<Self::Node as Node>::NodeId>,
        query: <<Self as Network>::Node as Node>::Query,
    ) -> Result<
        Vec<<<Self as Network>::Node as Node>::QueryResponse>,
        <<Self as Network>::Node as Node>::Error,
    >;

    /// Registers query filter that decides whether or not to respond incoming query.
    /// Can be used to blacklist nodes which are sending too many queries in short amount of time.
    fn register_query_filter(
        &mut self,
        filter: fn(
            query: <<Self as Network>::Node as Node>::Query,
            originating_node: &Self::Node,
        ) -> bool,
    ) -> Result<(), <<Self as Network>::Node as Node>::Error>;

    /// Informs network query executor that our node's preferred candidate is updated.
    /// This method *must* not trigger any network operation and cannot fail.
    fn update_preferred_candidate(
        &mut self,
        candidate: <<<Self as Network>::Node as Node>::QueryResponse as crate::traits::query::QueryResponse>::Candidate,
    );
}
