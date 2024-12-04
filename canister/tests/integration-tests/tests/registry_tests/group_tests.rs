use candid::Principal;
use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::{self, KycStatus, Relation, Result3},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
    provider_registry::{self},
};

use crate::common;

#[test]
fn test_group_creation_and_emr_access() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();

    // create another patient for group member
    let patient2 = common::Scenario::create_patient(&registries);

    // test group creation
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

    // generate consent code for patient2
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    let group_id = match group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    // add member to group
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

    // test granting EMR access
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id,
        grantee_nik: patient2.nik.to_string(),
    };

    registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            grant_access_req,
        )
        .unwrap();

    // test revoking EMR access
    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        grantee_nik: patient2.nik.to_string(),
    };

    registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            revoke_access_req,
        )
        .unwrap();

    // test leaving group
    let leave_group_req = patient_registry::LeaveGroupRequest { group_id };

    registries
        .patient
        .leave_group(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            leave_group_req,
        )
        .unwrap();
}

#[test]
fn test_emr_access_permissions() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);
    let patient3 = common::Scenario::create_patient(&registries);
    let provider = common::Provider(common::random_identity());

    println!("DEBUG patient1 NIK: {}", patient1.nik);
    println!("DEBUG patient2 NIK: {}", patient2.nik);
    println!("DEBUG patient3 NIK: {}", patient3.nik);

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

    println!("DEBUG issued EMR for Patient2");

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

    println!("DEBUG created group_id: {}", group_id);

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

    println!("DEBUG added patient2 to group");

    // verify patient2's groups
    let groups = registries
        .patient
        .get_user_groups(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();
    println!("DEBUG patient2's groups: {}", groups.groups.len());
    assert_eq!(groups.groups.len(), 1, "Patient2 should be in one group");

    // verify patient3 is not in any groups
    let groups = registries
        .patient
        .get_user_groups(
            &registries.ic,
            patient3.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();
    println!("DEBUG patient3's groups: {}", groups.groups.len());
    assert_eq!(
        groups.groups.len(),
        0,
        "Patient3 should not be in any groups"
    );

    // Test 1: Patient1 tries to view Patient2's EMR without permission (should fail)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient2.nik.to_string(),
        group_id,
        page: 0,
        limit: 10,
    };

    let view_result = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            view_request,
        );

    // Should fail with access not granted error
    match view_result {
        Ok(patient_registry::Result4::Err(error)) => {
            assert!(
                error.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error, got: {}",
                error
            );
            assert!(
                error.contains(&patient2.nik.to_string()),
                "Error should mention member's NIK"
            );
            assert!(
                error.contains(&patient1.nik.to_string()),
                "Error should mention viewer's NIK"
            );
        }
        _ => panic!("Expected error for unauthorized EMR access"),
    }

    // Test 2: Patient1 grants access to Patient2's EMR (should succeed)
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id,
        grantee_nik: patient1.nik.to_string(),
    };

    let grant_result = registries
        .patient
        .grant_group_access(
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

    let view_result = registries
        .patient
        .view_group_member_emr_information(
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

    // Test 4: Patient3 (not in group) tries to view Patient2's EMR (should fail)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient2.nik.to_string(),
        group_id,
        page: 0,
        limit: 10,
    };

    let view_result = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient3.principal.clone(),
            PatientCall::Query,
            view_request,
        );

    match view_result {
        Ok(patient_registry::Result4::Err(error)) => {
            assert!(
                error.contains("[ERR_NOT_GROUP_MEMBERS]"),
                "Expected not in group error, got: {}",
                error
            );
            assert!(
                error.contains(&patient3.nik.to_string()),
                "Error should mention viewer's NIK"
            );
        }
        _ => panic!("Expected error for non-group member access attempt"),
    }

    // Test 5: Patient1 tries to view Patient3's EMR (should fail - not in group)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient3.nik.to_string(),
        group_id,
        page: 0,
        limit: 10,
    };

    let view_result = registries
        .patient
        .view_group_member_emr_information(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            view_request,
        );

    match view_result {
        Ok(patient_registry::Result4::Err(error)) => {
            assert!(
                error.contains("[ERR_NOT_GROUP_MEMBERS]"),
                "Expected not in group error, got: {}",
                error
            );
            assert!(
                error.contains(&patient3.nik.to_string()),
                "Error should mention member's NIK"
            );
        }
        _ => panic!("Expected error for viewing non-group member EMR"),
    }
}

