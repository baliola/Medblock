use candid::{CandidType, Principal};
use canister_common::{
    common::{AsciiRecordsKey, EmrHeader, EmrId, ProviderId, UserId, H256},
    from,
    stable::{EncodingMarker, Stable},
};
use serde::Deserialize;

use crate::{
    consent::{Consent, ConsentCode, SessionId},
    encryption::vetkd::{HexEncodedPublicKey, HexEncodedSecretKey},
    log::Activity,
    registry::{Group, GroupId, HeaderStatus, KycStatus, Patient, Relation, NIK, V1},
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
        hospital_name: AsciiRecordsKey<64>,
    ) -> Self {
        Self {
            header: header.into_inner(),
            status: status.into_inner(),
            hospital_name,
        }
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

/// Response type for admin-only patient list requests.
/// Contains a list of all patients with their basic information and NIK.
/// This is specifically for backoffice UI administrative purposes.
#[derive(CandidType, Deserialize)]
pub struct PatientListAdminResponse {
    pub patients: Vec<PatientWithNik>,
}

from!(PatientListAdminResponse: Vec<PatientWithNik> as value {
    patients: value
});

#[derive(CandidType, Deserialize)]
pub struct PatientWithNikAndSession {
    pub info: Patient,
    pub nik: NIK,
    pub session_id: SessionId,
}

/// Represents a patient record with their information and NIK.
/// Used for basic patient identification without session context.
#[derive(CandidType, Deserialize)]
pub struct PatientWithNik {
    pub info: Patient,
    pub nik: NIK,
}

impl PatientWithNik {
    pub fn new(patient: Patient, nik: NIK) -> Self {
        Self { info: patient, nik }
    }
}

impl PatientWithNikAndSession {
    pub fn new(patient: Patient, nik: NIK, session_id: SessionId) -> Self {
        Self {
            info: patient,
            nik,
            session_id,
        }
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
pub struct SearchPatientAdminResponse {
    pub patient_info: PatientWithNik,
}

impl SearchPatientAdminResponse {
    pub fn new(patient_info: PatientWithNik) -> Self {
        Self { patient_info }
    }
}

from!(SearchPatientAdminResponse: PatientWithNik as value {
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

#[derive(CandidType, Deserialize)]
pub struct UpdateKycStatusRequest {
    pub nik: H256,
    pub kyc_status: KycStatus,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateKycStatusResponse {
    pub patient: Patient,
}

impl UpdateKycStatusResponse {
    pub fn new(patient: Patient) -> Self {
        Self { patient }
    }
}

from!(UpdateKycStatusResponse: Patient as value {
    patient: value
});

#[derive(CandidType, Deserialize)]
pub struct BindAdminRequest {
    pub principal: Principal,
    pub nik: H256,
}

#[derive(CandidType, Deserialize)]
pub struct CreateGroupRequest {
    pub name: String,
}

#[derive(CandidType, Deserialize)]
pub struct CreateGroupResponse {
    pub group_id: GroupId,
}

from!(CreateGroupResponse: GroupId as value {
    group_id: value
});

#[derive(CandidType, Deserialize)]
pub struct AddGroupMemberRequest {
    pub group_id: GroupId,
    pub consent_code: String,
    pub relation: Relation,
}

#[derive(CandidType, Deserialize)]
pub struct LeaveGroupRequest {
    pub group_id: GroupId,
}

#[derive(CandidType, Deserialize)]
pub struct GetUserGroupsResponse {
    pub groups: Vec<Group>,
}

from!(GetUserGroupsResponse: Vec<Group> as value {
    groups: value
});

#[derive(CandidType, Deserialize)]
pub struct GrantGroupAccessRequest {
    pub group_id: GroupId,
    pub grantee_nik: NIK,
}

#[derive(CandidType, Deserialize)]
pub struct RevokeGroupAccessRequest {
    pub grantee_nik: NIK,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct ViewGroupMemberEmrInformationRequest {
    pub member_nik: String,
    pub group_id: GroupId,
    pub page: usize,
    pub limit: usize,
}

/// API response and request structs for group details functionality.
/// Returns the below information about the group:
/// - Group member count
/// - Group leader name
/// - Total pages for pagination
///
/// Returns the below information for each group member:
/// - NIK
/// - Name
/// - Gender
/// - Age
/// - Role
#[derive(CandidType, Deserialize)]
pub struct GroupDetail {
    pub nik: NIK,
    pub name: AsciiRecordsKey<64>,
    pub gender: AsciiRecordsKey<64>,
    pub age: u8,
    pub role: Relation,
}

#[derive(CandidType, Deserialize)]
pub struct GetGroupDetailsRequest {
    pub group_id: GroupId,
    pub page: u64,
    pub limit: u64,
}

#[derive(CandidType, Deserialize)]
pub struct GetGroupDetailsResponse {
    pub group_details: Vec<GroupDetail>,
    pub member_count: u64,
    pub group_name: AsciiRecordsKey<64>,
    pub leader_name: AsciiRecordsKey<64>,
    pub total_pages: u64,
}

impl GetGroupDetailsResponse {
    pub fn new(
        group_details: Vec<GroupDetail>,
        member_count: u64,
        group_name: AsciiRecordsKey<64>,
        leader_name: AsciiRecordsKey<64>,
        total_pages: u64,
    ) -> Self {
        Self {
            group_details,
            member_count,
            group_name,
            leader_name,
            total_pages,
        }
    }
}

// End of API response and request structs for group details functionality.
