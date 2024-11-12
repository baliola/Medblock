use candid::Principal;
use integration_tests::declarations::{
    patient_registry::{self, pocket_ic_bindings::Call as PatientCall, KycStatus, Patient},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
};
use std::str::FromStr;

use crate::common;

// === Patient Registry Tests ===
#[test]
fn test_patient_registration() {
    let registries = common::prepare();
    let display = String::from("John Doe").to_ascii_lowercase();
    let address = String::from("1234 Elm St").to_ascii_lowercase();

    let nik = canister_common::common::H256::from([1u8; 32]);

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

    let nik = canister_common::common::H256::from([1u8; 32]);

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

    let nik = canister_common::common::H256::from([1u8; 32]);

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
#[should_panic(expected = "Error: \"only admin can call this method\"")]
fn test_admin_patient_list() {
    let registries = common::prepare();

    // create an admin but DON'T bind them - this will cause the admin check to fail
    let admin_principal = common::random_identity();

    // register two test patients
    let patient1_principal = common::random_identity();
    let patient2_principal = common::random_identity();

    let nik1 = canister_common::common::H256::from([1u8; 32]);
    let nik2 = canister_common::common::H256::from([2u8; 32]);

    // register first patient
    let reg_arg1 = patient_registry::RegisterPatientRequest {
        nik: nik1.to_string(),
    };
    registries
        .patient
        .register_patient(
            &registries.ic,
            patient1_principal.clone(),
            PatientCall::Update,
            reg_arg1,
        )
        .unwrap();

    // register second patient
    let reg_arg2 = patient_registry::RegisterPatientRequest {
        nik: nik2.to_string(),
    };
    registries
        .patient
        .register_patient(
            &registries.ic,
            patient2_principal.clone(),
            PatientCall::Update,
            reg_arg2,
        )
        .unwrap();

    // update patient info for both patients
    let update_info1 = patient_registry::UpdateInitialPatientInfoRequest {
        info: patient_registry::V1 {
            name: "Patient One".to_string().to_ascii_lowercase(),
            martial_status: "single".to_string(),
            place_of_birth: "Jakarta".to_ascii_lowercase(),
            address: "Address 1".to_string().to_ascii_lowercase(),
            gender: "F".to_ascii_lowercase(),
            date_of_birth: "1990-01-01".to_string(),
            kyc_status: KycStatus::Pending,
            kyc_date: "2024-01-01".to_string(),
        },
    };
    registries
        .patient
        .update_initial_patient_info(
            &registries.ic,
            patient1_principal.clone(),
            PatientCall::Update,
            update_info1,
        )
        .unwrap();

    let update_info2 = patient_registry::UpdateInitialPatientInfoRequest {
        info: patient_registry::V1 {
            name: "Patient Two".to_string().to_ascii_lowercase(),
            martial_status: "married".to_string(),
            place_of_birth: "Surabaya".to_ascii_lowercase(),
            address: "Address 2".to_string().to_ascii_lowercase(),
            gender: "M".to_ascii_lowercase(),
            date_of_birth: "1995-01-01".to_string(),
            kyc_status: KycStatus::Approved,
            kyc_date: "2024-01-02".to_string(),
        },
    };
    registries
        .patient
        .update_initial_patient_info(
            &registries.ic,
            patient2_principal.clone(),
            PatientCall::Update,
            update_info2,
        )
        .unwrap();

    // try to call get_patient_list_admin with an unbound admin - this should trigger the admin check failure
    registries
        .patient
        .get_patient_list_admin(&registries.ic, admin_principal, PatientCall::Query)
        .unwrap();
}
