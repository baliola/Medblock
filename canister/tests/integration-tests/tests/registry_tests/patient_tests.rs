use crate::common;
use integration_tests::declarations::{
    patient_registry::{self, pocket_ic_bindings::Call as PatientCall, KycStatus},
    provider_registry::{self, pocket_ic_bindings::Call as ProviderCall, GetProviderListRequest},
};

#[test]
fn test_patient_registration() {
    let (registries, patient, _) = common::Scenario::one_admin_one_patient();

    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(result.nik, patient.nik.to_string());
}

#[test]
fn test_patient_retrieval() {
    let (registries, patient, _) = common::Scenario::one_admin_one_patient();

    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(result.nik, patient.nik.to_string());
}

#[test]
#[should_panic(
    expected = "Error: \"[PATIENT_REGISTRY_LIB] Only admin or controller can call this method. Are you registered as Patient Registry Admin or Controller?\""
)]
fn test_admin_patient_list() {
    let (registries, _, _) = common::Scenario::one_admin_one_patient();
    let unauthorized_admin = common::random_identity();

    // this should panic due to unauthorized admin
    registries
        .patient
        .get_patient_list_admin(&registries.ic, unauthorized_admin, PatientCall::Query)
        .unwrap();
}

/// TEST LISTING PATIENTS
///
/// *PREREQUISITES*:
/// - One registered provider
/// - Two registered patients with EMRs from the provider above
///
/// *FLOW BEING TESTED*:
/// 1. List patients using admin privileges
/// 2. Verify the list includes the two patients
#[test]
fn test_list_patients() {
    let (registries, provider, patient1, patient2) =
        common::Scenario::one_provider_two_patient_with_emrs();

    // establish sessions by claiming consents for both patients
    common::Scenario::claim_consent_for_provider(&registries, &provider, &patient1);
    common::Scenario::claim_consent_for_provider(&registries, &provider, &patient2);

    // now try to list patients with the provider's principal
    let result = registries
        .patient
        .patient_list(&registries.ic, provider.0.clone(), PatientCall::Query)
        .unwrap();

    assert_eq!(result.patients.len(), 2);
}

