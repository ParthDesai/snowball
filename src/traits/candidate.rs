use serde::de::DeserializeOwned;
use serde::Serialize;

pub trait Candidate: Serialize + DeserializeOwned + Eq {
    fn id() -> String;
}
