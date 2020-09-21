use crate::traits::signable::Signable;
use serde::de::DeserializeOwned;
use serde::Serialize;

/// Context of the query, used to store additional data
/// that recipient node can use to figure out query's response
pub trait QueryContext: Serialize + DeserializeOwned {
    type Key: Serialize + DeserializeOwned + Ord;
    type Value: Serialize + DeserializeOwned;

    /// Get value for a key
    fn get(&self, key: Self::Key) -> Option<&Self::Value>;
}

/// Query object that is sent on wire
/// Contains query context, location of the query and
/// sending node's candidate.
pub trait Query: Signable + Serialize + DeserializeOwned {
    type Context: QueryContext;
    type Location: Serialize + DeserializeOwned + PartialEq;
    type Candidate: crate::traits::candidate::Candidate;

    fn candidate(&self) -> &Self::Candidate;
    fn location(&self) -> &Self::Location;
    fn context(&self) -> &Self::Context;
}

/// Response for a particular query.
/// Contains responded node's preferred candidate
pub trait QueryResponse: Signable + Serialize + DeserializeOwned {
    type Candidate: crate::traits::candidate::Candidate;

    fn preferred_candidate(&self) -> &Self::Candidate;
}

/// Helper object used to build a query object
pub trait QueryBuilder {
    type Context: QueryContext;
    type Location: Serialize + DeserializeOwned + PartialEq;
    type Candidate: crate::traits::candidate::Candidate;
    type Query: Query<
        Context = Self::Context,
        Location = Self::Location,
        Candidate = Self::Candidate,
    >;
    fn build_query(
        &mut self,
        candidate: &Self::Candidate,
        location: &Self::Location,
        context: &Self::Context,
    ) -> Self::Query;
}
