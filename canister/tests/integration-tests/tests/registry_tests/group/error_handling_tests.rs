use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::{self},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
    provider_registry::{self},
};

use crate::common;

#[test]
fn test_emr_access_error_messages() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);
    let patient3 = common::Scenario::create_patient(&registries);
    let provider = common::Provider(common::random_identity());

    // Register provider first
    let provider_reg_req = provider_registry::RegisternewProviderRequest {
        provider_principal: provider.0.clone(),
        display_name: "TEST HOSPITAL".to_ascii_lowercase(),
        address: "TEST ADDRESS".to_ascii_lowercase(),
    };

    registries
        .provider
        .register_new_provider(
            &registries.ic,
            registries.controller.clone(),
            ProviderCall::Update,
            provider_reg_req,
        )
        .unwrap();

    // Register patient2 in the EMR system by issuing a dummy EMR
    let dummy_emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "init".to_string(),
            value: "init".to_string(),
        }],
        user_id: patient2.nik.clone().to_string(),
    };

    registries
        .provider
        .issue_emr(
            &registries.ic,
            provider.0.clone(),
            ProviderCall::Update,
            dummy_emr_req,
        )
        .unwrap();

    // Create group with patient1 as leader
    let create_group_req = patient_registry::CreateGroupRequest {
        name: "test family".to_string(),
    };

    let group_response = registries
        .patient
        .create_group(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            create_group_req,
        )
        .unwrap();

    let group_id = match group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    // Test 1: Invalid NIK format
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: "invalid_nik".to_string(),
        group_id,
        page: 0,
        limit: 10,
    };

    let result = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            view_request,
        )
        .unwrap();

    match result {
        patient_registry::Result4::Err(error) => {
            assert!(
                error.contains("[ERR_INVALID_NIK]"),
                "Expected invalid NIK error message, got: {}",
                error
            );
        }
        _ => panic!("Expected error for invalid NIK format"),
    }

    // Test 2: Invalid group ID
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient2.nik.to_string(),
        group_id: 999, // Non-existent group ID
        page: 0,
        limit: 10,
    };

    let result = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            view_request,
        )
        .unwrap();

    match result {
        patient_registry::Result4::Err(error) => {
            let expected_error = format!(
                "[ERR_GROUP_NOT_FOUND] Group with ID {} does not exist in the system. Please verify the group ID or create a new group if needed.",
                999
            );
            assert_eq!(error, expected_error, "Got unexpected error message");
        }
        _ => panic!("Expected error for invalid group ID"),
    }

    // Test 3: Users not in group
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient2.nik.to_string(),
        group_id,
        page: 0,
        limit: 10,
    };

    let result = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Query,
            view_request,
        )
        .unwrap();

    match result {
        patient_registry::Result4::Err(error) => {
            let expected_error = format!(
                "[ERR_NOT_GROUP_MEMBERS] Neither you (NIK: {}) nor the member (NIK: {}) are members of group {}. Action required: Both users must join the group first. The group leader can add members using the add_group_member function.",
                patient2.nik, patient2.nik, group_id
            );
            assert_eq!(error, expected_error, "Got unexpected error message");
        }
        _ => panic!("Expected error for users not in group"),
    }
}

#[test]
fn test_claim_nonexistent_consent_for_group() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();

    // attempt to claim a non-existent consent code
    let result = registries.patient.claim_consent_for_group(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Update,
        patient_registry::ClaimConsentRequest {
            code: "123456".to_string(),
        },
    );

    match result.unwrap() {
        patient_registry::Result1::Ok(_) => panic!("Should not succeed"),
        patient_registry::Result1::Err(e) => assert!(e.contains("Consent not found")),
    }
}

#[test]
#[should_panic(expected = "only patient can call this method")]
fn test_claim_consent_for_group_unauthorized() {
    let (registries, _, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);
    let unauthorized = common::random_identity();

    // generate consent code for patient2
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    // attempt to claim consent with unauthorized principal (should panic)
    registries
        .patient
        .claim_consent_for_group(
            &registries.ic,
            unauthorized,
            PatientCall::Update,
            patient_registry::ClaimConsentRequest {
                code: consent_code.code,
            },
        )
        .unwrap();
}
