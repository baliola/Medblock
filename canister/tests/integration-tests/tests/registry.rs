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

mod common;

mod test {
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
}
