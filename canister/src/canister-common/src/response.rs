use candid::CandidType;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum ResponseStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error(String),
}

#[derive(CandidType, Deserialize, Serialize, Debug, Clone)]
pub struct StandardResponse<T> 
where 
    T: Serialize
{
    pub status: ResponseStatus,
    pub data: Option<T>,
    pub message: Option<String>,
    pub timestamp: u64,
    pub tx_hash: String,
}

impl<T> StandardResponse<T> 
where 
    T: Serialize
{
    pub fn success(data: T) -> Self {
        let tx_hash = Self::generate_hash(&data);
        Self {
            status: ResponseStatus::Success,
            data: Some(data),
            message: None,
            timestamp: ic_cdk::api::time(),
            tx_hash,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        let error_msg = message.into();
        let tx_hash = Self::generate_hash(&error_msg);
        Self {
            status: ResponseStatus::Error(error_msg.clone()),
            data: None,
            message: Some(error_msg),
            timestamp: ic_cdk::api::time(),
            tx_hash,
        }
    }

    fn generate_hash<D: Serialize>(data: &D) -> String {
        let mut hasher = Sha256::new();
        let serialized = serde_cbor::ser::to_vec_packed(&data).unwrap();
        hasher.update(&serialized);
        format!("{:x}", hasher.finalize())
    }
}

// Type alias for responses without data
pub type VoidResponse = StandardResponse<()>; 