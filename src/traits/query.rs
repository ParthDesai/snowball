use crate::traits::signable::Signable;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait QueryContext: Serialize + DeserializeOwned {
    type Key: Serialize + DeserializeOwned + Ord;
    type Value: Serialize + DeserializeOwned;

    fn get(&self, key: Self::Key) -> &Self::Value;
}

pub trait Query: Signable + Serialize + DeserializeOwned {
    type Context: QueryContext;
    type Location: Serialize + DeserializeOwned + PartialEq;
    type Candidate: crate::traits::candidate::Candidate;

    fn candidate(&self) -> &Self::Candidate;
    fn location(&self) -> &Self::Location;
    fn context(&self) -> &Self::Context;
}

pub trait QueryResponse: Signable + Serialize + DeserializeOwned {
    type Candidate: crate::traits::candidate::Candidate;

    fn preferred_candidate(&self) -> &Self::Candidate;
}

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
        candidate: Self::Candidate,
        location: Self::Location,
        context: Self::Context,
    ) -> Self::Query;
}
