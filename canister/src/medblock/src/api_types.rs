use candid::{ CandidType, Deserialize, Principal };

use crate::{ emr::{patient::NIK, RecrodsDisplay}, internal_types::{ self, AsciiRecordsKey } };

#[derive(CandidType, Deserialize)]
pub struct RegisterProviderRequest {
    pub new_provider: Principal,
    pub encryted_display_name: String,
}

#[derive(CandidType, Deserialize)]
pub struct SuspendProviderRequest {
    pub provider: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
    pub emr_id: internal_types::Id,
}

#[derive(CandidType, Deserialize)]
pub struct CreateEmrForUserRequest {
    pub owner: NIK,
    pub emr_records: RecrodsDisplay,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRequest {
    pub emr_id: internal_types::Id,
    pub updated_emr_data: Vec<(AsciiRecordsKey, String)>,
}

#[derive(CandidType, Deserialize)]
pub struct DeleteEmrRequest {
    pub emr_id: internal_types::Id,
}

#[derive(CandidType, Deserialize)]
pub struct RegisterPatientRequest {
    pub owner: Principal,
    pub hashed_nik: NIK,
}

#[derive(CandidType, Deserialize)]
pub struct RebindPatientRequest {
    pub owner: Principal,
    pub hashed_nik: NIK,
}

#[derive(CandidType, Deserialize)]
pub struct UnbindPatientRequest {
    pub owner: Principal,
    pub hashed_nik: NIK,
}
