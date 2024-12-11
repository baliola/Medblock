use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::{self, Relation},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
    provider_registry::{self},
};

use crate::common;

#[test]
fn test_emr_access_permissions() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);
    let _ = common::Scenario::create_patient(&registries);
    let provider = common::Provider(common::random_identity());

    // Register the provider first
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

    // Issue some EMRs for Patient2
    let emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        }],
        user_id: patient2.nik.clone().to_string(),
    };

    registries
        .provider
        .issue_emr(
            &registries.ic,
            provider.0.clone(),
            ProviderCall::Update,
            emr_req,
        )
        .unwrap();

    // create group with patient1 as leader
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    // add patient2 to group
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id: group_id.clone(),
        consent_code: consent_code.code,
        relation: Relation::Spouse,
    };

    registries
        .patient
        .add_group_member(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            add_member_req,
        )
        .unwrap();

    // Test 1: Patient1 tries to view Patient2's EMR without permission (should fail)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient2.nik.to_string(),
        group_id: group_id.clone(),
        page: 0,
        limit: 10,
    };

    let view_result = registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        view_request,
    );

    match view_result {
        Ok(patient_registry::Result5::Err(error)) => {
            assert!(
                error.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error"
            );
        }
        _ => panic!("Expected error for unauthorized EMR access"),
    }

    // Test 2: Patient1 grants access to Patient2's EMR (should succeed)
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id: group_id.clone(),
        grantee_nik: patient1.nik.to_string(),
    };

    let grant_result = registries.patient.grant_group_access(
        &registries.ic,
        patient2.principal.clone(),
        PatientCall::Update,
        grant_access_req,
    );

    assert!(grant_result.is_ok(), "Failed to grant access");

    // Test 3: Patient1 tries to view Patient2's EMR with permission (should succeed)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient2.nik.to_string(),
        group_id: group_id.clone(),
        page: 0,
        limit: 10,
    };

    let view_result = registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        view_request,
    );

    match view_result {
        Ok(patient_registry::Result5::Ok(_)) => (),
        Ok(patient_registry::Result5::Err(e)) => panic!("Expected success but got error: {}", e),
        Err(_) => panic!("Expected success but got pocket_ic error"),
    }
}

/// TEST EMR ACCESS AFTER GRANT
///
/// *PREREQUISITES*:
/// - One registered admin
/// - One registered provider
/// - One registered patient with EMR from the provider above
///
/// *FLOW BEING TESTED*:
/// 1. Create group
/// 2. Add member to group
/// 3. Grant access
/// 4. View EMRs
#[test]
fn test_emr_access_after_grant() {
    let (registries, provider, patient1, patient2) =
        common::Scenario::one_provider_two_patient_with_emrs();

    // step 1. patient 1 creates a group
    let group_response = registries
        .patient
        .create_group(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateGroupRequest {
                name: "test family".to_string(),
            },
        )
        .unwrap();

    // verify group creation is successful and get group id
    let group_id = match group_response {
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    // verify consent creation is successful
    assert!(consent_code.code.len() > 0, "Failed to create consent");

    // step 2. patient 1 adds patient2 to group
    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id: group_id.clone(),
        consent_code: consent_code.code,
        relation: Relation::Spouse,
    };

    let add_member_result = registries
        .patient
        .add_group_member(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            add_member_req,
        )
        .unwrap();

    // verify add member is successful
    match add_member_result {
        patient_registry::Result_::Ok => (),
        patient_registry::Result_::Err(e) => panic!("Failed to add member to group: {}", e),
    }

    // step 3. patient 2 grants access to patient 1
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id: group_id.clone(),
        grantee_nik: patient1.nik.to_string(),
    };

    let grant_result = registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            grant_access_req,
        )
        .unwrap();

    // verify grant access is successful
    match grant_result {
        patient_registry::Result_::Ok => (),
        patient_registry::Result_::Err(e) => panic!("Failed to grant access: {}", e),
    }

    // step 4. view EMRs (patient 1 should be able to view patient2's EMRs)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient2.nik.to_string(),
        group_id: group_id.clone(),
        page: 0,
        limit: 10,
    };

    let view_result = registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        view_request,
    );

    println!("[test_emr_access_after_grant] view_result: {:?}", view_result);

    match view_result {
        Ok(patient_registry::Result5::Ok(emr_info)) => {
            assert!(!emr_info.emrs.is_empty(), "EMR list should not be empty");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "User ID should match"
            );
            assert!(
                !emr_info.emrs[0].header.emr_id.is_empty(),
                "EMR ID should not be empty"
            );
        }
        Ok(patient_registry::Result5::Err(e)) => panic!("Expected success but got error: {}", e),
        Err(_) => panic!("Expected success but got pocket_ic error"),
    }
}