#[test]
fn test_group_retrieval() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();

    // create another patient for group member
    let patient2 = common::Scenario::create_patient(&registries);

    // test group creation
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

    // generate consent code for patient2
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    let group_id = match group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    // update the add_member_req
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

    // verify group membership
    let groups = registries
        .patient
        .get_user_groups(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(groups.groups.len(), 1);
    assert_eq!(groups.groups[0].id, group_id);

    // test leaving group
    let leave_group_req = patient_registry::LeaveGroupRequest { group_id };

    registries
        .patient
        .leave_group(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            leave_group_req,
        )
        .unwrap();

    // verify group membership after leaving
    let groups_after = registries
        .patient
        .get_user_groups(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(groups_after.groups.len(), 0);
}

#[test]
fn test_patient_group_assignment() {
    let (registry, provider, patient) = common::Scenario::one_provider_one_patient();

    let arg = integration_tests::declarations::provider_registry::IssueEmrRequest {
        emr: vec![
            integration_tests::declarations::provider_registry::EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            },
        ],
        user_id: patient.nik.clone().to_string(),
    };

    let response = registry
        .provider
        .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
        .unwrap();

    let result = registry
        .patient
        .create_consent(&registry.ic, patient.principal.clone(), PatientCall::Update)
        .unwrap();

    registry
        .patient
        .claim_consent(
            &registry.ic,
            provider.0.clone(),
            PatientCall::Update,
            integration_tests::declarations::patient_registry::ClaimConsentRequest {
                code: result.code.clone(),
            },
        )
        .unwrap();

    let search_result = registry
        .patient
        .search_patient(
            &registry.ic,
            provider.0.clone(),
            PatientCall::Query,
            integration_tests::declarations::patient_registry::SearchPatientRequest {
                nik: patient.nik.clone().to_string(),
            },
        )
        .unwrap();

    assert_eq!(
        search_result.patient_info.nik,
        patient.nik.clone().to_string()
    );
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
                "[ERR_ACCESS_NOT_GRANTED] Access not granted. The EMR owner (NIK: {}) has not granted you (NIK: {}) access to view their EMR information. They must use the grant_group_access function to give you permission.",
                patient2.nik, patient1.nik
            );
            assert_eq!(e, expected_error, "Unexpected error message");
        }
    }
}

