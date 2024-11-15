use candid::Principal;
use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::KycStatus,
    patient_registry::{self, Relation},
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
        patient_registry::Result1::Ok(response) => response.group_id,
        patient_registry::Result1::Err(e) => panic!("Failed to create group: {}", e),
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

    println!("DEBUG patient1 NIK: {}", patient1.nik);
    println!("DEBUG patient2 NIK: {}", patient2.nik);
    println!("DEBUG patient3 NIK: {}", patient3.nik);

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
        patient_registry::Result1::Ok(response) => response.group_id,
        patient_registry::Result1::Err(e) => panic!("Failed to create group: {}", e),
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

    // test granting EMR access to group member
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id,
        grantee_nik: patient2.nik.to_string(),
    };

    let grant_result = registries.patient.grant_group_access(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Update,
        grant_access_req,
    );
    println!("DEBUG grant_result to patient2: {}", grant_result.is_ok());
    assert!(grant_result.is_ok());

    // test granting EMR access to non-group member (should fail)
    let invalid_grant_req = patient_registry::GrantGroupAccessRequest {
        group_id,
        grantee_nik: patient3.nik.to_string(),
    };

    let invalid_grant_result = registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
            invalid_grant_req,
        )
        .unwrap();

    // Now check the inner Result_
    assert!(matches!(
        invalid_grant_result,
        patient_registry::Result_::Err(_)
    ));

    // verify that patient3 is not in the group
    let groups = registries
        .patient
        .get_user_groups(
            &registries.ic,
            patient3.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();
    assert_eq!(groups.groups.len(), 0);

    // test revoking EMR access
    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        grantee_nik: patient2.nik.to_string(),
    };

    let revoke_result = registries.patient.revoke_group_access(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Update,
        revoke_access_req,
    );
    println!("DEBUG revoke_result: {}", revoke_result.is_ok());
    assert!(revoke_result.is_ok());
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
        patient_registry::Result1::Ok(response) => response.group_id,
        patient_registry::Result1::Err(e) => panic!("Failed to create group: {}", e),
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
        patient_registry::Result1::Ok(response) => response.group_id,
        patient_registry::Result1::Err(e) => panic!("Failed to create group: {}", e),
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
        patient_registry::Result3::Ok(response) => {
            assert!(!response.emrs.is_empty(), "Should have returned EMRs");
        }
        patient_registry::Result3::Err(e) => panic!("Failed to view EMRs: {}", e),
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
        patient_registry::Result3::Ok(_) => {
            panic!("Should not be able to view EMRs after access revocation")
        }
        patient_registry::Result3::Err(e) => {
            assert!(e.contains("No access granted"), "Unexpected error message");
        }
    }
}

#[test]
fn test_get_group_details() {
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

    let group_id = match group_response {
        patient_registry::Result1::Ok(response) => response.group_id,
        patient_registry::Result1::Err(e) => panic!("Failed to create group: {}", e),
    };

    // generate consent code for patient2
    let consent_code = registries
        .patient
        .create_consent(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    // add patient2 to group
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

    // test getting group details as group leader (patient1)
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
        patient_registry::Result2::Ok(response) => {
            // verify response
            assert_eq!(response.member_count, 2);
            assert_eq!(response.total_pages, 1);
            assert_eq!(response.group_details.len(), 2);

            // verify leader details
            let leader = response
                .group_details
                .iter()
                .find(|m| matches!(m.role, Relation::Parent))
                .expect("Leader should be present");
            assert_eq!(leader.nik, patient1.nik.to_string());

            // verify member details
            let member = response
                .group_details
                .iter()
                .find(|m| matches!(m.role, Relation::Other))
                .expect("Member should be present");
            assert_eq!(member.nik, patient2.nik.to_string());

            // test getting group details as group member (patient2)
            let member_details_req = patient_registry::GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 10,
            };

            let member_details_response = registries
                .patient
                .get_group_details(
                    &registries.ic,
                    patient2.principal.clone(),
                    PatientCall::Query,
                    member_details_req,
                )
                .unwrap();

            // verify member can see the same details
            match member_details_response {
                patient_registry::Result2::Ok(member_response) => {
                    assert_eq!(member_response.member_count, response.member_count);
                    assert_eq!(member_response.total_pages, response.total_pages);
                    assert_eq!(
                        member_response.group_details.len(),
                        response.group_details.len()
                    );
                }
                patient_registry::Result2::Err(e) => panic!("Member failed to get details: {}", e),
            }
        }
        patient_registry::Result2::Err(e) => panic!("Failed to get group details: {}", e),
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
        patient_registry::Result1::Ok(response) => response.group_id,
        patient_registry::Result1::Err(e) => panic!("Failed to create group: {}", e),
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
        patient_registry::Result2::Ok(response) => {
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
                patient_registry::Result2::Ok(second_response) => {
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
                patient_registry::Result2::Err(e) => panic!("Failed to get second page: {}", e),
            }
        }
        patient_registry::Result2::Err(e) => panic!("Failed to get first page: {}", e),
    }
}
