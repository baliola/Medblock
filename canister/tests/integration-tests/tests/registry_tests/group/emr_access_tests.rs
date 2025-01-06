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
        Ok(response) => match &response.providers[0] {
            provider_registry::Provider::V1(provider) => provider.display_name.clone(),
        },
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
        patient_registry::EmrListPatientRequest { page: 0, limit: 10 },
    );
    println!(
        "DEBUG test: patient1 EMR count before test: {:?}",
        patient1_emrs.unwrap().emrs.len()
    );

    let patient2_emrs = registries.patient.emr_list_patient(
        &registries.ic,
        patient2.principal.clone(),
        PatientCall::Query,
        patient_registry::EmrListPatientRequest { page: 0, limit: 10 },
    );
    println!(
        "DEBUG test: patient2 EMR count before test: {:?}",
        patient2_emrs.unwrap().emrs.len()
    );

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
        provider_registry::EmrListProviderRequest { page: 0, limit: 10 },
    );

    println!(
        "DEBUG test: provider total EMR count: {:?}",
        emr_count.unwrap().ids.len()
    );

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
            println!(
                "DEBUG test: successfully retrieved EMRs, count: {:?}",
                emr_info.emrs.len()
            );
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
                read_emr_request,
            );

            match full_emr_result {
                Ok(patient_registry::Result4::Ok(full_emr)) => {
                    println!("DEBUG test: successfully retrieved full EMR");
                    assert_eq!(
                        full_emr.emr.header.emr_id, emr_info.emrs[0].header.emr_id,
                        "EMR IDs should match"
                    );
                    assert!(
                        !full_emr.emr.body.is_empty(),
                        "EMR body should not be empty"
                    );
                }
                Ok(patient_registry::Result4::Err(e)) => {
                    println!(
                        "DEBUG test: failed to retrieve full EMR with error: {:?}",
                        e
                    );
                    panic!("Function returned error: {:?}", e);
                }
                Err(e) => {
                    println!("DEBUG test: got RPC error: {:?}", e);
                    panic!("RPC call failed: {:?}", e);
                }
            }
        }
        Ok(patient_registry::Result5::Err(e)) => {
            println!("DEBUG test: failed to retrieve EMRs with error: {:?}", e);
            panic!("Expected success but got error: {}", e)
        }
        Err(e) => {
            println!("DEBUG test: got pocket_ic error: {:?}", e);
            panic!("Expected success but got pocket_ic error")
        }
    }

    // ok now add a third member
    let patient3 = common::Scenario::create_patient(&registries);

    // issue emr for patient3
    let emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "test_key".to_string(),
            value: "test_value".to_string(),
        }],
        user_id: patient3.nik.clone().to_string(),
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

    // add patient3 to group
    let consent_code3 = registries
        .patient
        .create_consent_for_group(
            &registries.ic,
            patient3.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateConsentForGroupRequest {
                nik: patient3.nik.clone().to_string(),
            },
        )
        .unwrap();

    // add patient3 to group
    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id: group_id.clone(),
        group_consent_code: consent_code3.group_consent_code,
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

    // patient1 should NOT have access to patient3's EMR
    match registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient2.principal.clone(),
        PatientCall::Query,
        patient_registry::ViewGroupMemberEmrInformationRequest {
            member_nik: patient3.nik.to_string(),
            group_id: group_id.clone(),
            page: 0,
            limit: 10,
        },
    ){
        Ok(patient_registry::Result5::Err(e)) => {
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Should not have access to patient3's EMR: {}",
                e
            );
        }
        Ok(patient_registry::Result5::Ok(ref emr_info)) => {
            for emr in &emr_info.emrs {
                println!("DEBUG test: emr: {:?}", emr.header.user_id);
            }
            panic!("Should not have access to patient3's EMR");
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // resulting emr from viewing patient1 emr from patient2 should not have patient3's emr
    match registries.patient.view_group_member_emr_information(
        &registries.ic,
        patient1.principal.clone(),
        PatientCall::Query,
        patient_registry::ViewGroupMemberEmrInformationRequest {
            member_nik: patient2.nik.to_string(),
            group_id: group_id.clone(),
            page: 0,
            limit: 10,
        },
    ) {
        Ok(patient_registry::Result5::Ok(emr_info)) => {
            // log the state of the emr list
            for emr in &emr_info.emrs {
                println!("DEBUG test: emr: You should only see 1 EMR here: {:?}", emr.header.user_id);
            }

            assert_eq!(emr_info.emrs.len(), 1, "Should have exactly one EMR");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "User ID should match:"
            );
            assert!(
                !emr_info.emrs[0].header.emr_id.contains(&patient3.nik.to_string()),
                "Patient3's EMR should not be in the list"
            );
        }
        Ok(patient_registry::Result5::Err(e)) => {
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Should not have access to patient3's EMR"
            );
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn test_typical_emr_access_flow() {
    println!("\nDEBUG test: Starting typical EMR access flow test");
    
    // start with base scenario that gives us 3 patients, 1 provider, and a group
    let (registries, provider, patient1, patient2, patient3, group_id) =
        common::Scenario::three_patients_one_provider_one_group();
    
    println!("DEBUG test: Initial setup complete:");
    println!("DEBUG test: - Patient1 NIK: {}", patient1.nik);
    println!("DEBUG test: - Patient2 NIK: {}", patient2.nik);
    println!("DEBUG test: - Patient3 NIK: {}", patient3.nik);
    println!("DEBUG test: - Group ID: {}", group_id);

    // patient2 grants access to patient1
    println!("\nDEBUG test: Step 1 - Patient2 granting access to Patient1");
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
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Access granted successfully"),
        patient_registry::Result_::Err(e) => panic!("Failed to grant access: {}", e),
    }

    // patient1 should now be able to view patient2's EMR
    println!("\nDEBUG test: Step 2 - Testing Patient1's access to Patient2's EMR");
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
            println!("DEBUG test: ✓ Patient1 successfully viewed Patient2's EMR");
            println!("DEBUG test: - Found {} EMR(s)", emr_info.emrs.len());
            println!("DEBUG test: - EMR owner NIK: {}", emr_info.emrs[0].header.user_id);
            assert_eq!(emr_info.emrs.len(), 1, "Should have exactly one EMR");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "EMR should belong to patient2"
            );
        }
        Ok(patient_registry::Result5::Err(e)) => panic!("Failed to view EMR: {}", e),
        Err(e) => panic!("Unexpected error: {}", e),
    }

    // patient2 should NOT be able to view patient1's EMR (one-way grant)
    println!("\nDEBUG test: Step 3 - Testing one-way grant (Patient2 should not access Patient1's EMR)");
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

    match view_result {
        Ok(patient_registry::Result5::Err(e)) => {
            println!("DEBUG test: ✓ Correctly denied Patient2 access to Patient1's EMR");
            println!("DEBUG test: - Error message: {}", e);
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error, got: {}",
                e
            );
        }
        _ => panic!("Expected error for unauthorized EMR access"),
    }

    // verify patient1 can still access patient2's EMR and only sees patient2's EMR
    println!("\nDEBUG test: Step 4 - Verifying Patient1's continued access to Patient2's EMR");
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
            println!("DEBUG test: ✓ Patient1 still has access to Patient2's EMR");
            println!("DEBUG test: - Found {} EMR(s)", emr_info.emrs.len());
            println!("DEBUG test: - EMR owner NIK: {}", emr_info.emrs[0].header.user_id);
            assert_eq!(emr_info.emrs.len(), 1, "Should still see exactly one EMR");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "EMR should still belong to patient2"
            );
        }
        _ => panic!("Should still have access to patient2's EMR"),
    }

    // patient1 should NOT be able to view patient3's EMR
    println!("\nDEBUG test: Step 5 - Testing Patient1's access to Patient3's EMR (should fail)");
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient3.nik.to_string(),
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
        Ok(patient_registry::Result5::Err(e)) => {
            println!("DEBUG test: ✓ Correctly denied access to Patient3's EMR");
            println!("DEBUG test: - Error message: {}", e);
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error, got: {}",
                e
            );
        }
        _ => panic!("Expected error for unauthorized EMR access to patient3"),
    }

    println!("\nDEBUG test: ✓ All test steps completed successfully");

    // Test revocation
    println!("\nDEBUG test: Step 6 - Patient2 revoking access from Patient1");
    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        revokee_nik: patient1.nik.to_string(),
        group_id: group_id.clone(),
    };

    let revoke_result = registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            revoke_access_req,
        )
        .unwrap();

    match revoke_result {
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Access revoked successfully"),
        patient_registry::Result_::Err(e) => panic!("Failed to revoke access: {}", e),
    }

    // Verify Patient1 can no longer access Patient2's EMR
    println!("\nDEBUG test: Step 7 - Verifying Patient1 can no longer access Patient2's EMR");
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
        Ok(patient_registry::Result5::Err(e)) => {
            println!("DEBUG test: ✓ Correctly denied access to Patient2's EMR after revocation");
            println!("DEBUG test: - Error message: {}", e);
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error after revocation, got: {}",
                e
            );
        }
        _ => panic!("Expected error for revoked EMR access to Patient2"),
    }

    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        revokee_nik: patient1.nik.to_string(),
        group_id: group_id.clone(),
    };

    // Test revoking again (should succeed silently)
    println!("\nDEBUG test: Step 8 - Testing double revocation (should succeed)");
    let revoke_result = registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            revoke_access_req,
        )
        .unwrap();

    match revoke_result {
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Second revocation succeeded silently"),
        patient_registry::Result_::Err(e) => panic!("Second revocation should succeed: {}", e),
    }

    println!("\nDEBUG test: ✓ All test steps completed successfully");
}

