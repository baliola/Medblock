use crate::common::Scenario;
use integration_tests::declarations::patient_registry::pocket_ic_bindings::Call as PatientCall;
use integration_tests::declarations::patient_registry::{
    KycStatus, Patient, RegisterPatientRequest, UpdateKycStatusRequest, UpdatePatientInfoRequest,
    V1,
};

/// TEST KYC STATUS TRANSITIONS
///
/// *PRECONDITIONS*
/// - One admin
/// - One patient
///
/// *TEST STEPS*
/// 1. Register patient (link NIK to Principal)
/// 2. Set initial patient info
/// 3. Admin denies KYC
/// 4. Update patient info after KYC denial
#[test]
fn test_kyc_status_transitions() {
    let (registries, patient, admin) = Scenario::one_admin_one_patient();

    // Step 1. Register patient (link NIK to Principal)
    registries
        .patient
        .register_patient(
            &registries.ic,
            patient.principal,
            PatientCall::Update,
            RegisterPatientRequest {
                nik: patient.nik.to_string(),
            },
        )
        .expect("Failed to register patient");

    // Step 2. Set initial patient info
    let initial_info = V1 {
        kyc_date: "2024-03-20".to_string(),
        name: "John Doe".to_string(),
        martial_status: "Single".to_string(),
        place_of_birth: "Jakarta".to_string(),
        address: "123 Main St".to_string(),
        gender: "Male".to_string(),
        kyc_status: KycStatus::Pending,
        date_of_birth: "1990-01-01".to_string(),
    };

    // Step 3. Set or Update patient info
    registries
        .patient
        .update_patient_info(
            &registries.ic,
            patient.principal,
            PatientCall::Update,
            UpdatePatientInfoRequest { info: initial_info },
        )
        .expect("Failed to update patient info");

    // Verify initial status is Pending
    let patient_info = registries
        .patient
        .get_patient_info(&registries.ic, patient.principal, PatientCall::Query)
        .expect("Failed to get patient info");

    match patient_info.patient {
        Patient::V1(info) => match info.kyc_status {
            KycStatus::Pending => (),
            _ => panic!("Initial status should be pending"),
        },
    }

    // Step 4. Admin denies KYC
    registries
        .patient
        .update_kyc_status(
            &registries.ic,
            admin,
            PatientCall::Update,
            UpdateKycStatusRequest {
                nik: patient.nik.to_string(),
                kyc_status: KycStatus::Denied,
            },
        )
        .expect("Failed to deny KYC");

    // Verify status is Denied
    let patient_info = registries
        .patient
        .get_patient_info(&registries.ic, patient.principal, PatientCall::Query)
        .expect("Failed to get patient info");

    match patient_info.patient {
        Patient::V1(info) => match info.kyc_status {
            KycStatus::Denied => (),
            _ => panic!("Status should be denied after admin denial"),
        },
    }

    // Step 5. Update patient info after KYC denial
    let updated_info = V1 {
        kyc_date: "2024-03-21".to_string(),
        name: "John Doe".to_string(),
        martial_status: "Single".to_string(),
        place_of_birth: "Jakarta".to_string(),
        address: "456 new st".to_string(),
        gender: "Male".to_string(),
        kyc_status: KycStatus::Pending,
        date_of_birth: "1990-01-01".to_string(),
    };

    // Step 6. Update patient info after KYC denial
    registries
        .patient
        .update_patient_info(
            &registries.ic,
            patient.principal,
            PatientCall::Update,
            UpdatePatientInfoRequest { info: updated_info },
        )
        .expect("Failed to update patient info after denial");

    // Verify the updated info was saved and status is back to Pending
    let patient_info = registries
        .patient
        .get_patient_info(&registries.ic, patient.principal, PatientCall::Query)
        .expect("Failed to get patient info");

    match patient_info.patient {
        Patient::V1(info) => {
            assert_eq!(
                info.address, "456 new st",
                "Updated address should be saved"
            );
            assert_eq!(
                info.kyc_date, "2024-03-21",
                "Updated KYC date should be saved"
            );
            match info.kyc_status {
                KycStatus::Pending => (),
                _ => panic!("Status should be back to pending after update"),
            }
        }
    }
}
