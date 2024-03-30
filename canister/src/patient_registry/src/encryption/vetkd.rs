#![allow(unused)]
use candid::CandidType;
use ic_principal::Principal;
use serde::Deserialize;
use tiny_keccak::Hasher;

use crate::registry::NIK;

// all the code inside vetkd abstraction block is subject to change following later audits results

// START ------------------------------ VETKD ABSTRACTION ------------------------------ START
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
        key_id: VetKDKeyId
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
    fn new(
        public_key_derivation_path: Vec<Vec<u8>>,
        derivation_id: Vec<u8>,
        key_id: VetKDKeyId,
        encryption_public_key: Vec<u8>
    ) -> Self {
        Self {
            public_key_derivation_path,
            derivation_id,
            key_id,
            encryption_public_key,
        }
    }
}

#[derive(CandidType, Deserialize)]
struct VetKDEncryptedKeyReply {
    encrypted_key: Vec<u8>,
}

struct VetKd;

pub type HexEncodedPublicKey = String;
pub type HexEncodedSecretKey = String;

impl VetKd {
    const VETKD_CANISTER_ID: &'static str = "s55qq-oqaaa-aaaaa-aaakq-cai";
    const VETKD_PUBLIC_KEY_METHOD_SIGNATURE: &'static str = "vetkd_public_key";
    const VETKD_SECRET_KEY_METHOD_SIGNATURE: &'static str = "vetkd_encrypted_key";

    /// for now, generate the key id by hashing the derivation path, e.g the user nik
    fn key_id_of(k: &impl AsRef<[u8]>) -> VetKDKeyId {
        let mut hash = tiny_keccak::Keccak::v512();
        let mut result = [0u8; 32];

        hash.update(k.as_ref());
        hash.finalize(&mut result);

        let result = hex::encode(result);
        VetKDKeyId::new(result)
    }


    fn id() -> Principal {
        Principal::from_text(Self::VETKD_CANISTER_ID).unwrap()
    }

    fn encode_to_string(bytes: Vec<u8>) -> String {
        hex::encode(bytes)
    }

    async fn vetkd_public_key(derivation_path: &impl AsRef<[u8]>) -> HexEncodedPublicKey {
        let key_id = Self::key_id_of(&derivation_path);

        let request = VetKDPublicKeyRequest::new(
            None,
            vec![derivation_path.as_ref().to_vec()],
            key_id
        );

        let (response,): (VetKDPublicKeyReply,) = ic_cdk::api::call
            ::call(Self::id(), Self::VETKD_PUBLIC_KEY_METHOD_SIGNATURE, (request,)).await
            .expect("call to vetkd_public_key failed");

        Self::encode_to_string(response.public_key)
    }

    async fn vetkd_encrypted_key(
        transport_key_public_key: Vec<u8>,
        derivation_path: &impl AsRef<[u8]>
    ) -> HexEncodedSecretKey {
        let derivation_id = ic_cdk::caller().as_slice().to_vec();
        let key_id = Self::key_id_of(&derivation_path);

        let request = VetKDEncryptedKeyRequest::new(
            vec![derivation_path.as_ref().to_vec()],
            derivation_id,
            key_id,
            transport_key_public_key
        );

        let (response,): (VetKDEncryptedKeyReply,) = ic_cdk::api::call
            ::call(Self::id(), Self::VETKD_SECRET_KEY_METHOD_SIGNATURE, (request,)).await
            .expect("call to vetkd_encrypted_key failed");

        Self::encode_to_string(response.encrypted_key)
    }
}

// END ------------------------------ VETKD ABSTRACTION ------------------------------ END

// START ------------------------------ MODULE PUBLIC API ------------------------------ START

/// EMR data encryption API. implementation are not secure and thus, not stable. subject to change following later audits results.
pub struct EncryptionApi;

impl EncryptionApi {
    // we aiming to expose this kind of api to canister public api

    /// retrieve verification key for decrypting EMR symmetric encryption key
    pub async fn verification_key_for(
        user: &NIK
    ) -> HexEncodedPublicKey {
        VetKd::vetkd_public_key(user).await
    }

    /// retrieve encryption key that will be used to encrypt and decrypt EMR with specified transport key
    pub async fn encrypted_symmetric_key_for_caller(
        transport_key_public_key: Vec<u8>,
        user: &NIK
    ) -> HexEncodedSecretKey {
        VetKd::vetkd_encrypted_key(transport_key_public_key, user).await
    }
}

// END ------------------------------ MODULE PUBLIC API ------------------------------ END
