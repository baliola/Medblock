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

use integration_tests::declarations::patient_registry::pocket_ic_bindings::Call as PatientCall;
use integration_tests::declarations::provider_registry::pocket_ic_bindings::Call as ProviderCall;

mod common;

mod test {
    use integration_tests::declarations::{
        self,
        patient_registry::EmrListPatientRequest,
        provider_registry::{ EmrFragment, IssueEmrRequest },
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
            provider: Principal::anonymous(),
        };

        let result = registries.provider
            .get_provider_info_with_principal(
                &registries.ic,
                registries.controller.clone(),
                Call::Query,
                arg
            )
            .unwrap();

        match result.provider {
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
}
