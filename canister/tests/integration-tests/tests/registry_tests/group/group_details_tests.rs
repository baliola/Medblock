use integration_tests::declarations::patient_registry::pocket_ic_bindings::Call as PatientCall;
use integration_tests::declarations::patient_registry::{
    self, AddGroupMemberRequest, CreateGroupRequest, GetGroupDetailsRequest, Relation,
};

use crate::common;

#[test]
fn test_group_details_includes_leader() {
    let (registries, leader, _) = common::Scenario::one_admin_one_patient();
    let member1 = common::Scenario::create_patient(&registries);
    let member2 = common::Scenario::create_patient(&registries);

    // create group with leader
    let create_group_req = patient_registry::CreateGroupRequest {
        name: "Test Group".to_string(),
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

    // add members to group
    let member1_consent = registries
        .patient
        .create_consent(
            &registries.ic,
            member1.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    registries
        .patient
        .add_group_member(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            AddGroupMemberRequest {
                group_id,
                consent_code: member1_consent.code,
                relation: Relation::Child,
            },
        )
        .unwrap();

    let member2_consent = registries
        .patient
        .create_consent(
            &registries.ic,
            member2.principal.clone(),
            PatientCall::Update,
        )
        .unwrap();

    registries
        .patient
        .add_group_member(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            AddGroupMemberRequest {
                group_id,
                consent_code: member2_consent.code,
                relation: Relation::Sibling,
            },
        )
        .unwrap();

    // get group details with pagination
    let details = registries
        .patient
        .get_group_details(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Query,
            GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match details {
        patient_registry::Result3::Ok(details) => {
            // verify leader is included in the first page
            let leader_detail = details
                .group_details
                .iter()
                .find(|detail| detail.nik == leader.nik.to_string())
                .expect("Leader should be included in group details");

            // verify role matches (using string comparison since Relation might not implement PartialEq)
            match leader_detail.role {
                Relation::Parent => (),
                _ => panic!("Leader role is not Parent"),
            }
            assert_eq!(details.group_details.len(), 3); // leader + 2 members
            assert_eq!(details.member_count, 3);

            // verify leader appears first in the list
            assert_eq!(details.group_details[0].nik, leader.nik.to_string());

            // test pagination - second page should not include leader
            let page_2 = registries
                .patient
                .get_group_details(
                    &registries.ic,
                    leader.principal.clone(),
                    PatientCall::Query,
                    GetGroupDetailsRequest {
                        group_id,
                        page: 1,
                        limit: 1,
                    },
                )
                .unwrap();

            match page_2 {
                patient_registry::Result3::Ok(page_2) => {
                    assert_eq!(page_2.group_details.len(), 1);
                    assert_ne!(page_2.group_details[0].nik, leader.nik.to_string());
                }
                patient_registry::Result3::Err(e) => panic!("Failed to get second page: {}", e),
            }
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get group details: {}", e),
    }
}

#[test]
fn test_group_details_member_roles() {
    let (registries, leader, _) = common::Scenario::one_admin_one_patient();
    let child = common::Scenario::create_patient(&registries);
    let sibling = common::Scenario::create_patient(&registries);
    let spouse = common::Scenario::create_patient(&registries);

    // create group
    let group_response = registries
        .patient
        .create_group(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Update,
            CreateGroupRequest {
                name: "Family Group".to_string(),
            },
        )
        .unwrap();

    let group_id = match group_response {
        patient_registry::Result2::Ok(response) => response.group_id,
        patient_registry::Result2::Err(e) => panic!("Failed to create group: {}", e),
    };

    // add members with different roles
    let members = vec![
        (child, Relation::Child),
        (sibling, Relation::Sibling),
        (spouse, Relation::Spouse),
    ];

    for (member, relation) in members {
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
                leader.principal.clone(),
                PatientCall::Update,
                AddGroupMemberRequest {
                    group_id,
                    consent_code: consent.code,
                    relation,
                },
            )
            .unwrap();
    }

    // get group details
    let details = registries
        .patient
        .get_group_details(
            &registries.ic,
            leader.principal.clone(),
            PatientCall::Query,
            GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    match details {
        patient_registry::Result3::Ok(details) => {
            // verify all roles are correct
            assert_eq!(details.group_details.len(), 4); // leader + 3 members

            // verify leader role
            let leader_detail = details
                .group_details
                .iter()
                .find(|d| d.nik == leader.nik.to_string())
                .expect("Leader should be in details");

            // verify role matches (using string comparison)
            match leader_detail.role {
                Relation::Parent => (),
                _ => panic!("Leader role is not Parent"),
            }

            // verify member roles
            for detail in details.group_details.iter() {
                if detail.nik == leader.nik.to_string() {
                    match detail.role {
                        Relation::Parent => (),
                        _ => panic!("Leader role is not Parent"),
                    }
                } else {
                    match detail.role {
                        Relation::Child
                        | Relation::Sibling
                        | Relation::Spouse
                        | Relation::Other => (),
                        _ => panic!("Unexpected role"),
                    }
                }
            }
        }
        patient_registry::Result3::Err(e) => panic!("Failed to get group details: {}", e),
    }
}
