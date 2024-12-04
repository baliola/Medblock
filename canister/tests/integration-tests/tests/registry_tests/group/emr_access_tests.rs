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
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
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
        group_id,
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
        group_id,
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
        Ok(patient_registry::Result4::Err(error)) => {
            assert!(
                error.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error"
            );
        }
        _ => panic!("Expected error for unauthorized EMR access"),
    }

    // Test 2: Patient1 grants access to Patient2's EMR (should succeed)
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id,
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
        group_id,
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
        Ok(patient_registry::Result4::Ok(_)) => (),
        Ok(patient_registry::Result4::Err(e)) => panic!("Expected success but got error: {}", e),
        Err(_) => panic!("Expected success but got pocket_ic error"),
    }
}

#[test]
fn test_emr_access_after_grant() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);
    let provider = common::Provider(common::random_identity());

    // register provider
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

    let group_id = match group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    // add patient2 to the group
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    registries
        .patient
        .add_group_member(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            patient_registry::AddGroupMemberRequest {
                group_id,
                consent_code: consent_code.code,
                relation: Relation::Spouse,
            },
        )
        .unwrap();

    // issue multiple EMRs for patient2
    for i in 1..=3 {
        let emr_req = provider_registry::IssueEmrRequest {
            emr: vec![provider_registry::EmrFragment {
                key: format!("test_key_{}", i),
                value: format!("test_value_{}", i),
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
    }

    // grant access from patient2 to patient1
    registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            patient_registry::GrantGroupAccessRequest {
                group_id,
                grantee_nik: patient1.nik.to_string(),
            },
        )
        .unwrap();

    // view EMRs after access grant (should succeed)
    let view_result_after = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            patient_registry::ViewGroupMemberEmrInformationRequest {
                member_nik: patient2.nik.to_string(),
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match view_result_after {
        patient_registry::Result4::Ok(response) => {
            assert_eq!(
                response.emrs.len(),
                3,
                "Should have access to all 3 EMRs after grant"
            );
            // verify EMR contents
            for emr in response.emrs.iter() {
                assert_eq!(
                    emr.header.user_id,
                    patient2.nik.to_string(),
                    "EMR should belong to patient2"
                );
            }
        }
        patient_registry::Result4::Err(e) => {
            panic!("Failed to view EMRs after access grant: {}", e)
        }
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
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
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
        group_id,
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
        group_id,
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
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    // Verify EMRs are returned
    match view_result {
        patient_registry::Result4::Ok(response) => {
            assert!(!response.emrs.is_empty(), "Should have returned EMRs");
        }
        patient_registry::Result4::Err(e) => panic!("Failed to view EMRs: {}", e),
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
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match view_result_after_revoke {
        patient_registry::Result4::Ok(_) => {
            panic!("Should not be able to view EMRs after access revocation")
        }
        patient_registry::Result4::Err(e) => {
            let expected_error = format!(
                "[ERR_ACCESS_NOT_GRANTED] Access not granted. The EMR owner (NIK: {}) has not granted you (NIK: {}) access to view their EMR information. Action required: The EMR owner must use the grant_group_access function to give you permission.",
                patient2.nik, patient1.nik
            );
            assert_eq!(e, expected_error, "Unexpected error message");
        }
    }
}
