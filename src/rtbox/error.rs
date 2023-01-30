use async_trait::async_trait;
use std::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RtBoxError {
    pub command: Option<String>,
    pub message: Option<String>,
    pub root_cause: Option<String>,
}

#[async_trait]
impl fmt::Display for RtBoxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}
