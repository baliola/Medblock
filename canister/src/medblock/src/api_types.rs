use candid::{ CandidType, Deserialize, Principal };

use crate::{ emr::{ patient::NIK, RecordsDisplay }, internal_types::{ self, AsciiRecordsKey } };

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
pub struct UnSuspendProviderRequest {
    pub provider: Principal,
}


#[derive(CandidType, Deserialize)]
pub struct IsProviderSuspendRequest {
    pub provider: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct ReadEmrByIdRequest {
    pub emr_id: internal_types::Id,
}

#[derive(CandidType, Deserialize)]
pub struct CreateEmrForUserRequest {
    pub owner: NIK,
    pub emr_records: RecordsDisplay,
}

#[derive(CandidType, Deserialize)]
pub struct UpdateEmrRequest {
    pub emr_id: internal_types::Id,
    pub updated_emr_data: Vec<(AsciiRecordsKey, String)>,
}

#[derive(CandidType, Deserialize)]
pub struct EmrListProviderRequest {
    pub anchor: u64,
    pub max: u8,
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
pub struct RevokePatientAccessRequest {
    pub owner: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct IsValidPatientRequest {
    pub principal: Principal,
}

#[derive(CandidType, Deserialize)]
pub struct IsValidProviderRequest {
    pub principal: Principal,
}
