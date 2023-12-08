use candid::CandidType;
use candid::Principal;
use serde::Deserialize;

type CanisterId = Principal;

#[derive(CandidType, Deserialize, Clone)]
enum VetKDCurve {
    #[serde(rename = "bls12_381")]
    Bls12_381,
}

impl Default for VetKDCurve {
    fn default() -> Self {
        Self::Bls12_381
    }
}

#[derive(CandidType, Deserialize, Clone)]
struct VetKDKeyId {
    curve: VetKDCurve,
    name: String,
}

impl VetKDKeyId {
    fn new(name: String) -> Self {
        Self {
            curve: VetKDCurve::default(),
            name,
        }
    }
}

#[derive(CandidType, Deserialize)]
struct VetKDPublicKeyRequest {
    canister_id: Option<CanisterId>,
    derivation_path: Vec<Vec<u8>>,
    key_id: VetKDKeyId,
}

impl VetKDPublicKeyRequest {
    fn new(
        canister_id: Option<CanisterId>,
        derivation_path: Vec<Vec<u8>>,
        key_id: VetKDKeyId,
    ) -> Self {
        Self {
            canister_id,
            derivation_path,
            key_id,
        }
    }
}

#[derive(CandidType, Deserialize)]
struct VetKDPublicKeyReply {
    public_key: Vec<u8>,
}

impl VetKDPublicKeyReply {
    fn new(public_key: Vec<u8>) -> Self {
        Self { public_key }
    }
}

#[derive(CandidType, Deserialize)]
struct VetKDEncryptedKeyRequest {
    public_key_derivation_path: Vec<Vec<u8>>,
    derivation_id: Vec<u8>,
    key_id: VetKDKeyId,
    encryption_public_key: Vec<u8>,
}

impl VetKDEncryptedKeyRequest {
    fn new(public_key_derivation_path: Vec<Vec<u8>>, derivation_id: Vec<u8>, key_id: VetKDKeyId, encryption_public_key: Vec<u8>) -> Self { Self { public_key_derivation_path, derivation_id, key_id, encryption_public_key } }
}

#[derive(CandidType, Deserialize)]
struct VetKDEncryptedKeyReply {
    encrypted_key: Vec<u8>,
}

struct VetKdSystemApi;

pub type HexEncodedPublicKey = String;
pub type HexEncodedSecretKey = String;

impl VetKdSystemApi {
    const VETKD_CANISTER_ID: &'static str = "s55qq-oqaaa-aaaaa-aaakq-cai";
    const VETKD_PUBLIC_KEY_METHOD_SIGNATURE: &'static str = "vetkd_public_key";
    const VETKD_SECRET_KEY_METHOD_SIGNATURE: &'static str = "vetkd_encrypted_key";
    const STATIC_DERIVATION_PATH: &'static [u8] = b"symmetric_key";
    const STATIC_KEY_ID: &'static str = "symmetric_key";

    fn static_key_id() -> VetKDKeyId {
        VetKDKeyId::new(String::from(Self::STATIC_KEY_ID))
    }

    fn id() -> Principal {
        Principal::from_text(Self::VETKD_CANISTER_ID).unwrap()
    }

    fn encode_to_string(bytes: Vec<u8>) -> String {
        hex::encode(bytes)
    }

    async fn vetkd_public_key() -> HexEncodedPublicKey {
        let request = VetKDEncryptedKeyRequest::
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
        let derivation_id = ic_cdk::caller().as_slice().to_vec();

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
    pub async fn symmetric_key_verification_key() -> HexEncodedPublicKey {
        VetKdSystemApi::vetkd_public_key().await
    }

    pub async fn encrypted_symmetric_key_for_caller(
        transport_key_public_key: Vec<u8>,
    ) -> HexEncodedSecretKey {
        VetKdSystemApi::vetkd_encrypted_key(transport_key_public_key).await
    }
}
