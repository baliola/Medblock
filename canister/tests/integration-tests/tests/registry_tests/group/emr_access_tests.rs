use std::time::Duration;

use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::{self, Relation},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
    provider_registry::{self},
};

use crate::common;

#[test]
fn test_emr_access_permissions() {
    let (registries, provider, patient1, patient2, group_id) =
        common::Scenario::two_patients_one_provider_one_group();

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

    let group_details_req = patient_registry::CreateGroupResponse {
        group_id: group_id.clone(),
    };

    // lets see whos actually in the group at this time (act as controller)
    let group_details = registries.patient.get_group_details_async_no_pagination(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        group_details_req,
    );

    match group_details {
        Ok(patient_registry::Result3::Ok(response)) => {
            println!("group details member count: {:?}", response.member_count);
        }
        Ok(patient_registry::Result3::Err(e)) => {
            println!("error: {:?}", e);
        }
        Err(e) => {
            println!("error: {:?}", e);
        }
    }

    match view_result {
        Ok(patient_registry::Result5::Err(error)) => {
            assert!(
                error.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error, but got {}",
                error
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
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    let consent_code = registries
        .patient
        .create_consent_for_group(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateConsentForGroupRequest {
                nik: patient2.nik.clone().to_string(),
            },
        )
        .unwrap();

    // step 2. patient 1 adds patient2 to group
    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id: group_id.clone(),
        group_consent_code: consent_code.group_consent_code,
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

    println!(
        "viewing EMRs for patient 1: {:?}, from group: {:?}, by patient 2: {:?}",
        patient1.principal, group_id, patient2.principal
    );

    let view_result = registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        view_request,
    );

    let result = match view_result {
        Ok(patient_registry::Result5::Ok(ref emr_info)) => {
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
            true
        }
        Ok(patient_registry::Result5::Err(e)) => {
            println!("Error: {}", e);
            false
        }
        Err(_) => false,
    };

    assert!(result, "Failed to view EMRs");
}

#[test]
fn test_view_group_member_emr_information() {
    let (registries, provider, patient1, patient2, group_id) =
        common::Scenario::two_patients_one_provider_one_group();

        // patient 1 gives access to patient 2
        let grant_access_req = patient_registry::GrantGroupAccessRequest {
            group_id: group_id.clone(),
            grantee_nik: patient2.nik.to_string(),
        };

        let grant_result = registries
            .patient
            .grant_group_access(
                &registries.ic,
                patient1.principal.clone(),
                PatientCall::Update,
                grant_access_req,
            )
            .unwrap();

        match grant_result {
            patient_registry::Result_::Ok => (),
            patient_registry::Result_::Err(e) => panic!("Failed to grant access: {}", e),
        }

        // patient 2 views patient1's EMRs
        let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
            member_nik: patient1.nik.to_string(),
            group_id: group_id.clone(),
            page: 0,
            limit: 10,
        };

        let view_result = registries.patient.view_group_member_emr_information(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Query,
            view_request,
        );

        // get the hospital name from the provider
        let hospital_name = registries.provider.get_provider_info_with_principal(
            &registries.ic,
            provider.0.clone(),
            ProviderCall::Query,
            provider_registry::ProviderInfoRequest {
                provider: vec![provider.0.clone()],
            },
        );

        let hospital_name = match hospital_name {
            Ok(response) => {
                match &response.providers[0] {
                    provider_registry::Provider::V1(provider) => {
                        provider.display_name.clone()
                    }
                }
            }
            Err(e) => panic!("Expected success but got error: {}", e),
        };

        match view_result {
            Ok(patient_registry::Result5::Ok(emr_info)) => {
                assert!(!emr_info.emrs.is_empty(), "EMR list should not be empty");
                assert_eq!(
                    emr_info.emrs[0].header.user_id,
                    patient1.nik.to_string(),
                    "User ID should match"
                );
                assert!(
                    !emr_info.emrs[0].header.emr_id.is_empty(),
                    "EMR ID should not be empty"
                );
                assert!(
                    emr_info.emrs[0].hospital_name == hospital_name,
                    "Hospital name should match: {:?} vs {:?}",
                    emr_info.emrs[0].hospital_name,
                    hospital_name
                );
            }
            Ok(patient_registry::Result5::Err(e)) => panic!("Expected success but got error: {}", e),
            Err(_) => panic!("Expected success but got pocket_ic error"),
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

    // advance IC time by 2 seconds
    registries.ic.advance_time(Duration::from_secs(2));
    // process multiple ticks to ensure all operations are complete
    for _ in 0..5 {
        registries.ic.tick();
    }

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
        patient_registry::Result2::Ok(response) => response.group_id,
        _ => panic!("Failed to create group1"),
    };

    let group2_id = match group2_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        _ => panic!("Failed to create group2"),
    };

    // Add patient2 to both groups
    let consent_code1 = registries
        .patient
        .create_consent_for_group(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateConsentForGroupRequest {
                nik: patient2.nik.clone().to_string(),
            },
        )
        .unwrap();

    // wait for 1 second
    std::thread::sleep(std::time::Duration::from_secs(2));

    let consent_code2 = registries
        .patient
        .create_consent_for_group(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateConsentForGroupRequest {
                nik: patient2.nik.clone().to_string(),
            },
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
                group_consent_code: consent_code1.group_consent_code,
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
                group_consent_code: consent_code2.group_consent_code,
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

#[test]
fn test_view_single_emr_through_group() {
    let (registries, provider, patient1, patient2, group_id) =
        common::Scenario::two_patients_one_provider_one_group();

    // check EMRs for both patients before proceeding
    let patient1_emrs = registries.patient.emr_list_patient(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        patient_registry::EmrListPatientRequest {
            page: 0,
            limit: 10,
        },
    );
    println!("DEBUG test: patient1 EMR count before test: {:?}", patient1_emrs.unwrap().emrs.len());

    let patient2_emrs = registries.patient.emr_list_patient(
        &registries.ic,
        patient2.principal.clone(),
        PatientCall::Query,
        patient_registry::EmrListPatientRequest {
            page: 0,
            limit: 10,
        },
    );
    println!("DEBUG test: patient2 EMR count before test: {:?}", patient2_emrs.unwrap().emrs.len());

    // patient2 grants access to patient1
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

    match grant_result {
        patient_registry::Result_::Ok => println!("DEBUG test: access granted successfully"),
        patient_registry::Result_::Err(e) => panic!("Failed to grant access: {}", e),
    }

    // check that provider has emr
    let emr_count = registries.provider.emr_list_provider(
        &registries.ic,
        provider.0.clone(),
        ProviderCall::Query,
        provider_registry::EmrListProviderRequest {
            page: 0,
            limit: 10,
        },
    );

    println!("DEBUG test: provider total EMR count: {:?}", emr_count.unwrap().ids.len());

    // patient1 views patient2's single EMR
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
        Ok(patient_registry::Result5::Ok(emr_info)) => {
            println!("DEBUG test: successfully retrieved EMRs, count: {:?}", emr_info.emrs.len());
            assert_eq!(emr_info.emrs.len(), 1, "Should have exactly one EMR");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "User ID should match"
            );
            
            // get full EMR information using the header information
            let read_emr_request = patient_registry::ReadGroupMembersEmrInfoRequest {
                provider_id: emr_info.emrs[0].header.provider_id.to_string(),
                emr_id: emr_info.emrs[0].header.emr_id.to_string(),
                registry_id: registries.emr.0,
                group_id: group_id.clone(),
                member_nik: patient2.nik.to_string(),
            };

            let full_emr_result = registries.patient.read_group_members_emr_info(
                &registries.ic,
                patient1.principal.clone(),
                PatientCall::Query,
                read_emr_request
            );

            match full_emr_result {
                Ok(patient_registry::Result4::Ok(full_emr)) => {
                    println!("DEBUG test: successfully retrieved full EMR");
                    assert_eq!(
                        full_emr.emr.header.emr_id,
                        emr_info.emrs[0].header.emr_id,
                        "EMR IDs should match"
                    );
                    assert!(!full_emr.emr.body.is_empty(), "EMR body should not be empty");
                },
                Ok(patient_registry::Result4::Err(e)) => {
                    println!("DEBUG test: failed to retrieve full EMR with error: {:?}", e);
                    panic!("Function returned error: {:?}", e);
                },
                Err(e) => {
                    println!("DEBUG test: got RPC error: {:?}", e);
                    panic!("RPC call failed: {:?}", e);
                }
            }
        }
        Ok(patient_registry::Result5::Err(e)) => {
            println!("DEBUG test: failed to retrieve EMRs with error: {:?}", e);
            panic!("Expected success but got error: {}", e)
        },
        Err(e) => {
            println!("DEBUG test: got pocket_ic error: {:?}", e);
            panic!("Expected success but got pocket_ic error")
        },
    }
}
