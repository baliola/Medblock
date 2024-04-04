use candid::{ CandidType, Principal };
use canister_common::{
    common::{ EmrHeader, EmrId, ProviderId, UserId, H256 },
    from,
    stable::{ Candid, Stable },
};
use serde::Deserialize;

use crate::{
    consent::{ ConsentCode, SessionId },
    encryption::vetkd::{ HexEncodedPublicKey, HexEncodedSecretKey },
    registry::NIK,
};

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
    pub provider_id: ProviderId,
    pub emr_id: EmrId,
    pub registry_id: Principal,
}

impl ReadEmrByIdRequest {
    pub fn to_args(self, user_id: UserId) -> crate::declarations::emr_registry::ReadEmrByIdRequest {
        crate::declarations::emr_registry::ReadEmrByIdRequest {
            provider_id: self.provider_id.to_string(),
            emr_id: self.emr_id.to_string(),
            user_id: user_id.to_string(),
        }
    }
}

#[derive(CandidType, Deserialize)]
pub struct EmrListPatientRequest {
    pub limit: u8,
    pub page: u8,
}

#[derive(CandidType, Deserialize)]
pub struct EmrListPatientResponse {
    emrs: Vec<EmrHeader>,
}

impl From<Vec<Stable<EmrHeader>>> for EmrListPatientResponse {
    fn from(value: Vec<Stable<EmrHeader>>) -> Self {
        Self {
            emrs: value
                .into_iter()
                .map(|x| x.into_inner())
                .collect(),
        }
    }
}
from!(EmrListPatientResponse: Vec<EmrHeader> as value {
    emrs: value
});

#[derive(CandidType, Deserialize)]
pub struct RegisterPatientRequest {
    pub nik: H256,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterProviderResponse {
    // empty for now
}

#[derive(CandidType, Deserialize)]
pub struct PingResult {
    pub emr_registry_status: bool,
}

#[derive(CandidType, Deserialize)]
pub struct IssueRequest {
    pub header: EmrHeader,
}

#[derive(CandidType, Deserialize)]
pub struct CreateConsentRequest {
    pub allowed: Vec<EmrHeader>,
}
#[derive(CandidType, Deserialize)]
pub struct CreateConsentResponse {
    code: ConsentCode,
}

from!(CreateConsentResponse: ConsentCode as value {
    code: value
});

#[derive(CandidType, Deserialize)]
pub struct ReadEmrSessionRequest {
    pub session_id: SessionId,
    pub args: ReadEmrByIdRequest,
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrSessionResponse {}

#[derive(CandidType, Deserialize)]
pub struct EmrListConsentRequest {
    pub session_id: SessionId,
}

pub type EmrListConsentResponse = EmrListPatientResponse;

#[derive(CandidType, Deserialize)]
pub struct DeriveVerificationKeyRequest {
    pub session_id: SessionId,
}

#[derive(CandidType, Deserialize)]
pub struct DeriveVerificationKeyResponse {
    hex_encoded_public_key: String,
}

from!(DeriveVerificationKeyResponse: HexEncodedPublicKey as value {
    hex_encoded_public_key: value
});

#[derive(CandidType, Deserialize)]
pub struct DeriveSecretKeyRequest {
    pub session_id: SessionId,
    pub transport_key: Vec<u8>,
}

#[derive(CandidType, Deserialize)]
pub struct DeriveSecretKeyResponse {
    hex_encoded_encrypted_secret_key: String,
}

from!(DeriveSecretKeyResponse: HexEncodedSecretKey as value {
    hex_encoded_encrypted_secret_key: value
});

#[derive(CandidType, Deserialize)]
pub struct ClaimConsentRequest {
    pub code: ConsentCode,
}
#[derive(CandidType, Deserialize)]
pub struct ClaimConsentResponse {
    pub session_id: SessionId,
}

from!(ClaimConsentResponse: SessionId as value {
    session_id: value
});

pub type RevokeConsentRequest = ClaimConsentRequest;

#[derive(CandidType, Deserialize)]
pub struct FinishSessionRequest {
    pub session_id: SessionId,
}


#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRegistryRequest{
    pub principal: Principal
}

#[derive(CandidType, Deserialize)]
pub struct AuthorizedCallerRequest{
    pub caller: Principal,
}
