use integration_tests::declarations::provider_registry::pocket_ic_bindings::Call as ProviderCall;
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
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
        patient_registry::Result2::Ok(nik) => {
            assert_eq!(nik, patient2.nik.to_string());
        }
        patient_registry::Result2::Err(e) => panic!("Failed to claim consent: {}", e),
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
        patient_registry::Result2::Ok(_) => panic!("Should not succeed"),
        patient_registry::Result2::Err(e) => assert!(e.contains("Consent already claimed")),
    }
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

    let result = registry
        .provider
        .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
        .unwrap();

    assert!(
        result.emr_header.emr_id.len() > 0,
        "EMR ID should not be empty"
    );

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
                _type: None,
            },
        )
        .unwrap();

    assert_eq!(
        search_result.patient_info.nik,
        patient.nik.clone().to_string()
    );
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
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
        matches!(result, patient_registry::Result4::Err(_)),
        "group should no longer exist"
    );
}

#[test]
fn test_group_access_cleanup() {
    let (registries, provider, patient1) = common::Scenario::one_provider_one_patient();
    let patient2 = common::Scenario::create_patient(&registries);

    // issue EMRs for both patients
    let emr_req = integration_tests::declarations::provider_registry::IssueEmrRequest {
        emr: vec![
            integration_tests::declarations::provider_registry::EmrFragment {
                key: "test_key".to_string(),
                value: "test_value".to_string(),
            },
        ],
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

    let emr_req = integration_tests::declarations::provider_registry::IssueEmrRequest {
        emr: vec![
            integration_tests::declarations::provider_registry::EmrFragment {
                key: "test_key2".to_string(),
                value: "test_value2".to_string(),
            },
        ],
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
        patient_registry::Result3::Ok(response) => response.group_id,
        patient_registry::Result3::Err(e) => panic!("Failed to create group: {}", e),
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
        matches!(result, patient_registry::Result5::Ok(_)),
        "Patient2 should be able to view Patient1's EMR initially. Got error: {:?}",
        if let patient_registry::Result5::Err(e) = result {
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
        matches!(result, patient_registry::Result5::Err(_)),
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
        matches!(result, patient_registry::Result5::Err(_)),
        "Patient2 should not be able to view Patient1's EMR after leaving"
    );
}
