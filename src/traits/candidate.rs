use serde::de::DeserializeOwned;
use serde::Serialize;
use std::hash::Hash;

/// An object representing a snowball candidate
pub trait Candidate: Serialize + DeserializeOwned + Ord + Hash + Copy {}
