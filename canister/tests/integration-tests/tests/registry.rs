use std::{ str::FromStr, time::Duration };

use candid::Principal;
use integration_tests::declarations::{
    patient_registry,
    provider_registry::{
        pocket_ic_bindings::Call,
        ProviderInfoRequest,
        RegisternewProviderRequest,
    },
};

use integration_tests::declarations::patient_registry::KycStatus;
use integration_tests::declarations::patient_registry::pocket_ic_bindings::Call as PatientCall;
use integration_tests::declarations::provider_registry::pocket_ic_bindings::Call as ProviderCall;

mod common;

mod test {
    use integration_tests::declarations::{
        self,
        patient_registry::{
            ActivityType,
            ClaimConsentRequest,
            EmrListPatientRequest,
            SearchPatientRequest,
        },
        provider_registry::{ EmrFragment, IssueEmrRequest, UpdateEmrRequest },
    };

    use super::*;
    #[test]
    fn register_provider() {
        let registries = common::prepare();
        let display = String::from("Dr. John Doe").to_ascii_lowercase();
        let address = String::from("1234 Elm St").to_ascii_lowercase();

        let arg = RegisternewProviderRequest {
            provider_principal: Principal::anonymous(),
            display_name: display.clone(),
            address: address.clone(),
        };

        registries.provider
            .register_new_provider(&registries.ic, registries.controller.clone(), Call::Update, arg)
            .unwrap();

        let arg = ProviderInfoRequest {
            provider: vec![Principal::anonymous()],
        };

        let result = registries.provider
            .get_provider_info_with_principal(
                &registries.ic,
                registries.controller.clone(),
                Call::Query,
                arg
            )
            .unwrap();

        match result.providers.first().unwrap() {
            integration_tests::declarations::provider_registry::Provider::V1(provider) => {
                assert_eq!(provider.display_name, display);
                assert_eq!(provider.address, address);
            }
        }
    }

    #[test]
    fn register_patient() {
        let registries = common::prepare();
        let display = String::from("John Doe").to_ascii_lowercase();
        let address = String::from("1234 Elm St").to_ascii_lowercase();

        let nik = canister_common::common::H256
            ::from_str("3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709")
            .unwrap();

        let arg = patient_registry::RegisterPatientRequest {
            nik: nik.to_string(),
        };

        let patient_principal = common::random_identity();

        registries.patient
            .register_patient(
                &registries.ic,
                patient_principal.clone(),
                patient_registry::pocket_ic_bindings::Call::Update,
                arg
            )
            .unwrap();

        let arg = patient_registry::UpdateInitialPatientInfoRequest {
            info: patient_registry::V1 {
                name: display.clone(),
                martial_status: "married".to_string(),
                place_of_birth: "Jakarta".to_ascii_lowercase(),
                address,
                gender: "men".to_ascii_lowercase(),
                date_of_birth: "1990-01-01".to_string(),
                kyc_status: KycStatus::Pending,
                kyc_date: "2024-01-01".to_string(),
            },
        };

        registries.patient
            .update_initial_patient_info(
                &registries.ic,
                patient_principal.clone(),
                patient_registry::pocket_ic_bindings::Call::Update,
                arg
            )
            .unwrap();

        let result = registries.patient
            .get_patient_info(
                &registries.ic,
                patient_principal.clone(),
                patient_registry::pocket_ic_bindings::Call::Query
            )
            .unwrap();

        assert_eq!(result.nik, nik.to_string());
    }