#[test]
fn test_get_group_details() {
    // Create patient1 with specific gender info
    let (registries, _, _) = common::Scenario::one_admin_one_patient();
    let patient1 = common::Scenario::create_patient_with_info(
        &registries,
        patient_registry::V1 {
            name: "leader".to_string(),
            martial_status: "single".to_string(),
            place_of_birth: "jakarta".to_string(),
            address: "leader_address".to_string(),
            gender: "m".to_string(), // Explicitly set gender
            date_of_birth: "1990-01-01".to_string(),
            kyc_status: KycStatus::Pending,
            kyc_date: "2024-01-01".to_string(),
        },
    );

    // Rest of test patients with specific details for validation
    let test_patients = vec![
        ("test1", "m", "1990-01-01", Relation::Sibling),
        ("test2", "f", "2000-01-01", Relation::Child),
        ("test3", "m", "1995-01-01", Relation::Spouse),
    ];

    let mut patients = Vec::new();
    for (name, gender, dob, relation) in test_patients {
        let patient = common::Scenario::create_patient_with_info(
            &registries,
            patient_registry::V1 {
                name: name.to_string(),
                martial_status: "single".to_string(),
                place_of_birth: "jakarta".to_string(),
                address: format!("addr_{}", name),
                gender: gender.to_string(),
                date_of_birth: dob.to_string(),
                kyc_status: KycStatus::Pending,
                kyc_date: "2024-01-01".to_string(),
            },
        );
        patients.push((patient, relation));
    }

    // create group
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

    // add all patients to group with their specific relations
    for (patient, relation) in &patients {
        let consent_code = registries
            .patient
            .create_consent(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
            )
            .unwrap();

        let add_member_req = patient_registry::AddGroupMemberRequest {
            group_id,
            consent_code: consent_code.code,
            relation: match relation {
                Relation::Spouse => Relation::Spouse,
                Relation::Child => Relation::Child,
                Relation::Sibling => Relation::Sibling,
                Relation::Other => Relation::Other,
                Relation::Parent => Relation::Parent,
            },
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
    }

    // test getting group details
    let details_req = patient_registry::GetGroupDetailsRequest {
        group_id,
        page: 0,
        limit: 10,
    };

    let details_response = registries
        .patient
        .get_group_details(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            details_req,
        )
        .unwrap();

    match details_response {
        patient_registry::Result3::Ok(response) => {
            // verify basic group details
            assert_eq!(response.member_count, 4); // 3 members + 1 leader
            assert_eq!(response.total_pages, 1);
            assert_eq!(response.group_details.len(), 4);
            assert_eq!(response.group_name, "test family");

            // verify leader details
            let leader = response
                .group_details
                .iter()
                .find(|m| m.nik.to_string() == patient1.nik.to_string())
                .expect("Leader should be present");

            // Since we can't use assert_eq! for Relation, check role manually
            match leader.role {
                Relation::Parent => (), // this is what we expect
                _ => panic!("Leader should have Parent relation"),
            }

            assert!(leader.age > 0 && leader.age < 150, "Invalid age for leader");
            assert!(!leader.name.is_empty(), "Leader name should not be empty");
            assert!(
                leader.gender == "m" || leader.gender == "f",
                "Invalid gender for leader"
            );

            // verify each member's details
            for (patient, expected_relation) in &patients {
                let member = response
                    .group_details
                    .iter()
                    .find(|m| m.nik.to_string() == patient.nik.to_string())
                    .unwrap_or_else(|| panic!("Member {} not found in group", patient.nik));

                // verify all required fields
                assert!(!member.name.is_empty(), "Member name should not be empty");
                assert!(
                    member.gender == "m" || member.gender == "f",
                    "Invalid gender for member"
                );
                assert!(
                    member.age > 0 && member.age < 150,
                    "Invalid age for member: {}",
                    member.age
                );

                // Check relation manually with better error context
                let matches = match (&member.role, expected_relation) {
                    (Relation::Parent, Relation::Parent)
                    | (Relation::Spouse, Relation::Spouse)
                    | (Relation::Child, Relation::Child)
                    | (Relation::Sibling, Relation::Sibling)
                    | (Relation::Other, Relation::Other) => true,
                    _ => false,
                };

                if !matches {
                    panic!(
                        "Incorrect relation for member with NIK: {} (name: {})",
                        member.nik.to_string(),
                        member.name.to_string()
                    );
                }
            }

            // test pagination with smaller limit
            let paginated_req = patient_registry::GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 2,
            };

            let paginated_response = registries
                .patient
                .get_group_details(
                    &registries.ic,
                    patient1.principal.clone(),
                    PatientCall::Query,
                    paginated_req,
                )
                .unwrap();

            match paginated_response {
                patient_registry::Result3::Ok(paginated) => {
                    assert_eq!(paginated.group_details.len(), 2);
                    assert_eq!(paginated.member_count, 4);
                    assert_eq!(paginated.total_pages, 2);
                }
                patient_registry::Result3::Err(e) => {
                    panic!("Failed to get paginated details: {}", e)
                }
            }
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get group details: {}", e),
    }

    // test unauthorized access
    let unauthorized_patient = common::Scenario::create_patient(&registries);
    let unauthorized_req = patient_registry::GetGroupDetailsRequest {
        group_id,
        page: 0,
        limit: 10,
    };

    let unauthorized_response = registries
        .patient
        .get_group_details(
            &registries.ic,
            unauthorized_patient.principal.clone(),
            PatientCall::Query,
            unauthorized_req,
        )
        .unwrap();

    match unauthorized_response {
        patient_registry::Result3::Ok(_) => panic!("Unauthorized access should fail"),
        patient_registry::Result3::Err(e) => {
            assert!(
                e.contains("Only group members can view group details"),
                "Unexpected error message: {}",
                e
            )
        }
    }
}

#[test]
fn test_get_group_details_pagination() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();

    // create multiple patients for testing pagination
    let patients: Vec<_> = (0..5)
        .map(|i| {
            let patient = common::Scenario::create_patient_with_info(
                &registries,
                patient_registry::V1 {
                    name: format!("test{}", i),
                    martial_status: "single".to_string(),
                    place_of_birth: "jakarta".to_string(),
                    address: format!("addr{}", i),
                    gender: "m".to_string(),
                    date_of_birth: "1990-01-01".to_string(),
                    kyc_status: KycStatus::Pending,
                    kyc_date: "2024-01-01".to_string(),
                },
            );
            patient
        })
        .collect();

    // create group
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

    // add all patients to group
    for patient in &patients {
        let consent_code = registries
            .patient
            .create_consent(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
            )
            .unwrap();

        let add_member_req = patient_registry::AddGroupMemberRequest {
            group_id,
            consent_code: consent_code.code,
            relation: Relation::Other,
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
    }

    // test pagination with 2 members per page
    let details_req = patient_registry::GetGroupDetailsRequest {
        group_id,
        page: 0,
        limit: 2,
    };

    let first_page = registries
        .patient
        .get_group_details(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            details_req,
        )
        .unwrap();

    match first_page {
        patient_registry::Result3::Ok(response) => {
            assert_eq!(response.group_details.len(), 2);
            assert_eq!(response.member_count, 6); // 5 members + 1 leader
            assert_eq!(response.total_pages, 3);

            // check second page
            let second_page_req = patient_registry::GetGroupDetailsRequest {
                group_id,
                page: 1,
                limit: 2,
            };

            let second_page = registries
                .patient
                .get_group_details(
                    &registries.ic,
                    patient1.principal.clone(),
                    PatientCall::Query,
                    second_page_req,
                )
                .unwrap();

            match second_page {
                patient_registry::Result3::Ok(second_response) => {
                    assert_eq!(second_response.group_details.len(), 2);
                    assert_eq!(second_response.member_count, 6);
                    assert_eq!(second_response.total_pages, 3);

                    // verify different members on different pages
                    let first_page_niks: Vec<_> = response
                        .group_details
                        .iter()
                        .map(|d| d.nik.clone())
                        .collect();
                    let second_page_niks: Vec<_> = second_response
                        .group_details
                        .iter()
                        .map(|d| d.nik.clone())
                        .collect();

                    assert!(first_page_niks
                        .iter()
                        .all(|nik| !second_page_niks.contains(nik)));
                }
                patient_registry::Result3::Err(e) => panic!("Failed to get second page: {}", e),
            }
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get first page: {}", e),
    }
}

#[test]
fn test_claim_consent_for_group() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);

    // generate consent code for patient2
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    // patient1 claims patient2's consent
    let claim_result = registries
        .patient
        .claim_consent_for_group(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            patient_registry::ClaimConsentRequest {
                code: consent_code.code.clone(),
            },
        )
        .unwrap();

    // verify the returned NIK matches patient2's NIK
    match claim_result {
        patient_registry::Result1::Ok(nik) => {
            assert_eq!(nik, patient2.nik.to_string());
        }
        patient_registry::Result1::Err(e) => panic!("Failed to claim consent: {}", e),
    }

    // attempt to claim the same consent again (should fail)
    let second_claim = registries.patient.claim_consent_for_group(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Update,
        patient_registry::ClaimConsentRequest {
            code: consent_code.code,
        },
    );

    match second_claim.unwrap() {
        patient_registry::Result1::Ok(_) => panic!("Should not succeed"),
        patient_registry::Result1::Err(e) => assert!(e.contains("Consent already claimed")),
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

#[test]
fn test_emr_access_after_grant() {
    // setup: Create patients and provider
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

    // try to view EMRs before access grant (should fail)
    let view_result_before = registries
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

    assert!(
        matches!(view_result_before, patient_registry::Result4::Err(_)),
        "Should not be able to view EMRs before access grant"
    );

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
            for (i, emr) in response.emrs.iter().enumerate() {
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
fn test_group_access_cleanup() {
    let (registries, provider, patient1) = common::Scenario::one_provider_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);

    // issue EMRs for both patients
    let emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        }],
        user_id: patient1.nik.clone().to_string(),
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

    let emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "test_key2".to_string(),
            value: "test_value2".to_string(),
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

    let create_group_req = patient_registry::CreateGroupRequest {
        name: "test_group".to_string(),
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

    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id,
        consent_code: registries
            .patient
            .create_consent(
                &registries.ic,
                patient2.principal.clone(),
                PatientCall::Update,
            )
            .unwrap()
            .code,
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

    // patient1 grants access to patient2 (patient2 can view patient1's EMR)
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id,
        grantee_nik: patient2.nik.to_string(),
    };

    registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            grant_access_req,
        )
        .unwrap();

    // verify patient2 can view patient1's EMR (granted access)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        group_id,
        member_nik: patient1.nik.to_string(),
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

    assert!(
        matches!(result, patient_registry::Result4::Ok(_)),
        "Patient2 should be able to view Patient1's EMR initially. Got error: {:?}",
        if let patient_registry::Result4::Err(e) = result {
            e
        } else {
            "Unexpected result type".to_string()
        }
    );

    // verify patient1 cannot view patient2's EMR (no access granted)
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        group_id,
        member_nik: patient2.nik.to_string(),
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

    assert!(
        matches!(result, patient_registry::Result4::Err(_)),
        "Patient1 should not be able to view Patient2's EMR (no access granted)"
    );

    // patient2 leaves group
    let leave_group_req = patient_registry::LeaveGroupRequest { group_id };

    registries
        .patient
        .leave_group(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            leave_group_req,
        )
        .unwrap();

    // verify Patient2 can no longer view Patient1's EMR
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        group_id,
        member_nik: patient1.nik.to_string(),
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

    assert!(
        matches!(result, patient_registry::Result4::Err(_)),
        "Patient2 should not be able to view Patient1's EMR after leaving"
    );
}

