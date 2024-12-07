use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::{self, KycStatus, Relation},
    provider_registry::pocket_ic_bindings::Call as ProviderCall,
    provider_registry::{self},
};

use crate::common;

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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
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
        patient_registry::Result4::Ok(response) => {
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
                patient_registry::Result4::Ok(paginated) => {
                    assert_eq!(paginated.group_details.len(), 2);
                    assert_eq!(paginated.member_count, 4);
                    assert_eq!(paginated.total_pages, 2);
                }
                patient_registry::Result4::Err(e) => {
                    panic!("Failed to get paginated details: {}", e)
                }
            }
        }
        patient_registry::Result4::Err(e) => panic!("Failed to get group details: {}", e),
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
        patient_registry::Result4::Ok(_) => panic!("Unauthorized access should fail"),
        patient_registry::Result4::Err(e) => {
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
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
        patient_registry::Result4::Ok(response) => {
            // Leader should always be in first page
            assert!(!response.group_details.is_empty());
            assert_eq!(response.group_details[0].nik, patient1.nik.to_string());
            assert!(!response.leader_name.is_empty());

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
                patient_registry::Result4::Ok(second_response) => {
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
                patient_registry::Result4::Err(e) => panic!("Failed to get second page: {}", e),
            }
        }
        patient_registry::Result4::Err(e) => panic!("Failed to get first page: {}", e),
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    // step 2. verify initial state (only leader)
    let initial_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            initial_leader.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match initial_details {
        patient_registry::Result4::Ok(response) => {
            assert_eq!(
                response.member_count, 1,
                "New group should only have leader"
            );
            assert_eq!(response.group_details.len(), 1);
            assert_eq!(response.leader_name, response.group_details[0].name);
        }
        patient_registry::Result4::Err(e) => panic!("Failed to get initial details: {}", e),
    }

    // step 3. add a member
    let consent = registries
        .patient
        .create_consent(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    registries
        .patient
        .add_group_member(
            &registries.ic,
            initial_leader.principal.clone(),
            PatientCall::Update,
            patient_registry::AddGroupMemberRequest {
                group_id,
                consent_code: consent.code,
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
            patient_registry::LeaveGroupRequest { group_id },
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
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match leader_left_details {
        patient_registry::Result4::Ok(response) => {
            assert_eq!(
                response.member_count, 1,
                "Leader should have left the group and only 1 member should be left"
            );
        }
        patient_registry::Result4::Err(e) => panic!("Failed to get leader left details: {}", e),
    }

    // step 5. new leader should now be that member
    let new_leader_details = registries
        .patient
        .get_group_details(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Query,
            patient_registry::GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match new_leader_details {
        patient_registry::Result4::Ok(response) => {
            assert_eq!(response.group_details.len(), 1);
            assert_eq!(response.leader_name, response.group_details[0].name);
        }
        patient_registry::Result4::Err(e) => panic!("Failed to get new leader details: {}", e),
    }

    // step 6. new leader leaves group (should dissolve the group)
    registries
        .patient
        .leave_group(
            &registries.ic,
            member.principal.clone(),
            PatientCall::Update,
            patient_registry::LeaveGroupRequest { group_id },
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
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match final_details {
        patient_registry::Result4::Ok(_) => panic!("Group should be dissolved when leader leaves"),
        patient_registry::Result4::Err(e) => {
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    // verify group exists
    let initial_details = registries
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

    match initial_details {
        patient_registry::Result4::Ok(_) => (),
        patient_registry::Result4::Err(e) => panic!("Failed to get initial details: {}", e),
    }

    // leader (last member) leaves group
    registries
        .patient
        .leave_group(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            patient_registry::LeaveGroupRequest { group_id },
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
        patient_registry::Result4::Ok(_) => panic!("Group should be dissolved"),
        patient_registry::Result4::Err(e) => {
            assert!(
                e.contains("Group not found") || e.contains("Group does not exist"),
                "Expected group not found error, got: {}",
                e
            )
        }
    }
}