#[test]
fn test_view_group_member_emr_information() {
    // setup initial registries and patients
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);

    // create and register a provider using the same registry
    let provider = common::Provider(common::random_identity());

    // register the provider
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

    // create a group
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    // add patient2 to group
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id: group_id.clone(),
        consent_code: consent_code.code,
        relation: Relation::Spouse,
    };

    registries
        .patient
        .add_group_member(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            add_member_req,
        )
        .unwrap();

    // issue EMRs for patient2 using the same registry
    let emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        }],
        user_id: patient2.nik.clone().to_string(),
    };

    registries
        .provider
        .issue_emr(
            &registries.ic,
            provider.0.clone(),
            ProviderCall::Update,
            emr_req,
        )
        .unwrap();

    // grant access to patient1 to view patient2's EMRs
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id: group_id.clone(),
        grantee_nik: patient1.nik.to_string(),
    };

    registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            grant_access_req,
        )
        .unwrap();

    // test viewing EMRs - should succeed
    let view_result = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            patient_registry::ViewGroupMemberEmrInformationRequest {
                member_nik: patient2.nik.to_string(),
                group_id: group_id.clone(),
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    // Verify EMRs are returned
    match view_result {
        patient_registry::Result5::Ok(response) => {
            assert!(!response.emrs.is_empty(), "Should have returned EMRs");
        }
        patient_registry::Result5::Err(e) => panic!("Failed to view EMRs: {}", e),
    }

    // revoke access
    let revoke_req = patient_registry::RevokeGroupAccessRequest {
        grantee_nik: patient1.nik.to_string(),
    };

    registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            revoke_req,
        )
        .unwrap();

    // test viewing EMRs after revocation - should fail
    let view_result_after_revoke = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            patient_registry::ViewGroupMemberEmrInformationRequest {
                member_nik: patient2.nik.to_string(),
                group_id: group_id.clone(),
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match view_result_after_revoke {
        patient_registry::Result5::Ok(_) => {
            panic!("Should not be able to view EMRs after access revocation")
        }
        patient_registry::Result5::Err(e) => {
            let expected_error = format!(
                "[ERR_ACCESS_NOT_GRANTED] Access not granted for group {}. The EMR owner (NIK: {}) has not granted you (NIK: {}) access to view their EMR information in this group.",
                group_id, patient2.nik, patient1.nik
            );
            assert_eq!(e, expected_error, "Unexpected error message");
        }
    }
}

#[test]
fn test_group_specific_access() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);
    let provider = common::Provider(common::random_identity());

    // Create two groups
    let group1_response = registries
        .patient
        .create_group(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateGroupRequest {
                name: "group1".to_string(),
            },
        )
        .unwrap();

    // wait for 1 second
    std::thread::sleep(std::time::Duration::from_secs(1));

    let group2_response = registries
        .patient
        .create_group(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateGroupRequest {
                name: "group2".to_string(),
            },
        )
        .unwrap();

    let group1_id = match group1_response {
        patient_registry::Result3::Ok(response) => response.group_id,
        _ => panic!("Failed to create group1"),
    };

    let group2_id = match group2_response {
        patient_registry::Result3::Ok(response) => response.group_id,
        _ => panic!("Failed to create group2"),
    };

    // Add patient2 to both groups
    let consent_code1 = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    let consent_code2 = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    // Add to group1
    registries
        .patient
        .add_group_member(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            patient_registry::AddGroupMemberRequest {
                group_id: group1_id.clone(),
                consent_code: consent_code1.code,
                relation: Relation::Spouse,
            },
        )
        .unwrap();

    // Add to group2
    registries
        .patient
        .add_group_member(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            patient_registry::AddGroupMemberRequest {
                group_id: group2_id.clone(),
                consent_code: consent_code2.code,
                relation: Relation::Spouse,
            },
        )
        .unwrap();

    // Register provider and issue EMR for patient2
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

    let emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        }],
        user_id: patient2.nik.clone().to_string(),
    };

    registries
        .provider
        .issue_emr(
            &registries.ic,
            provider.0.clone(),
            ProviderCall::Update,
            emr_req,
        )
        .unwrap();

    // Grant access only in group1
    registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            patient_registry::GrantGroupAccessRequest {
                group_id: group1_id.clone(),
                grantee_nik: patient1.nik.to_string(),
            },
        )
        .unwrap();

    // Should succeed for group1
    let view_result_group1 = registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        patient_registry::ViewGroupMemberEmrInformationRequest {
            member_nik: patient2.nik.to_string(),
            group_id: group1_id.clone(),
            page: 0,
            limit: 10,
        },
    );

    assert!(view_result_group1.is_ok(), "Should have access in group1");

    // Should fail for group2
    let view_result_group2 = registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        patient_registry::ViewGroupMemberEmrInformationRequest {
            member_nik: patient2.nik.to_string(),
            group_id: group2_id.clone(),
            page: 0,
            limit: 10,
        },
    );

    match view_result_group2 {
        Ok(patient_registry::Result5::Ok(_)) => panic!("Should not have access in group2"),
        Ok(patient_registry::Result5::Err(e)) => assert!(
            e.contains("[ERR_ACCESS_NOT_GRANTED]"),
            "Expected access not granted error, got: {}",
            e
        ),
        Err(e) => panic!("Unexpected error: {}", e),
    }
}
