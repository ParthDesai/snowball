use serde::{Serialize};
use serde::de::DeserializeOwned;
use crate::traits::signable::Signable;

pub trait Query: Signable + Serialize + DeserializeOwned {
    type Key: Serialize + DeserializeOwned + Ord;
    type Value: Serialize + DeserializeOwned;

    fn get(&self, key: Self::Key) -> Self::Value;
}

pub trait QueryResponse: Signable + Serialize + DeserializeOwned {

}