#[test]
fn test_dissolve_group() {
    let (registries, leader, _) = common::Scenario::one_admin_one_patient();
    let member1 = common::Scenario::create_patient(&registries);

    let create_group_req = patient_registry::CreateGroupRequest {
        name: "test_group".to_string(),
    };

    let group_response = registries
        .patient
        .create_group(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            create_group_req,
        )
        .unwrap();

    let group_id = match group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id,
        consent_code: registries
            .patient
            .create_consent(
                &registries.ic,
                member1.principal.clone(),
                PatientCall::Update,
            )
            .unwrap()
            .code,
        relation: Relation::Spouse,
    };

    registries
        .patient
        .add_group_member(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            add_member_req,
        )
        .unwrap();

    // leader leaves group (should dissolve group)
    let leave_group_req = patient_registry::LeaveGroupRequest { group_id };

    registries
        .patient
        .leave_group(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            leave_group_req,
        )
        .unwrap();

    // verify group no longer exists
    let group_details_req = patient_registry::GetGroupDetailsRequest {
        group_id,
        page: 0,
        limit: 10,
    };

    let result = registries
        .patient
        .get_group_details(
            &registries.ic,
            member1.principal.clone(),
            PatientCall::Query,
            group_details_req,
        )
        .unwrap();

    assert!(
        matches!(result, patient_registry::Result3::Err(_)),
        "group should no longer exist"
    );
}

