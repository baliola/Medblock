use candid::Principal;
use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::{self, Relation},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
};

use crate::common;

// === Patient Registry Tests ===
#[test]
fn test_patient_registration() {
    let registries = common::prepare();
    let display = String::from("John Doe").to_ascii_lowercase();
    let address = String::from("1234 Elm St").to_ascii_lowercase();

    let nik = canister_common::common::H256::from_str(
        "3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709",
    )
    .unwrap();

    let arg = patient_registry::RegisterPatientRequest {
        nik: nik.to_string(),
    };

    let patient_principal = common::random_identity();

    registries
        .patient
        .register_patient(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Update,
            arg,
        )
        .unwrap();

    let arg = patient_registry::UpdateInitialPatientInfoRequest {
        info: patient_registry::V1 {
            name: display.clone(),
            martial_status: "married".to_string(),
            place_of_birth: "Jakarta".to_ascii_lowercase(),
            address,
            gender: "men".to_ascii_lowercase(),
            date_of_birth: "1990-01-01".to_string(),
            kyc_status: KycStatus::Pending,
            kyc_date: "2024-01-01".to_string(),
        },
    };

    registries
        .patient
        .update_initial_patient_info(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Update,
            arg,
        )
        .unwrap();

    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Query,
        )
        .unwrap();

    assert_eq!(result.nik, nik.to_string());
}

#[test]
fn test_patient_retrieval() {
    let registries = common::prepare();
    let display = String::from("John Doe").to_ascii_lowercase();
    let address = String::from("1234 Elm St").to_ascii_lowercase();

    let nik = canister_common::common::H256::from_str(
        "3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709",
    )
    .unwrap();

    let arg = patient_registry::RegisterPatientRequest {
        nik: nik.to_string(),
    };

    let patient_principal = common::random_identity();

    registries
        .patient
        .register_patient(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Update,
            arg,
        )
        .unwrap();

    let arg = patient_registry::UpdateInitialPatientInfoRequest {
        info: patient_registry::V1 {
            name: display.clone(),
            martial_status: "married".to_string(),
            place_of_birth: "Jakarta".to_ascii_lowercase(),
            address,
            gender: "men".to_ascii_lowercase(),
            date_of_birth: "1990-01-01".to_string(),
            kyc_status: KycStatus::Pending,
            kyc_date: "2024-01-01".to_string(),
        },
    };

    registries
        .patient
        .update_initial_patient_info(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Update,
            arg,
        )
        .unwrap();

    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Query,
        )
        .unwrap();

    assert_eq!(result.nik, nik.to_string());
}

#[test]
fn test_invalid_patient_registration() {
    let registries = common::prepare();
    let display = String::from("John Doe").to_ascii_lowercase();
    let address = String::from("1234 Elm St").to_ascii_lowercase();

    let nik = canister_common::common::H256::from_str(
        "3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709",
    )
    .unwrap();

    let arg = patient_registry::RegisterPatientRequest {
        nik: nik.to_string(),
    };

    let patient_principal = common::random_identity();

    registries
        .patient
        .register_patient(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Update,
            arg,
        )
        .unwrap();

    let arg = patient_registry::UpdateInitialPatientInfoRequest {
        info: patient_registry::V1 {
            name: display.clone(),
            martial_status: "married".to_string(),
            place_of_birth: "Jakarta".to_ascii_lowercase(),
            address,
            gender: "men".to_ascii_lowercase(),
            date_of_birth: "1990-01-01".to_string(),
            kyc_status: KycStatus::Pending,
            kyc_date: "2024-01-01".to_string(),
        },
    };

    registries
        .patient
        .update_initial_patient_info(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Update,
            arg,
        )
        .unwrap();

    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient_principal.clone(),
            patient_registry::pocket_ic_bindings::Call::Query,
        )
        .unwrap();

    assert_eq!(result.nik, nik.to_string());
}
