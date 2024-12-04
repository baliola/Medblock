use candid::Principal;
use integration_tests::declarations::{
    patient_registry::pocket_ic_bindings::Call as PatientCall,
    patient_registry::{self, Relation},
};

use crate::common;

#[test]
fn test_group_creation_and_emr_access() {
    let (registries, patient1, _) = common::Scenario::one_admin_one_patient();
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
