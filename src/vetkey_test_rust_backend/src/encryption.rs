use candid::CandidType;
use candid::Principal;
use serde::Deserialize;

use crate::types::Users;
use crate::types::VerifiedEmrManagerSet;

pub type CanisterId = Principal;

#[derive(CandidType, Deserialize, Clone)]
pub enum VetKDCurve {
    #[serde(rename = "bls12_381")]
    Bls12_381,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct VetKDKeyId {
    pub curve: VetKDCurve,
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDPublicKeyRequest {
    pub canister_id: Option<CanisterId>,
    pub derivation_path: Vec<Vec<u8>>,
    pub key_id: VetKDKeyId,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDPublicKeyReply {
    pub public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDEncryptedKeyRequest {
    pub public_key_derivation_path: Vec<Vec<u8>>,
    pub derivation_id: Vec<u8>,
    pub key_id: VetKDKeyId,
    pub encryption_public_key: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct VetKDEncryptedKeyReply {
    pub encrypted_key: Vec<u8>,
}

struct VetKdSystemApi;

pub type HexEncodedPublicKey = String;
pub type HexEncodedSecretKey = String;

impl VetKdSystemApi {
    const VETKD_CANISTER_ID: &'static str = "s55qq-oqaaa-aaaaa-aaakq-cai";
    const VETKD_PUBLIC_KEY_METHOD_SIGNATURE: &'static str = "vetkd_public_key";
    const VETKD_SECRET_KEY_METHOD_SIGNATURE: &'static str = "vetkd_encrypted_key";
    const STATIC_DERIVATION_PATH: &'static [u8] = b"symmetric_key";

    fn static_key_id() -> VetKDKeyId {
        VetKDKeyId {
            curve: VetKDCurve::Bls12_381,
            name: String::from("test_key_1"),
        }
    }

    fn id() -> Principal {
        Principal::from_text(Self::VETKD_CANISTER_ID).unwrap()
    }

    fn encode_to_string(bytes: Vec<u8>) -> String {
        hex::encode(bytes)
    }

    async fn vetkd_public_key() -> HexEncodedPublicKey {
        let request = VetKDPublicKeyRequest {
            canister_id: None,
            derivation_path: vec![Self::STATIC_DERIVATION_PATH.to_vec()],
            key_id: Self::static_key_id(),
        };

        let (response,): (VetKDPublicKeyReply,) = ic_cdk::api::call::call(
            Self::id(),
            Self::VETKD_PUBLIC_KEY_METHOD_SIGNATURE,
            (request,),
        )
        .await
        .expect("call to vetkd_public_key failed");

        Self::encode_to_string(response.public_key)
    }

    async fn vetkd_encrypted_key(transport_key_public_key: Vec<u8>) -> HexEncodedSecretKey {
        let derivation_id = Users::current_user().to_principal().as_slice().to_vec();

        let request = VetKDEncryptedKeyRequest {
            derivation_id,
            public_key_derivation_path: vec![Self::STATIC_DERIVATION_PATH.to_vec()],
            key_id: Self::static_key_id(),
            encryption_public_key: transport_key_public_key,
        };

        let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::api::call::call(
            Self::id(),
            Self::VETKD_SECRET_KEY_METHOD_SIGNATURE,
            (request,),
        )
        .await
        .expect("call to vetkd_encrypted_key failed");

        Self::encode_to_string(response.encrypted_key)
    }
}

pub struct EncryptionApi;

impl EncryptionApi {
    fn ensure_verified(registry: &VerifiedEmrManagerSet) -> Users {
        let user = Users::current_user();

        if !registry.is_verified(&user) {
            ic_cdk::trap("caller is not verified");
        }

        user
    }

    pub async fn symmetric_key_verification_key(
        verified_user_registry: &VerifiedEmrManagerSet,
    ) -> HexEncodedPublicKey {
        let user = Self::ensure_verified(verified_user_registry);
        VetKdSystemApi::vetkd_public_key().await
    }

    pub async fn encrypted_symmetric_key_for_caller(
        transport_key_public_key: Vec<u8>,
        verified_user_registry: &VerifiedEmrManagerSet,
    ) -> HexEncodedSecretKey {
        Self::ensure_verified(verified_user_registry);
        VetKdSystemApi::vetkd_encrypted_key(transport_key_public_key).await
    }
}
