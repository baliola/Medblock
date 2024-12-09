use integration_tests::declarations::patient_registry::pocket_ic_bindings::Call as PatientCall;
use integration_tests::declarations::patient_registry::{
    self, AddGroupMemberRequest, CreateGroupRequest, GetGroupDetailsRequest, Relation,
};

use crate::common;

/// TEST THAT GROUP DETAILS DOES NOT RETURN EMPTY GROUP DETAILS
///
/// *PREREQUISITES*:
/// - One registered admin
/// - One registered provider
/// - One registered patient with EMR from the provider above
///
/// *FLOW BEING TESTED*:
/// 1. Create group
/// 2. Get group details
#[test]
fn test_group_details_does_not_return_empty_group_details() {
    let (registries, _provider, patient) = common::Scenario::one_provider_one_patient();

    // step 1. create group
    let create_group_req = patient_registry::CreateGroupRequest {
        name: "Test Group".to_string(),
    };

    let group_response = registries
        .patient
        .create_group(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Update,
            create_group_req,
        )
        .unwrap();

    // verify group was created
    let group_id = match group_response {
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    // step 2. get group details
    let details = registries
        .patient
        .get_group_details(
            &registries.ic,
            patient.principal.clone(),
            PatientCall::Query,
            GetGroupDetailsRequest {
                group_id,
                page: 0,
                limit: 10,
            },
        )
        .unwrap();

    // verify group details are not empty
    match details {
        patient_registry::Result4::Ok(response) => assert!(!response.details_of_members.is_empty()),
        patient_registry::Result4::Err(e) => panic!("Failed to get group details: {}", e),
    }
}

/// TEST GROUP DETAILS INCLUDES LEADER
///
/// *PREREQUISITES*:
/// - One registered admin
/// - One registered provider
/// - One registered patient with EMR from the provider above
///
/// *FLOW BEING TESTED*:
/// 1. Create group
/// 2. Add member to group
/// 3. Get group details
#[test]
fn test_group_details_includes_leader() {
    let (registries, _provider, leader, member1) =
        common::Scenario::one_provider_two_patient_with_emrs();

    // step 1.create group with leader
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    // step 2. add members to group
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

    // verify group has 2 members
    let groups = registries
        .patient
        .get_user_groups(&registries.ic, leader.principal.clone(), PatientCall::Query)
        .unwrap();

    assert_eq!(groups.groups[0].members.len(), 2);
    assert_eq!(groups.groups[0].id, group_id);

    // step 3. get group details with pagination
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

    // verify group details
    match details {
        patient_registry::Result4::Ok(response) => {
            // Verify leader is first in the list
            assert!(!response.details_of_members.is_empty());
            let first_member = &response.details_of_members[0];
            assert_eq!(first_member.patient_info.nik, leader.nik.to_string());
            assert_eq!(
                response.leader_name,
                match &first_member.patient_info.info {
                    patient_registry::Patient::V1(v1) => v1.name.clone(),
                }
            );

            // leader should be counted in member_count
            assert_eq!(response.member_count, 2);
        }
        _ => panic!("Failed to get group details"),
    }
}

/// TEST GROUP DETAILS INCLUDES MEMBER ROLES
///
/// *PREREQUISITES*:
/// - One registered admin
/// - One registered provider
/// - Two registered patients with EMR from the provider above
///
/// *FLOW BEING TESTED*:
/// 1. Create group
/// 2. Add members to group with different roles
/// 3. Get group details
#[test]
fn test_group_details_member_roles() {
    let (registries, _provider, leader, member1) =
        common::Scenario::one_provider_two_patient_with_emrs();

    // step 1. create group
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

    // verify this step worked
    let group_id = match group_response {
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
    };

    // step 2. add members with different roles
    let members = vec![(member1, Relation::Child)];

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

    // verify group has 2 members
    let groups = registries
        .patient
        .get_user_groups(&registries.ic, leader.principal.clone(), PatientCall::Query)
        .unwrap();

    assert_eq!(groups.groups.len(), 1);
    assert_eq!(groups.groups[0].id, group_id);

    // step 3. get group details
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

    // verify group details
    match details {
        patient_registry::Result4::Ok(details) => {
            // verify all roles are correct
            assert_eq!(details.details_of_members.len(), 2); // leader + 1 member

            // verify leader role
            let leader_detail = details
                .details_of_members
                .iter()
                .find(|d| d.patient_info.nik == leader.nik.to_string())
                .expect("Leader should be in details");

            // verify role matches (using string comparison)
            match leader_detail.role {
                Relation::Parent => (),
                _ => panic!("Leader role is not Parent"),
            }

            // verify member roles
            for detail in details.details_of_members.iter() {
                if detail.patient_info.nik == leader.nik.to_string() {
                    match detail.role {
                        Relation::Parent => (),
                        _ => panic!("Leader role is not Parent"),
                    }
                } else {
                    match detail.role {
                        Relation::Child => (),
                        _ => panic!("Unexpected role"),
                    }
                }
            }
        }
        patient_registry::Result4::Err(e) => panic!("Failed to get group details: {}", e),
    }
}
