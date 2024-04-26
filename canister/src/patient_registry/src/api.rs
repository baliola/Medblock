use candid::{ CandidType, Principal };
use canister_common::{
    common::{ AsciiRecordsKey, EmrHeader, EmrId, ProviderId, UserId, H256 },
    from,
    stable::{ EncodingMarker, Stable },
};
use serde::Deserialize;

use crate::{
    consent::{ Consent, ConsentCode, SessionId },
    encryption::vetkd::{ HexEncodedPublicKey, HexEncodedSecretKey },
    log::Activity,
    registry::{ HeaderStatus, Patient, NIK, V1 },
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
    emrs: Vec<EmrHeaderWithStatus>,
}
from!(EmrListPatientResponse: Vec<EmrHeaderWithStatus> as value {
    emrs: value
});

#[derive(CandidType, Deserialize)]
pub struct EmrHeaderWithStatus {
    header: EmrHeader,
    status: HeaderStatus,
    hospital_name: AsciiRecordsKey<64>,
}

impl EmrHeaderWithStatus {
    pub fn new<E1: EncodingMarker, E2: EncodingMarker>(
        header: Stable<EmrHeader, E1>,
        status: Stable<HeaderStatus, E2>,
        hospital_name: AsciiRecordsKey<64>
    ) -> Self {
        Self { header: header.into_inner(), status: status.into_inner(), hospital_name }
    }
}

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
pub type UpdateRequest = IssueRequest;

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
    pub page: u8,
    pub limit: u8,
}

#[derive(CandidType, Deserialize)]
pub struct EmrListConsentResponse {
    emr: Vec<EmrHeaderWithStatus>,
    username: AsciiRecordsKey<64>,
}

impl EmrListConsentResponse {
    pub fn new(emr: Vec<EmrHeaderWithStatus>, username: AsciiRecordsKey<64>) -> Self {
        Self { emr, username }
    }
}

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
    pub name: AsciiRecordsKey<64>,
}

impl ClaimConsentResponse {
    pub fn new(session_id: SessionId, name: AsciiRecordsKey<64>) -> Self {
        Self { session_id, name }
    }
}

#[derive(CandidType, Deserialize)]
pub struct RevokeConsentRequest {
    pub codes: Vec<ConsentCode>,
}

#[derive(CandidType, Deserialize)]
pub struct FinishSessionRequest {
    pub session_id: SessionId,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRegistryRequest {
    pub principal: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct AuthorizedCallerRequest {
    pub caller: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateInitialPatientInfoRequest {
    pub info: V1,
}

#[derive(CandidType, Deserialize)]
pub struct GetPatientInfoResponse {
    pub patient: Patient,
    pub nik: NIK,
}

impl GetPatientInfoResponse {
    pub fn new(patient: Patient, nik: NIK) -> Self {
        Self { patient, nik }
    }
}

#[derive(CandidType, Deserialize)]
pub struct GetPatientInfoBySessionRequest {
    pub session_id: SessionId,
}

#[derive(CandidType, Deserialize)]
pub struct PatientListResponse {
    pub patients: Vec<PatientWithNikAndSession>,
}

from!(PatientListResponse: Vec<PatientWithNikAndSession> as value {
    patients: value
});

#[derive(CandidType, Deserialize)]
pub struct PatientWithNikAndSession {
    pub info: Patient,
    pub nik: NIK,
    pub session_id: SessionId,
}

impl PatientWithNikAndSession {
    pub fn new(patient: Patient, nik: NIK, session_id: SessionId) -> Self {
        Self { info: patient, nik, session_id }
    }
}

#[derive(CandidType, Deserialize)]
pub struct IsConsentClaimedRequest {
    pub code: ConsentCode,
}

#[derive(CandidType, Deserialize, Default)]
pub struct IsConsentClaimedResponse {
    pub claimed: bool,
    pub info: Option<Consent>,
}
from!(IsConsentClaimedResponse: bool as value {
    claimed: value
    info: None
});

#[derive(CandidType, Deserialize)]
pub struct ConsentListResponse {
    pub consents: Vec<Consent>,
}

from!(ConsentListResponse: Vec<Consent> as value {
    consents: value
});

#[derive(CandidType, Deserialize)]
pub struct SearchPatientRequest {
    pub nik: H256,
}

#[derive(CandidType, Deserialize)]
pub struct SearchPatientResponse {
    pub patient_info: PatientWithNikAndSession,
}

from!(SearchPatientResponse: PatientWithNikAndSession as value {
    patient_info: value
});

#[derive(CandidType, Deserialize)]
pub struct LogResponse {
    logs: Vec<Activity>,
}

impl LogResponse {
    pub fn new(logs: Vec<Activity>) -> Self {
        Self { logs }
    }
}

from!(LogResponse: Vec<Activity> as value {
    logs: value
});