/// TEST LISTING PATIENTS AS ADMIN
///
/// *PREREQUISITES*:
/// - One registered admin
/// - Two registered patients with EMRs from the provider above
///
/// *FLOW BEING TESTED*:
/// 1. List patients using admin privileges
/// 2. Verify the list includes the two patients
#[test]
fn test_list_patients_admin() {
    let (registries, provider, admin_principal, patient2) =
        common::Scenario::one_provider_two_patient_with_emrs();

    // establish sessions by claiming consents for both patients
    common::Scenario::claim_consent_for_provider(&registries, &provider, &admin_principal);
    common::Scenario::claim_consent_for_provider(&registries, &provider, &patient2);

    // bind the admin_principal as admin
    let args = patient_registry::BindAdminRequest {
        nik: admin_principal.nik.to_string(),
        principal: admin_principal.principal,
    };

    registries
        .patient
        .bind_admin(
            &registries.ic,
            registries.controller.clone(),
            PatientCall::Update,
            args,
        )
        .unwrap();

    // now try to list patients with admin privileges
    let result = registries
        .patient
        .get_patient_list_admin(
            &registries.ic,
            admin_principal.principal,
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(result.patients.len(), 2);
}

#[test]
fn test_search_patient_admin() {
    let (registries, patient, admin_principal) = common::Scenario::one_admin_one_patient();

    // search for the patient using admin privileges
    let search_req = patient_registry::SearchPatientRequest {
        nik: patient.nik.to_string(),
        _type: None,
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
    assert_eq!(result.patient_info.nik, patient.nik.to_string());

    // match on the Patient enum to access V1 fields
    match result.patient_info.info {
        patient_registry::Patient::V1(v1) => {
            assert!(matches!(v1.kyc_status, KycStatus::Pending));
        }
    }
}

#[test]
#[should_panic(
    expected = "Error: \"[PATIENT_REGISTRY_LIB] Only admin or controller can call this method. Are you registered as Patient Registry Admin or Controller?\""
)]
fn test_search_patient_admin_unauthorized() {
    let (registries, patient, _) = common::Scenario::one_admin_one_patient();
    let unauthorized_principal = common::random_identity();

    let search_req = patient_registry::SearchPatientRequest {
        nik: patient.nik.to_string(),
        _type: None,
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
fn test_nik_duplication() {
    let (registries, existing_patient, _) = common::Scenario::one_admin_one_patient();
    let new_patient_principal = common::random_identity();

    // attempt to register a new patient with the same NIK
    let reg_arg = patient_registry::RegisterPatientRequest {
        nik: existing_patient.nik.to_string(),
    };

    let result = registries
        .patient
        .register_patient(
            &registries.ic,
            new_patient_principal,
            PatientCall::Update,
            reg_arg,
        )
        .unwrap();

    match result.result {
        patient_registry::RegisterPatientStatus::Error(err) => {
            assert_eq!(err, "[REGISTER_PATIENT] This NIK is already registered to another user. Each NIK can only be registered to one user account. If you believe this is an error, please contact support.");
        }
        _ => panic!("Expected error but got success"),
    }
}

#[test]
fn test_multiple_nik_registration() {
    let registries = common::prepare();

    // Create first patient using the helper
    let patient = common::Scenario::create_patient(&registries);

    // Generate a new NIK without registering it
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut nik_bytes = [0u8; 32];
    nik_bytes[..16].copy_from_slice(&timestamp.to_be_bytes());
    let second_nik = canister_common::common::H256::from(nik_bytes);

    // Try to register the new NIK with the first patient's principal
    let reg_arg = patient_registry::RegisterPatientRequest {
        nik: second_nik.to_string(),
    };

    let result = registries
        .patient
        .register_patient(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Update,
            reg_arg,
        )
        .unwrap();

    match result.result {
        patient_registry::RegisterPatientStatus::Error(err) => {
            assert_eq!(err, "[REGISTER_PATIENT] You already have a registered NIK associated with your account. Each user can only register one NIK. Please contact support if you need to change your registered NIK.");
        }
        _ => panic!("Expected error but got success"),
    }
}

#[test]
fn test_reregister_verified_kyc() {
    let (registries, patient, admin_principal) = common::Scenario::one_admin_one_patient();

    // admin approves KYC
    let approve_kyc_req = patient_registry::UpdateKycStatusRequest {
        nik: patient.nik.to_string(),
        kyc_status: KycStatus::Approved,
    };

    registries
        .patient
        .update_kyc_status(
            &registries.ic,
            admin_principal,
            PatientCall::Update,
            approve_kyc_req,
        )
        .expect("Failed to approve KYC");

    // attempt to reregister with verified KYC
    let resubmit_reg = patient_registry::RegisterPatientRequest {
        nik: patient.nik.to_string(),
    };

    let result = registries
        .patient
        .register_patient(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Update,
            resubmit_reg,
        )
        .unwrap();

    match result.result {
        patient_registry::RegisterPatientStatus::Error(err) => {
            assert_eq!(err, "[REGISTER_PATIENT] This NIK is already registered and verified. Re-registration is only allowed for denied KYC applications. Please contact support if you need assistance.");
        }
        _ => panic!("Expected error but got success"),
    }
}

#[test]
fn test_kyc_resubmission() {
    let (registries, patient, admin_principal) = common::Scenario::one_admin_one_patient();

    // admin denies KYC
    let deny_kyc_req = patient_registry::UpdateKycStatusRequest {
        nik: patient.nik.to_string(),
        kyc_status: KycStatus::Denied,
    };

    registries
        .patient
        .update_kyc_status(
            &registries.ic,
            admin_principal,
            PatientCall::Update,
            deny_kyc_req,
        )
        .expect("Failed to deny KYC");

    // patient attempts to reregister - this should work since KYC was denied
    let resubmit_reg = patient_registry::RegisterPatientRequest {
        nik: patient.nik.to_string(),
    };

    registries
        .patient
        .register_patient(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Update,
            resubmit_reg,
        )
        .expect("Failed to reregister patient");

    // update patient info after reregistration
    let updated_patient_info = patient_registry::V1 {
        name: "john doe updated".to_string(),
        martial_status: "single".to_string(),
        place_of_birth: "jakarta".to_string(),
        address: "5678 oak st".to_string(),
        gender: "men".to_string(),
        date_of_birth: "1990-01-01".to_string(),
        kyc_status: KycStatus::Pending,
        kyc_date: "2024-01-02".to_string(),
    };

    let update_info_req = patient_registry::UpdatePatientInfoRequest {
        info: updated_patient_info,
    };

    registries
        .patient
        .update_patient_info(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Update,
            update_info_req,
        )
        .expect("Failed to update patient info");

    // verify updated info
    let result = registries
        .patient
        .get_patient_info(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    match result.patient {
        patient_registry::Patient::V1(v1) => {
            assert_eq!(v1.name, "john doe updated");
            assert_eq!(v1.martial_status, "single");
            match v1.kyc_status {
                KycStatus::Pending => (),
                _ => panic!("KYC status is not pending"),
            }
        }
    }
}
