use serde::de::DeserializeOwned;
use serde::Serialize;
use std::hash::Hash;

pub trait Candidate: Serialize + DeserializeOwned + Ord + Hash + Copy {}
