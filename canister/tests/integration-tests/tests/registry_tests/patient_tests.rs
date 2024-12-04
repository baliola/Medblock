use candid::Principal;
use integration_tests::declarations::{
    patient_registry::{self, pocket_ic_bindings::Call as PatientCall, KycStatus, Patient},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
};
use std::str::FromStr;

use crate::common;

// === Patient Registry Tests ===

// modified helper function to handle pocket_ic::UserError
fn register_test_patient(
    registries: &common::Registries,
    patient_principal: Principal,
    nik: canister_common::common::H256,
    patient_info: Option<patient_registry::V1>,
) -> Result<(), String> {
    let reg_arg = patient_registry::RegisterPatientRequest {
        nik: nik.to_string(),
    };

    registries
        .patient
        .register_patient(
            &registries.ic,
            patient_principal.clone(),
            PatientCall::Update,
            reg_arg,
        )
        .map_err(|e| e.to_string())?;

    if let Some(info) = patient_info {
        let update_arg = patient_registry::UpdateInitialPatientInfoRequest { info };
        registries
            .patient
            .update_initial_patient_info(
                &registries.ic,
                patient_principal.clone(),
                PatientCall::Update,
                update_arg,
            )
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[test]
fn test_patient_registration() {
    let registries = common::prepare();
    let patient_principal = common::random_identity();
    let nik = canister_common::common::H256::from([1u8; 32]);

    let patient_info = patient_registry::V1 {
        name: "john doe".to_string(),
        martial_status: "married".to_string(),
        place_of_birth: "jakarta".to_string(),
        address: "1234 elm st".to_string(),
        gender: "men".to_string(),
        date_of_birth: "1990-01-01".to_string(),
        kyc_status: KycStatus::Pending,
        kyc_date: "2024-01-01".to_string(),
    };

    register_test_patient(
        &registries,
        patient_principal.clone(),
        nik.clone(),
        Some(patient_info),
    )
    .expect("Failed to register patient");

    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient_principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(result.nik, nik.to_string());
}

#[test]
fn test_patient_retrieval() {
    let registries = common::prepare();
    let patient_principal = common::random_identity();
    let nik = canister_common::common::H256::from([1u8; 32]);

    let patient_info = patient_registry::V1 {
        name: "john doe".to_string(),
        martial_status: "married".to_string(),
        place_of_birth: "jakarta".to_string(),
        address: "1234 elm st".to_string(),
        gender: "men".to_string(),
        date_of_birth: "1990-01-01".to_string(),
        kyc_status: KycStatus::Pending,
        kyc_date: "2024-01-01".to_string(),
    };

    register_test_patient(
        &registries,
        patient_principal.clone(),
        nik.clone(),
        Some(patient_info),
    )
    .expect("Failed to register patient");

    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient_principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(result.nik, nik.to_string());
}

#[test]
#[should_panic(expected = "Error: \"only admin or controller can call this method\"")]
fn test_admin_patient_list() {
    let registries = common::prepare();
    let admin_principal = common::random_identity();

    // register multiple test patients
    let patient1_principal = common::random_identity();
    let patient2_principal = common::random_identity();
    let nik1 = canister_common::common::H256::from([1u8; 32]);
    let nik2 = canister_common::common::H256::from([2u8; 32]);

    let patient1_info = patient_registry::V1 {
        name: "patient one".to_string(),
        martial_status: "single".to_string(),
        place_of_birth: "jakarta".to_string(),
        address: "address 1".to_string(),
        gender: "f".to_string(),
        date_of_birth: "1990-01-01".to_string(),
        kyc_status: KycStatus::Pending,
        kyc_date: "2024-01-01".to_string(),
    };

    let patient2_info = patient_registry::V1 {
        name: "patient two".to_string(),
        martial_status: "married".to_string(),
        place_of_birth: "surabaya".to_string(),
        address: "address 2".to_string(),
        gender: "m".to_string(),
        date_of_birth: "1995-01-01".to_string(),
        kyc_status: KycStatus::Approved,
        kyc_date: "2024-01-02".to_string(),
    };

    register_test_patient(&registries, patient1_principal, nik1, Some(patient1_info))
        .expect("Failed to register patient 1");
    register_test_patient(&registries, patient2_principal, nik2, Some(patient2_info))
        .expect("Failed to register patient 2");

    // this should panic due to unauthorized admin
    registries
        .patient
        .get_patient_list_admin(&registries.ic, admin_principal, PatientCall::Query)
        .unwrap();
}

#[test]
fn test_search_patient_admin() {
    let registries = common::prepare();
    let admin_principal = registries.controller;
    let patient_principal = common::random_identity();
    let nik = canister_common::common::H256::from([1u8; 32]);

    let patient_info = patient_registry::V1 {
        name: "john doe".to_string(),
        martial_status: "married".to_string(),
        place_of_birth: "jakarta".to_string(),
        address: "1234 elm st".to_string(),
        gender: "men".to_string(),
        date_of_birth: "1990-01-01".to_string(),
        kyc_status: KycStatus::Pending,
        kyc_date: "2024-01-01".to_string(),
    };

    // register a test patient first
    register_test_patient(
        &registries,
        patient_principal.clone(),
        nik.clone(),
        Some(patient_info),
    )
    .expect("Failed to register patient");

    // search for the patient using admin privileges
    let search_req = patient_registry::SearchPatientRequest {
        nik: nik.to_string(),
    };

    let result = registries
        .patient
        .search_patient_admin(
            &registries.ic,
            admin_principal,
            PatientCall::Query,
            search_req,
        )
        .unwrap();

    // verify the search results
    assert_eq!(result.patient_info.nik, nik.to_string());

    // match on the Patient enum to access V1 fields
    match result.patient_info.info {
        patient_registry::Patient::V1(v1) => {
            assert_eq!(v1.name, "john doe");
            assert!(matches!(v1.kyc_status, KycStatus::Pending));
        }
    }
}

#[test]
#[should_panic(expected = "Error: \"only admin or controller can call this method\"")]
fn test_search_patient_admin_unauthorized() {
    let registries = common::prepare();
    let unauthorized_principal = common::random_identity();
    let nik = canister_common::common::H256::from([1u8; 32]);

    let search_req = patient_registry::SearchPatientRequest {
        nik: nik.to_string(),
    };

    // this should panic due to unauthorized access
    registries
        .patient
        .search_patient_admin(
            &registries.ic,
            unauthorized_principal,
            PatientCall::Query,
            search_req,
        )
        .unwrap();
}

#[test]
#[should_panic(expected = "trapped explicitly: NIK is already registered")]
fn test_nik_duplication() {
    let registries = common::prepare();
    let patient1_principal = common::random_identity();
    let patient2_principal = common::random_identity();
    let nik = canister_common::common::H256::from([1u8; 32]);

    // register first patient
    registries
        .patient
        .register_patient(
            &registries.ic,
            patient1_principal.clone(),
            PatientCall::Update,
            patient_registry::RegisterPatientRequest {
                nik: nik.to_string(),
            },
        )
        .unwrap();

    // try to register second patient with same NIK - this should panic
    registries
        .patient
        .register_patient(
            &registries.ic,
            patient2_principal.clone(),
            PatientCall::Update,
            patient_registry::RegisterPatientRequest {
                nik: nik.to_string(),
            },
        )
        .unwrap();
}
