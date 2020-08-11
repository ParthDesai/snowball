use serde::{Serialize};
use serde::de::DeserializeOwned;

pub trait Query: Serialize + DeserializeOwned {
    type Key: Serialize + DeserializeOwned + Ord;
    type Value: Serialize + DeserializeOwned;

    fn get(&self, key: Self::Key) -> Self::Value;
    fn sign_bytes() -> Vec<u8>;
}

pub trait QueryResponse: Serialize + DeserializeOwned {

}