#[test]
fn test_typical_emr_access_flow_with_new_patient() {
    println!("\nDEBUG test: Starting extended EMR access flow test with new patient");

    // start with base scenario that gives us 3 patients, 1 provider, and a group
    let (registries, provider, patient1, patient2, patient3, group_id) =
        common::Scenario::three_patients_one_provider_one_group();

    println!("DEBUG test: Initial setup complete:");
    println!("DEBUG test: - Patient1 NIK: {}", patient1.nik);
    println!("DEBUG test: - Patient2 NIK: {}", patient2.nik);
    println!("DEBUG test: - Patient3 NIK: {}", patient3.nik);
    println!("DEBUG test: - Group ID: {}", group_id);

    // patient2 grants access to patient1
    println!("\nDEBUG test: Step 1 - Patient2 granting access to Patient1");
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
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Access granted successfully"),
        patient_registry::Result_::Err(e) => panic!("Failed to grant access: {}", e),
    }

    // patient1 should now be able to view patient2's EMR
    println!("\nDEBUG test: Step 2 - Testing Patient1's access to Patient2's EMR");
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
            println!("DEBUG test: ✓ Patient1 successfully viewed Patient2's EMR");
            println!("DEBUG test: - Found {} EMR(s)", emr_info.emrs.len());
            println!("DEBUG test: - EMR owner NIK: {}", emr_info.emrs[0].header.user_id);
            assert_eq!(emr_info.emrs.len(), 1, "Should have exactly one EMR");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "EMR should belong to patient2"
            );
        }
        _ => panic!("Expected access to Patient2's EMR"),
    }

    // patient2 should NOT be able to view patient1's EMR (one-way grant)
    println!("\nDEBUG test: Step 3 - Testing one-way grant (Patient2 should not access Patient1's EMR)");
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

    match view_result {
        Ok(patient_registry::Result5::Err(e)) => {
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error, got: {}",
                e
            );
            println!("DEBUG test: ✓ Correctly denied Patient2 access to Patient1's EMR");
        }
        _ => panic!("Expected error for unauthorized EMR access"),
    }

    // verify patient1 can still access patient2's EMR and only sees patient2's EMR
    println!("\nDEBUG test: Step 4 - Verifying Patient1's continued access to Patient2's EMR");
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
            println!("DEBUG test: ✓ Patient1 still has access to Patient2's EMR");
            println!("DEBUG test: - Found {} EMR(s)", emr_info.emrs.len());
            println!("DEBUG test: - EMR owner NIK: {}", emr_info.emrs[0].header.user_id);
            assert_eq!(emr_info.emrs.len(), 1, "Should still see exactly one EMR");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "EMR should still belong to patient2"
            );
        }
        _ => panic!("Should still have access to patient2's EMR"),
    }

    // Now, create a new patient (patient4) and issue an EMR for them
    println!("\nDEBUG test: Step 5 - Creating Patient4 and issuing EMR");
    let patient4 = common::Scenario::create_patient(&registries);
    let emr_req = provider_registry::IssueEmrRequest {
        emr: vec![provider_registry::EmrFragment {
            key: "patient4_emr".to_string(),
            value: "patient4_value".to_string(),
        }],
        user_id: patient4.nik.clone().to_string(),
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

    // Create consent code for patient4 to be added to the group by patient2
    println!("\nDEBUG test: Step 6 - Patient4 creating consent to join group");
    let consent_code = registries
        .patient
        .create_consent_for_group(
            &registries.ic,
            patient4.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateConsentForGroupRequest {
                nik: patient4.nik.clone().to_string(),
            },
        )
        .unwrap();

    // Add patient4 to the group
    let add_member_req = patient_registry::AddGroupMemberRequest {
        group_id: group_id.clone(),
        group_consent_code: consent_code.group_consent_code,
        relation: Relation::Sibling,
    };

    registries
        .patient
        .add_group_member(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            add_member_req,
        )
        .unwrap();

    // Verify patient4 is in the group
    println!("\nDEBUG test: Step 7 - Verifying Patient4 is in the group");
    let groups = registries
        .patient
        .get_user_groups(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Query,
        )
        .unwrap();

    assert_eq!(groups.groups.len(), 1, "Patient2 should have one group");
    assert_eq!(groups.groups[0].id, group_id, "Group ID should match");

    // Verify that patient1 can still access patient2's EMR
    println!("\nDEBUG test: Step 8 - Verifying Patient1's access to Patient2's EMR after adding Patient4");
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
            assert_eq!(emr_info.emrs.len(), 1, "Should still see exactly one EMR");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient2.nik.to_string(),
                "EMR should still belong to patient2"
            );
        }
        _ => panic!("Should still have access to patient2's EMR"),
    }

    // Verify patient1 cannot access patient4's EMR yet
    println!("\nDEBUG test: Step 9 - Testing Patient1's access to Patient4's EMR (should fail)");
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient4.nik.to_string(),
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
        Ok(patient_registry::Result5::Err(e)) => {
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error, got: {}",
                e
            );
            println!("DEBUG test: ✓ Correctly denied access to Patient4's EMR");
        }
        _ => panic!("Expected error for unauthorized EMR access to patient4"),
    }

    // Now, patient4 grants access to patient1
    println!("\nDEBUG test: Step 10 - Patient4 granting access to Patient1");
    let grant_access_req = patient_registry::GrantGroupAccessRequest {
        group_id: group_id.clone(),
        grantee_nik: patient1.nik.to_string(),
    };

    let grant_result = registries
        .patient
        .grant_group_access(
            &registries.ic,
            patient4.principal.clone(),
            PatientCall::Update,
            grant_access_req,
        )
        .unwrap();

    match grant_result {
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Access granted successfully"),
        patient_registry::Result_::Err(e) => panic!("Failed to grant access: {}", e),
    }

    // Verify patient1 can now access both patient2 and patient4's EMRs
    println!("\nDEBUG test: Step 11 - Verifying Patient1's access to both Patient2's and Patient4's EMRs");
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient4.nik.to_string(),
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
            println!("DEBUG test: ✓ Successfully viewed patient4's EMR information:");
            println!("DEBUG test: - EMR ID: {}", emr_info.emrs[0].header.emr_id);
            println!("DEBUG test: - Provider ID: {}", emr_info.emrs[0].header.provider_id);
            println!("DEBUG test: - User ID: {}", emr_info.emrs[0].header.user_id);
            assert_eq!(emr_info.emrs.len(), 1, "Should see exactly one EMR from Patient4");
            assert_eq!(
                emr_info.emrs[0].header.user_id,
                patient4.nik.to_string(),
                "EMR should belong to patient4"
            );
        }
        _ => panic!("Expected access to Patient4's EMR"),
    }

    println!("\nDEBUG test: ✓ All test steps completed successfully, including revocation testing");

    // Test revocation for both Patient2 and Patient4
    println!("\nDEBUG test: Step 12 - Patient2 and Patient4 revoking access from Patient1");
    
    // Patient2 revokes access
    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        revokee_nik: patient1.nik.to_string(),
        group_id: group_id.clone(),
    };

    let revoke_result = registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            revoke_access_req,
        )
        .unwrap();

    match revoke_result {
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Access revoked successfully by Patient2"),
        patient_registry::Result_::Err(e) => panic!("Failed to revoke access by Patient2: {}", e),
    }

    // Patient4 revokes access
    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        revokee_nik: patient1.nik.to_string(),
        group_id: group_id.clone(),
    };

    let revoke_result = registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient4.principal.clone(),
            PatientCall::Update,
            revoke_access_req,
        )
        .unwrap();

    match revoke_result {
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Access revoked successfully by Patient4"),
        patient_registry::Result_::Err(e) => panic!("Failed to revoke access by Patient4: {}", e),
    }

    // Verify Patient1 can no longer access Patient2's EMR
    println!("\nDEBUG test: Step 13 - Verifying Patient1 can no longer access Patient2's EMR");
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
        Ok(patient_registry::Result5::Err(e)) => {
            println!("DEBUG test: ✓ Correctly denied access to Patient2's EMR after revocation");
            println!("DEBUG test: - Error message: {}", e);
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error after revocation, got: {}",
                e
            );
        }
        _ => panic!("Expected error for revoked EMR access to Patient2"),
    }

    // Verify Patient1 can no longer access Patient4's EMR
    println!("\nDEBUG test: Step 14 - Verifying Patient1 can no longer access Patient4's EMR");
    let view_request = patient_registry::ViewGroupMemberEmrInformationRequest {
        member_nik: patient4.nik.to_string(),
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
        Ok(patient_registry::Result5::Err(e)) => {
            println!("DEBUG test: ✓ Correctly denied access to Patient4's EMR after revocation");
            println!("DEBUG test: - Error message: {}", e);
            assert!(
                e.contains("[ERR_ACCESS_NOT_GRANTED]"),
                "Expected access not granted error after revocation, got: {}",
                e
            );
        }
        _ => panic!("Expected error for revoked EMR access to Patient4"),
    }

    println!("\nDEBUG test: ✓ All test steps completed successfully, including revocation testing");

    // Test error cases for revoke_group_access
    println!("\nDEBUG test: Step 15 - Testing error cases for revoke_group_access");


    // Test case 1: Create a new group and attempt to revoke access between users in different groups
    println!("\nDEBUG test: Testing revocation between users in different groups");
    
    // Create a new group
    let new_group_response = registries
        .patient
        .create_group(
            &registries.ic,
            patient3.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateGroupRequest {
                name: "different_group".to_string(),
            },
        )
        .unwrap();

    let different_group_id = match new_group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        _ => panic!("Failed to create different group"),
    };

    // Attempt to revoke access using the different group
    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        revokee_nik: patient1.nik.to_string(),
        group_id: different_group_id.clone(),
    };

    let revoke_result = registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            revoke_access_req,
        )
        .unwrap();

    match revoke_result {
        patient_registry::Result_::Err(e) => {
            println!("DEBUG test: ✓ Correctly failed with users in different groups");
            println!("DEBUG test: - Error message: {}", e);
            assert!(
                e.contains("[ERR_NOT_GROUP_MEMBERS]"),
                "Expected not group members error, got: {}",
                e
            );
        }
        _ => panic!("Expected error for users in different groups"),
    }

    // Test case 2: Attempt to revoke access twice
    println!("\nDEBUG test: Testing revocation twice");
    let revoke_access_req = patient_registry::RevokeGroupAccessRequest {
        revokee_nik: patient1.nik.to_string(),
        group_id: group_id.clone(),
    };

    let revoke_result = registries
        .patient
        .revoke_group_access(
            &registries.ic,
            patient2.principal.clone(),
            PatientCall::Update,
            revoke_access_req,
        )
        .unwrap();

    match revoke_result {
        patient_registry::Result_::Ok => println!("DEBUG test: ✓ Access revoked successfully"),
        patient_registry::Result_::Err(e) => panic!("Failed to revoke access: {}", e),
    }

    println!("\nDEBUG test: ✓ All error case tests completed successfully");
}
