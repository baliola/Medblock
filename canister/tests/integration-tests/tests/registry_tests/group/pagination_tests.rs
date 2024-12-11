use integration_tests::declarations::{
    patient_registry::{
        self, pocket_ic_bindings::Call as PatientCall, ClaimConsentRequest, KycStatus, Relation,
    },
    provider_registry::{self, pocket_ic_bindings::Call as ProviderCall},
};

use crate::common;

/// TEST GROUP DETAILS PAGINATION
///
#[test]
fn test_get_group_details() {
    let (registries, provider, patient1, patient2) =
        common::Scenario::one_provider_two_patient_with_emrs();

    // create and claim consent for patient1
    let patient_1_consent = registries
        .patient
        .create_consent(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    // claim the consent
    registries
        .patient
        .claim_consent(
            &registries.ic,
            provider.0.clone(),
            PatientCall::Update,
            ClaimConsentRequest {
                code: patient_1_consent.code.clone(),
            },
        )
        .unwrap();

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

    // claim the consent
    let patient_2_consent = registries
        .patient
        .create_consent_for_group(
            &registries.ic,
            provider.0.clone(),
            PatientCall::Update,
            patient_registry::CreateConsentForGroupRequest {
                nik: patient2.nik.clone().to_string(),
            },
        )
        .unwrap();

    let add_member_req = patient_registry::AddGroupMemberRequest {
        relation: Relation::Sibling,
        group_id: group_id.clone(),
        group_consent_code: patient_2_consent.group_consent_code,
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

    // get group details
    let details_req = patient_registry::GetGroupDetailsRequest {
        group_id: group_id.clone(),
        page: 0,
        limit: 10,
    };

    // get group details
    let details_req_2 = patient_registry::GetGroupDetailsRequest {
        group_id: group_id.clone(),
        page: 0,
        limit: 10,
    };

    let details = registries
        .patient
        .get_group_details(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            details_req,
        )
        .unwrap();

    let ok_details = match details {
        patient_registry::Result3::Ok(response) => response,
        patient_registry::Result3::Err(e) => panic!("Failed to get group details: {}", e),
    };

    let details = registries
        .patient
        .get_group_details(
            &registries.ic,
            patient1.principal.clone(),
            PatientCall::Query,
            details_req_2,
        )
        .unwrap();

    println!("patient 1: {:?}", patient1.nik.to_string());
    println!("patient 2: {:?}", patient2.nik.to_string());
    println!("details: {:?}", ok_details.group_details[0].nik);
    println!("details: {:?}", ok_details.group_details[1].nik);
    println!("details: {:?}", ok_details.leader_name);
    println!("details: {:?}", ok_details.member_count);
    println!("details: {:?}", ok_details.total_pages);

    match details {
        patient_registry::Result3::Ok(response) => {
            assert_eq!(response.member_count, 2);
            assert_eq!(response.total_pages, 1);
            assert_eq!(response.group_details[0].nik, patient1.nik.to_string());
            assert_eq!(response.group_details[1].nik, patient2.nik.to_string());
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get group details: {}", e),
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
            .create_consent_for_group(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                patient_registry::CreateConsentForGroupRequest {
                    nik: patient.nik.clone().to_string(),
                },
            )
            .unwrap();

        let add_member_req = patient_registry::AddGroupMemberRequest {
            group_id: group_id.clone(),
            group_consent_code: consent_code.group_consent_code,
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
        group_id: group_id.clone(),
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
            // Leader should always be in first page
            assert!(!response.group_details.is_empty());
            assert_eq!(response.group_details[0].nik, patient1.nik.to_string());
            assert!(!response.leader_name.is_empty());

            assert_eq!(response.group_details.len(), 2);
            assert_eq!(response.member_count, 6); // 5 members + 1 leader
            assert_eq!(response.total_pages, 3);

            // check second page
            let second_page_req = patient_registry::GetGroupDetailsRequest {
                group_id: group_id.clone(),
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
fn test_group_leader_transfer() {
    let (registries, _provider, initial_leader, member) =
        common::Scenario::one_provider_two_patient_with_emrs();

    // step 1. create group (should have leader as first member)
    let create_group_req = patient_registry::CreateGroupRequest {
        name: "Test Leader Transfer".to_string(),
    };

    let group_response = registries
        .patient
        .create_group(
            &registries.ic,
            initial_leader.principal.clone(),
            PatientCall::Update,
            create_group_req,
        )
        .unwrap();

    let group_id = match group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    // step 2. verify initial state (only leader)
    let initial_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            initial_leader.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id: group_id.clone(),
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match initial_details {
        patient_registry::Result3::Ok(response) => {
            assert_eq!(
                response.member_count, 1,
                "New group should only have leader"
            );
            assert_eq!(response.group_details.len(), 1);
            assert_eq!(response.leader_name, response.group_details[0].name);
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get initial details: {}", e),
    }

    // step 3. add a member
    let consent = registries
        .patient
        .create_consent_for_group(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Update,
            patient_registry::CreateConsentForGroupRequest {
                nik: member.nik.clone().to_string(),
            },
        )
        .unwrap();

    registries
        .patient
        .add_group_member(
            &registries.ic,
            initial_leader.principal.clone(),
            PatientCall::Update,
            patient_registry::AddGroupMemberRequest {
                group_id: group_id.clone(),
                group_consent_code: consent.group_consent_code,
                relation: Relation::Sibling,
            },
        )
        .unwrap();

    // step 4. leader leaves group (should transfer leadership to member)
    registries
        .patient
        .leave_group(
            &registries.ic,
            initial_leader.principal.clone(),
            PatientCall::Update,
            patient_registry::LeaveGroupRequest {
                group_id: group_id.clone(),
            },
        )
        .unwrap();

    // verify leader left group by counting amount
    let leader_left_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id: group_id.clone(),
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match leader_left_details {
        patient_registry::Result3::Ok(response) => {
            assert_eq!(
                response.member_count, 1,
                "Leader should have left the group and only 1 member should be left"
            );
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get leader left details: {}", e),
    }

    // step 5. new leader should now be that member
    let new_leader_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id: group_id.clone(),
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match new_leader_details {
        patient_registry::Result3::Ok(response) => {
            assert_eq!(response.group_details.len(), 1);
            assert_eq!(response.leader_name, response.group_details[0].name);
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get new leader details: {}", e),
    }

    // step 6. new leader leaves group (should dissolve the group)
    registries
        .patient
        .leave_group(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Update,
            patient_registry::LeaveGroupRequest {
                group_id: group_id.clone(),
            },
        )
        .unwrap();

    // verify group is dissolved
    let final_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id: group_id.clone(),
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match final_details {
        patient_registry::Result3::Ok(_) => panic!("Group should be dissolved when leader leaves"),
        patient_registry::Result3::Err(e) => {
            assert!(
                e.contains("Group not found") || e.contains("Group does not exist"),
                "Expected group not found error, got: {}",
                e
            )
        }
    }
}

#[test]
fn test_group_dissolution() {
    let (registries, leader, _) = common::Scenario::one_admin_one_patient();

    // create group
    let create_group_req = patient_registry::CreateGroupRequest {
        name: "Test Dissolution".to_string(),
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

    // verify group exists
    let initial_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id: group_id.clone(),
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match initial_details {
        patient_registry::Result3::Ok(_) => (),
        patient_registry::Result3::Err(e) => panic!("Failed to get initial details: {}", e),
    }

    // leader (last member) leaves group
    registries
        .patient
        .leave_group(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            patient_registry::LeaveGroupRequest {
                group_id: group_id.clone(),
            },
        )
        .unwrap();

    // verify group is dissolved (should return error)
    let final_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match final_details {
        patient_registry::Result3::Ok(_) => panic!("Group should be dissolved"),
        patient_registry::Result3::Err(e) => {
            assert!(
                e.contains("Group not found") || e.contains("Group does not exist"),
                "Expected group not found error, got: {}",
                e
            )
        }
    }
}