#[test]
fn test_emr_access_error_messages() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);
    let patient3 = common::Scenario::create_patient(&registries);
    let provider = common::Provider(common::random_identity());

    println!("DEBUG patient1 NIK: {}", patient1.nik);
    println!("DEBUG patient2 NIK: {}", patient2.nik);
    println!("DEBUG patient3 NIK: {}", patient3.nik);

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
    // This ensures the user exists in the EMR system
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

    println!("DEBUG registered patient2 in EMR system");

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

    // Add patient1 to group
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

    // Test 4: Access not granted
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
            patient1.principal.clone(),
            PatientCall::Query,
            view_request,
        )
        .unwrap();

    match result {
        patient_registry::Result4::Err(error) => {
            let expected_error = format!(
                "[ERR_ACCESS_NOT_GRANTED] Access not granted. The EMR owner (NIK: {}) has not granted you (NIK: {}) access to view their EMR information. Action required: The EMR owner must use the grant_group_access function to give you permission.",
                patient2.nik, patient1.nik
            );
            assert_eq!(error, expected_error, "Got unexpected error message");
        }
        _ => panic!("Expected error for access not granted"),
    }

    // Test 5: No EMRs found
    // First add patient3 to group
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient3.principal.clone(),
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

    // Grant access to patient3's EMR
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id,
        grantee_nik: patient1.nik.to_string(),
    };

    registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient3.principal.clone(),
            PatientCall::Update,
            grant_access_req,
        )
        .unwrap();

    // Try to view patient3's non-existent EMRs
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient3.nik.to_string(),
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
            println!("DEBUG actual error message: {}", error);
            let expected_error = format!(
                "[ERR_NO_EMR_RECORDS] The member (NIK: {}) has not been registered in the EMR system yet. Action required: They need to visit a healthcare provider who will create their first EMR record.",
                patient3.nik
            );
            assert_eq!(error, expected_error, "Got unexpected error message");
        }
        patient_registry::Result4::Ok(_) => {
            println!("DEBUG got unexpected success when expecting no EMRs error");
            panic!("Expected error for no EMRs found");
        }
        _ => panic!("Expected error for no EMRs found"),
    }

    // Test 6: Successful case after adding EMRs
    // Add EMR for patient2 (using the already registered provider)
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

    // Grant access to patient2's EMR
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

    // Now viewing should succeed
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
            patient1.principal.clone(),
            PatientCall::Query,
            view_request,
        )
        .unwrap();

    assert!(
        matches!(result, patient_registry::Result4::Ok(_)),
        "Expected successful EMR view after all conditions are met"
    );
}