    #[test]
    pub fn test_issued_update() {
        let (registry, provider, patient) = common::Scenario::one_provider_one_patient();

        let arg = IssueEmrRequest {
            emr: vec![EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
            user_id: patient.nik.clone().to_string(),
        };

        let response = registry.provider
            .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
            .unwrap();

        let emr = registry.patient
            .emr_list_patient(
                &registry.ic,
                patient.principal.clone(),
                PatientCall::Query,
                EmrListPatientRequest {
                    page: 0,
                    limit: 10,
                }
            )
            .unwrap();

        assert!(emr.emrs.len() == 1);

        let old_emr = emr.emrs.first().unwrap();

        // advance time
        registry.ic.advance_time(Duration::from_secs(60));

        // update emr
        registry.provider
            .update_emr(
                &registry.ic,
                provider.0.clone(),
                ProviderCall::Update,
                declarations::provider_registry::UpdateEmrRequest {
                    fields: vec![
                        EmrFragment { key: "key".to_string(), value: "new value".to_string() },
                        EmrFragment { key: "new key".to_string(), value: "new value".to_string() }
                    ],
                    header: declarations::provider_registry::EmrHeader {
                        provider_id: response.emr_header.provider_id.clone(),
                        user_id: response.emr_header.user_id.clone(),
                        emr_id: response.emr_header.emr_id.clone(),
                        registry_id: response.emr_header.registry_id.clone(),
                    },
                }
            )
            .unwrap();

        let emr_list = registry.patient
            .emr_list_patient(
                &registry.ic,
                patient.principal.clone(),
                PatientCall::Query,
                EmrListPatientRequest {
                    page: 0,
                    limit: 10,
                }
            )
            .unwrap();

        let emr = emr_list.emrs.first().unwrap();

        assert!(emr_list.emrs.len() == 1);
        assert!(emr.status.updated_at > old_emr.status.updated_at);

        let emr = registry.patient
            .read_emr_by_id(
                &registry.ic,
                patient.principal.clone(),
                PatientCall::Query,
                declarations::patient_registry::ReadEmrByIdRequest {
                    provider_id: emr.header.provider_id.clone(),
                    emr_id: emr.header.emr_id.clone(),
                    registry_id: emr.header.registry_id.clone(),
                }
            )
            .unwrap();

        // no partial eq is implemented for auto generated types, so we need to check manually
        let mut key1 = false;
        let mut key2 = false;

        for emr in emr.emr.body {
            if emr.key == "key" {
                assert_eq!(emr.value, "new value");
                key1 = true;
            }

            if emr.key == "new key" {
                assert_eq!(emr.value, "new value");
                key2 = true;
            }
        }

        assert!(key1, "key 1 does not updated");
        assert!(key2, "key 2 does not updated");
    }

    #[test]
    fn test_search_user() {
        let (registry, provider, patient) = common::Scenario::one_provider_one_patient();

        let arg = IssueEmrRequest {
            emr: vec![EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
            user_id: patient.nik.clone().to_string(),
        };

        let response = registry.provider
            .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
            .unwrap();

        let result = registry.patient
            .create_consent(&registry.ic, patient.principal.clone(), PatientCall::Update)
            .unwrap();

        registry.patient
            .claim_consent(
                &registry.ic,
                provider.0.clone(),
                PatientCall::Update,
                ClaimConsentRequest {
                    code: result.code.clone(),
                }
            )
            .unwrap();

        let search_result = registry.patient
            .search_patient(
                &registry.ic,
                provider.0.clone(),
                PatientCall::Query,
                SearchPatientRequest {
                    nik: patient.nik.clone().to_string(),
                }
            )
            .unwrap();

        assert_eq!(search_result.patient_info.nik, patient.nik.clone().to_string());
    }

    #[test]
    fn test_2_emr() {
        let (registry, provider, patient) = common::Scenario::one_provider_one_patient();

        let arg = IssueEmrRequest {
            emr: vec![EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
            user_id: patient.nik.clone().to_string(),
        };

        let response = registry.provider
            .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
            .unwrap();

        let arg = IssueEmrRequest {
            emr: vec![EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
            user_id: patient.nik.clone().to_string(),
        };

        let response = registry.provider
            .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
            .unwrap();

        let emr = registry.patient
            .emr_list_patient(
                &registry.ic,
                patient.principal.clone(),
                PatientCall::Query,
                EmrListPatientRequest {
                    page: 0,
                    limit: 10,
                }
            )
            .unwrap();

        assert!(emr.emrs.len() == 2);
    }

    #[test]
    fn test_add_emr_after_sharing_session() {
        let (registry, provider, patient) = common::Scenario::one_provider_one_patient();

        let arg = IssueEmrRequest {
            emr: vec![EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
            user_id: patient.nik.clone().to_string(),
        };

        let response = registry.provider
            .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
            .unwrap();

        let result = registry.patient
            .create_consent(&registry.ic, patient.principal.clone(), PatientCall::Update)
            .unwrap();

        registry.patient
            .claim_consent(
                &registry.ic,
                provider.0.clone(),
                PatientCall::Update,
                ClaimConsentRequest {
                    code: result.code.clone(),
                }
            )
            .unwrap();

        let arg = IssueEmrRequest {
            emr: vec![EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
            user_id: patient.nik.clone().to_string(),
        };

        let response = registry.provider
            .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
            .unwrap();

        let emr = registry.patient
            .emr_list_patient(
                &registry.ic,
                patient.principal.clone(),
                PatientCall::Query,
                EmrListPatientRequest {
                    page: 0,
                    limit: 10,
                }
            )
            .unwrap();

        assert!(emr.emrs.len() == 2);
    }

    #[test]
    fn test_log() {
        let scenario = common::Scenario::one_provider_one_patient_with_one_emr();

        let response = scenario.registries.patient
            .get_logs(
                &scenario.registries.ic,
                scenario.patient.principal.clone(),
                PatientCall::Query
            )
            .unwrap();

        assert!(response.logs.len() == 0);

        let response = scenario.registries.patient
            .create_consent(
                &scenario.registries.ic,
                scenario.patient.principal.clone(),
                PatientCall::Update
            )
            .unwrap();

        let code = response.code;

        let response = scenario.registries.patient
            .claim_consent(
                &scenario.registries.ic,
                scenario.provider.0.clone(),
                PatientCall::Update,
                ClaimConsentRequest { code: code.clone() }
            )
            .unwrap();

        let session_id = response.session_id;

        let response = scenario.registries.patient
            .get_logs(
                &scenario.registries.ic,
                scenario.patient.principal.clone(),
                PatientCall::Query
            )
            .unwrap();

        assert!(response.logs.len() == 1);

        let log = response.logs.first().unwrap();

        match log.activity_type {
            ActivityType::Updated => unreachable!("should not have updated log"),
            ActivityType::Accessed => (),
            ActivityType::Revoked => unreachable!("should not have revoked log"),
        }

        scenario.registries.provider
            .update_emr(
                &scenario.registries.ic,
                scenario.provider.0.clone(),
                ProviderCall::Update,
                UpdateEmrRequest {
                    fields: vec![EmrFragment {
                        key: "new key".to_string(),
                        value: "new value".to_string(),
                    }],
                    header: declarations::provider_registry::EmrHeader {
                        provider_id: scenario.ext.emr_header.provider_id.clone(),
                        user_id: scenario.ext.emr_header.user_id.clone(),
                        emr_id: scenario.ext.emr_header.emr_id.clone(),
                        registry_id: scenario.ext.emr_header.registry_id.clone(),
                    },
                }
            )
            .unwrap();

        let response = scenario.registries.patient
            .get_logs(
                &scenario.registries.ic,
                scenario.patient.principal.clone(),
                PatientCall::Query
            )
            .unwrap();

        assert!(response.logs.len() == 2);

        let log = response.logs.get(1).unwrap();

        match log.activity_type {
            ActivityType::Updated => (),
            ActivityType::Accessed => unreachable!("should not have accessed log"),
            ActivityType::Revoked => unreachable!("should not have revoked log"),
        }

        scenario.registries.patient
            .revoke_consent(
                &scenario.registries.ic,
                scenario.patient.principal.clone(),
                PatientCall::Update,
                declarations::patient_registry::RevokeConsentRequest {
                    codes: vec![code.clone()],
                }
            )
            .unwrap();

        let response = scenario.registries.patient
            .get_logs(
                &scenario.registries.ic,
                scenario.patient.principal.clone(),
                PatientCall::Query
            )
            .unwrap();

        assert!(response.logs.len() == 3);

        let log = response.logs.get(2).unwrap();

        match log.activity_type {
            ActivityType::Updated => unreachable!("should not have updated log"),
            ActivityType::Accessed => unreachable!("should not have accessed log"),
            ActivityType::Revoked => (),
        }
    }

    // todo: will fix later
    // #[test]
    // fn test_get_provider_list() {
    //     let (registry, provider, patient) = common::Scenario::one_provider_one_patient();

    //     // Register a few more providers
    //     let provider2 = common::Provider(common::random_identity());
    //     let provider3 = common::Provider(common::random_identity());

    //     let register_provider = |provider: &common::Provider| {
    //         let arg = RegisternewProviderRequest {
    //             provider_principal: provider.0.clone(),
    //             display_name: format!("Provider {}", provider.0),
    //             address: format!("Address {}", provider.0),
    //         };

    //         registry.provider
    //             .register_new_provider(&registry.ic, registry.controller.clone(), Call::Update, arg)
    //             .unwrap();
    //     };

    //     register_provider(&provider2);
    //     register_provider(&provider3);

    //     // Get the provider list
    //     let result = registry.provider
    //         .get_provider_list(&registry.ic, registry.controller.clone(), Call::Query)
    //         .unwrap();

    //     // Check that we have at least 3 providers (the original one plus the two we just added)
    //     assert!(result.providers.len() >= 3);

    //     // Check that our newly added providers are in the list
    //     let provider_ids: Vec<String> = result.providers
    //         .iter()
    //         .map(|p| match p {
    //             integration_tests::declarations::provider_registry::Provider::V1(p) => p.internal_id.clone()
    //         })
    //         .collect();

    //     assert!(provider_ids.contains(&provider.0.to_string()));
    //     assert!(provider_ids.contains(&provider2.0.to_string()));
    //     assert!(provider_ids.contains(&provider3.0.to_string()));
    // }

    #[test]
    fn test_update_kyc_status() {
        let (registries, patient, admin_principal) = common::Scenario::one_admin_one_patient();

        // test authorized access - should succeed
        let update_kyc_arg = patient_registry::UpdateKycStatusRequest {
            nik: patient.nik.to_string(),
            kyc_status: patient_registry::KycStatus::Approved,
        };

        let response = registries.patient
            .update_kyc_status(
                &registries.ic,
                admin_principal.clone(), 
                patient_registry::pocket_ic_bindings::Call::Update,
                update_kyc_arg
            )
            .unwrap();

        // verify response
        if let patient_registry::Patient::V1(v1) = response.patient {
            match v1.kyc_status {
                patient_registry::KycStatus::Approved => {},
                _ => panic!("Expected KYC status to be Approved"),
            }
        }

        // verify updated status through get_patient_info
        let patient_info = registries.patient
            .get_patient_info(
                &registries.ic,
                patient.principal.clone(),
                patient_registry::pocket_ic_bindings::Call::Query
            )
            .unwrap();

        if let patient_registry::Patient::V1(v1) = patient_info.patient {
            match v1.kyc_status {
                patient_registry::KycStatus::Approved => {},
                _ => panic!("Expected KYC status to be Approved"),
            }
        }

        // test updating to Denied status
        let update_kyc_arg = patient_registry::UpdateKycStatusRequest {
            nik: patient.nik.to_string(),
            kyc_status: patient_registry::KycStatus::Denied,
        };

        let response = registries.patient
            .update_kyc_status(
                &registries.ic,
                admin_principal.clone(), // Using admin principal
                patient_registry::pocket_ic_bindings::Call::Update,
                update_kyc_arg
            )
            .unwrap();

        // verify final response
        if let patient_registry::Patient::V1(v1) = response.patient {
            match v1.kyc_status {
                patient_registry::KycStatus::Denied => {},
                _ => panic!("Expected KYC status to be Denied"),
            }
        }

        // verify final status through get_patient_info
        let patient_info = registries.patient
            .get_patient_info(
                &registries.ic,
                patient.principal.clone(),
                patient_registry::pocket_ic_bindings::Call::Query
            )
            .unwrap();

        if let patient_registry::Patient::V1(v1) = patient_info.patient {
            match v1.kyc_status {
                patient_registry::KycStatus::Denied => {},
                _ => panic!("Expected KYC status to be Denied"),
            }
        }
    }
}

