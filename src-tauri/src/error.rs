use serde::{Serialize, Deserialize};
use thiserror::Error;

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum MyError {
    #[error("KubeError: {0}")]
    KubeError(String)
}

impl From<kube::Error> for MyError {
    fn from(value: kube::Error) -> Self {
        MyError::KubeError(value.to_string())
    }
    
}