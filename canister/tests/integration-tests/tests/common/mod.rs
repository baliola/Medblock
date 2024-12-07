use std::{path::Path, process::Command, str::FromStr, time::Duration};

use candid::{CandidType, Encode};
use canister_common::common::H256;
use ic_agent::Identity;
use ic_cdk::api::management_canister::main::CanisterSettings;
use ic_principal::Principal;
use integration_tests::declarations::{
    self, patient_registry,
    provider_registry::{ProviderInfoRequest, RegisternewProviderRequest},
};
use pocket_ic::UserError;

use integration_tests::declarations::patient_registry::pocket_ic_bindings::Call as PatientCall;
use integration_tests::declarations::provider_registry::pocket_ic_bindings::Call as ProviderCall;

const PROVIDER_REGISTRY_WASM: &[u8] =
    include_bytes!("../../../../target/wasm32-unknown-unknown/release/provider_registry.wasm");
const PATIENT_REGISTRY_WASM: &[u8] =
    include_bytes!("../../../../target/wasm32-unknown-unknown/release/patient_registry.wasm");
const EMR_REGISTRY_WASM: &[u8] =
    include_bytes!("../../../../target/wasm32-unknown-unknown/release/emr_registry.wasm");

const POCKET_IC_PATH: Option<&'static str> = std::option_env!("POCKET_IC_BIN");

// 10 Triliion cycle
const CYCLE: u128 = 10_000_000_000_000;

fn create_canister(
    server: &pocket_ic::PocketIc,
    bytes: &'static [u8],
    arg: Vec<u8>,
    controller: Principal,
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = server.create_canister_with_settings(
        Some(controller.clone()),
        Some(CanisterSettings {
            controllers: Some(vec![controller.clone()]),
            compute_allocation: None,
            memory_allocation: None,
            freezing_threshold: None,
        }),
    );

    server.add_cycles(id.clone(), CYCLE);
    server.install_canister(id.clone(), bytes.to_vec(), arg, Some(controller));

    id
}

fn setup_emr_registry(
    server: &pocket_ic::PocketIc,
    controller: Principal,
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = create_canister(server, EMR_REGISTRY_WASM, Vec::new(), controller);
    id
}

fn bind_emr_registry(
    server: &pocket_ic::PocketIc,
    id: Principal,
    controller: Principal,
    provider: Principal,
    patient: Principal,
) {
    let args = declarations::emr_registry::AuthorizedCallerRequest {
        caller: patient.clone(),
    };

    server.update_call(
        id.clone(),
        controller.clone(),
        "add_authorized_caller",
        Encode!(&args).unwrap(),
    );

    let args = declarations::emr_registry::AuthorizedCallerRequest {
        caller: provider.clone(),
    };

    server.update_call(
        id.clone(),
        controller.clone(),
        "add_authorized_caller",
        Encode!(&args).unwrap(),
    );
}

fn setup_provider_registry(
    server: &pocket_ic::PocketIc,
    controller: Principal,
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = create_canister(server, PROVIDER_REGISTRY_WASM, Vec::new(), controller);
    id
}

fn bind_provider_registry(
    server: &pocket_ic::PocketIc,
    id: Principal,
    controller: Principal,
    emr: Principal,
    patient: Principal,
) {
    let args = declarations::provider_registry::AuthorizedCallerRequest {
        caller: emr.clone(),
    };
    let registry =
        integration_tests::declarations::provider_registry::pocket_ic_bindings::ProviderRegistry(
            id.clone(),
        );

    registry.update_emr_registry_principal(
        server,
        controller.clone(),
        ProviderCall::Update,
        declarations::provider_registry::SuspendRequest {
            principal: emr.clone(),
        },
    );

    registry.update_patient_registry_principal(
        server,
        controller.clone(),
        ProviderCall::Update,
        declarations::provider_registry::SuspendRequest {
            principal: patient.clone(),
        },
    );
}

fn setup_patient_registry(
    server: &pocket_ic::PocketIc,
    controller: Principal,
) -> ic_cdk::api::management_canister::provisional::CanisterId {
    let id = create_canister(server, PATIENT_REGISTRY_WASM, Vec::new(), controller);

    id
}

fn bind_patient_registry(
    server: &pocket_ic::PocketIc,
    id: Principal,
    controller: Principal,
    emr: Principal,
    provider: Principal,
) {
    let args = declarations::patient_registry::AuthorizedCallerRequest {
        caller: emr.clone(),
    };
    let registry =
        integration_tests::declarations::patient_registry::pocket_ic_bindings::PatientRegistry(
            id.clone(),
        );

    registry.update_emr_registry_principal(
        server,
        controller.clone(),
        PatientCall::Update,
        declarations::patient_registry::UpdateEmrRegistryRequest {
            principal: emr.clone(),
        },
    );

    // bind the patient registry to the provider registry
    let args = declarations::patient_registry::AuthorizedCallerRequest {
        caller: provider.clone(),
    };

    registry.update_provider_registry_principal(
        server,
        controller.clone(),
        PatientCall::Update,
        declarations::patient_registry::UpdateEmrRegistryRequest {
            principal: provider.clone(),
        },
    );
}

// try to resolve pocket ic binary from path
fn resolve_pocket_ic_path() {
    if POCKET_IC_PATH.is_none() {
        let path = Command::new("which")
            .arg("pocket-ic")
            .output()
            .expect("pocket-ic not found")
            .stdout;

        let path = std::str::from_utf8(&path).unwrap().trim();
        let path = Path::new(path);
        let path = path.to_str().unwrap();

        std::env::set_var("POCKET_IC_BIN", path);
    }
}

fn bind_canisters(
    server: &pocket_ic::PocketIc,
    provider: Principal,
    patient: Principal,
    emr: Principal,
    controller: Principal,
) {
    bind_emr_registry(
        server,
        emr.clone(),
        controller.clone(),
        provider.clone(),
        patient.clone(),
    );
    bind_patient_registry(
        server,
        patient.clone(),
        controller.clone(),
        emr.clone(),
        provider.clone(),
    );
    bind_provider_registry(
        server,
        provider.clone(),
        controller.clone(),
        emr.clone(),
        patient.clone(),
    );
}

pub struct Registries {
    pub ic: pocket_ic::PocketIc,
    pub emr: integration_tests::declarations::emr_registry::pocket_ic_bindings::EmrRegistry,
    pub patient:
        integration_tests::declarations::patient_registry::pocket_ic_bindings::PatientRegistry,
    pub provider:
        integration_tests::declarations::provider_registry::pocket_ic_bindings::ProviderRegistry,
    pub controller: Principal,
}

pub fn prepare() -> Registries {
    resolve_pocket_ic_path();

    let server = pocket_ic::PocketIcBuilder::new()
        .with_application_subnet()
        .build();
    let controller = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();

    let id = Principal::anonymous();

    let provider = setup_provider_registry(&server, controller.clone());
    let patient = setup_patient_registry(&server, controller.clone());
    let emr = setup_emr_registry(&server, controller.clone());

    bind_canisters(
        &server,
        provider.clone(),
        patient.clone(),
        emr.clone(),
        controller.clone(),
    );

    // to fully initialize the canisters
    server.advance_time(Duration::from_secs(1000));
    for _ in 0..10 {
        server.tick();
    }

    Registries {
        ic: server,
        emr: integration_tests::declarations::emr_registry::pocket_ic_bindings::EmrRegistry(emr),
        patient:
            integration_tests::declarations::patient_registry::pocket_ic_bindings::PatientRegistry(
                patient,
            ),
        provider:
            integration_tests::declarations::provider_registry::pocket_ic_bindings::ProviderRegistry(
                provider,
            ),
        controller,
    }
}

pub fn random_identity() -> Principal {
    let rand = ring::rand::SystemRandom::new();
    let key = ring::signature::Ed25519KeyPair::generate_pkcs8(&rand).unwrap();
    let key = ring::signature::Ed25519KeyPair::from_pkcs8(key.as_ref()).unwrap();

    let identity = ic_agent::identity::BasicIdentity::from_key_pair(key);

    identity.sender().unwrap()
}

pub struct Scenario;

pub struct ScenarioResult<Ext> {
    pub registries: Registries,
    pub provider: Provider,
    pub patient: Patient,
    pub ext: Ext,
}

use ext::*;
pub mod ext {
    use super::*;

    pub struct EmrExt {
        pub emr_header: declarations::provider_registry::Header,
    }
}

impl<Ext> ScenarioResult<Ext> {
    pub fn new(registries: Registries, provider: Provider, patient: Patient, ext: Ext) -> Self {
        Self {
            registries,
            provider,
            patient,
            ext,
        }
    }
}

pub struct Provider(pub Principal);
pub struct Patient {
    pub principal: Principal,
    pub nik: H256,
}

impl Scenario {
    pub fn one_provider_one_patient() -> (Registries, Provider, Patient) {
        let registries = prepare();

        let provider = Provider(random_identity());
        let nik = canister_common::common::H256::from_str(
            "3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709",
        )
        .unwrap();

        let patient = Patient {
            principal: random_identity(),
            nik: nik.clone(),
        };

        // prepare provider
        let display = String::from("PT RUMAH SAKIT").to_ascii_lowercase();
        let address = String::from("JL.STREET").to_ascii_lowercase();

        let arg = RegisternewProviderRequest {
            provider_principal: provider.0.clone(),
            display_name: display.clone(),
            address: address.clone(),
        };

        registries
            .provider
            .register_new_provider(
                &registries.ic,
                registries.controller.clone(),
                ProviderCall::Update,
                arg,
            )
            .unwrap();

        let arg = ProviderInfoRequest {
            provider: vec![provider.0.clone()],
        };

        // prepare patient

        let display = String::from("pasien").to_ascii_lowercase();
        let address = String::from("jl.rumah").to_ascii_lowercase();

        let arg = patient_registry::RegisterPatientRequest {
            nik: nik.to_string(),
        };

        registries
            .patient
            .register_patient(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        let arg = patient_registry::UpdatePatientInfoRequest {
            info: patient_registry::V1 {
                name: display.clone(),
                martial_status: "married".to_string(),
                place_of_birth: "Jakarta".to_ascii_lowercase(),
                address,
                gender: "men".to_ascii_lowercase(),
                date_of_birth: "1990-01-01".to_string(),
                kyc_status: patient_registry::KycStatus::Pending,
                kyc_date: "2024-01-01".to_string(),
            },
        };

        registries
            .patient
            .update_patient_info(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        (registries, provider, patient)
    }

    pub fn one_provider_one_patient_with_one_emr() -> ScenarioResult<EmrExt> {
        let (registry, provider, patient) = Self::one_provider_one_patient();

        let arg = declarations::provider_registry::IssueEmrRequest {
            emr: vec![declarations::provider_registry::EmrFragment {
                key: "key".to_string(),
                value: "value".to_string(),
            }],
            user_id: patient.nik.clone().to_string(),
        };

        let response = registry
            .provider
            .issue_emr(&registry.ic, provider.0.clone(), ProviderCall::Update, arg)
            .unwrap();

        ScenarioResult::new(
            registry,
            provider,
            patient,
            EmrExt {
                emr_header: response.emr_header,
            },
        )
    }

    pub fn one_provider_two_patient_with_emrs() -> (Registries, Provider, Patient, Patient) {
        let registries = prepare();
        let patient1 = Self::create_patient(&registries);
        let patient2 = Self::create_patient(&registries);
        let provider = Provider(random_identity());

        // prepare provider
        let display = String::from("PT RUMAH SAKIT").to_ascii_lowercase();
        let address = String::from("JL.STREET").to_ascii_lowercase();

        let arg = RegisternewProviderRequest {
            provider_principal: provider.0.clone(),
            display_name: display.clone(),
            address: address.clone(),
        };

        registries
            .provider
            .register_new_provider(
                &registries.ic,
                registries.controller.clone(),
                ProviderCall::Update,
                arg,
            )
            .unwrap();

        // Advance time and tick to ensure provider registration is complete
        registries.ic.advance_time(Duration::from_secs(1));
        registries.ic.tick();

        // issue EMRs for both patients
        let emr_req1 = declarations::provider_registry::IssueEmrRequest {
            emr: vec![declarations::provider_registry::EmrFragment {
                key: "key1".to_string(),
                value: "value1".to_string(),
            }],
            user_id: patient1.nik.clone().to_string(),
        };

        let emr_req2 = declarations::provider_registry::IssueEmrRequest {
            emr: vec![declarations::provider_registry::EmrFragment {
                key: "key2".to_string(),
                value: "value2".to_string(),
            }],
            user_id: patient2.nik.clone().to_string(),
        };

        // issue EMRs for both patients and ensure they complete
        registries
            .provider
            .issue_emr(
                &registries.ic,
                provider.0.clone(),
                ProviderCall::Update,
                emr_req1,
            )
            .unwrap();

        // Advance time and tick to ensure first EMR issuance is complete
        registries.ic.advance_time(Duration::from_secs(1));
        registries.ic.tick();

        registries
            .provider
            .issue_emr(
                &registries.ic,
                provider.0.clone(),
                ProviderCall::Update,
                emr_req2,
            )
            .unwrap();

        // Advance time and tick to ensure second EMR issuance is complete
        registries.ic.advance_time(Duration::from_secs(1));
        registries.ic.tick();

        (registries, provider, patient1, patient2)
    }

    pub fn one_admin_one_patient() -> (Registries, Patient, Principal) {
        let registries = prepare();

        // Create patient
        let nik = canister_common::common::H256::from_str(
            "3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709",
        )
        .unwrap();

        let patient = Patient {
            principal: random_identity(),
            nik: nik.clone(),
        };

        // Register patient
        let display = String::from("pasien").to_ascii_lowercase();
        let address = String::from("jl.rumah").to_ascii_lowercase();

        let arg = patient_registry::RegisterPatientRequest {
            nik: nik.to_string(),
        };

        registries
            .patient
            .register_patient(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        // Set initial patient info
        let arg = patient_registry::UpdatePatientInfoRequest {
            info: patient_registry::V1 {
                name: display.clone(),
                martial_status: "single".to_string(),
                place_of_birth: "Jakarta".to_ascii_lowercase(),
                address,
                gender: "male".to_ascii_lowercase(),
                date_of_birth: "1990-01-01".to_string(),
                kyc_status: patient_registry::KycStatus::Pending,
                kyc_date: "2024-01-01".to_string(),
            },
        };

        registries
            .patient
            .update_patient_info(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        // Create and bind admin
        let admin_principal = random_identity();
        let admin_nik = canister_common::common::H256::from([1u8; 32]);

        let bind_admin_arg = patient_registry::BindAdminRequest {
            principal: admin_principal.clone(),
            nik: admin_nik.to_string(),
        };

        registries
            .patient
            .bind_admin(
                &registries.ic,
                registries.controller.clone(),
                PatientCall::Update,
                bind_admin_arg,
            )
            .unwrap();

        (registries, patient, admin_principal)
    }

    pub fn create_patient(registries: &Registries) -> Patient {
        // generate a random NIK using timestamp to ensure uniqueness
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut nik_bytes = [0u8; 32];
        nik_bytes[..16].copy_from_slice(&timestamp.to_be_bytes());
        let nik = canister_common::common::H256::from(nik_bytes);

        let patient = Patient {
            principal: random_identity(),
            nik: nik.clone(),
        };

        // register patient
        let arg = patient_registry::RegisterPatientRequest {
            nik: nik.to_string(),
        };

        registries
            .patient
            .register_patient(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        // set initial patient info
        let arg = patient_registry::UpdatePatientInfoRequest {
            info: patient_registry::V1 {
                name: "test patient".to_string(),
                martial_status: "single".to_string(),
                place_of_birth: "Jakarta".to_ascii_lowercase(),
                address: "test address".to_string(),
                gender: "male".to_ascii_lowercase(),
                date_of_birth: "1990-01-01".to_string(),
                kyc_status: patient_registry::KycStatus::Pending,
                kyc_date: "2024-01-01".to_string(),
            },
        };

        registries
            .patient
            .update_patient_info(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        patient
    }

    pub fn create_patient_with_info(
        registries: &Registries,
        info: patient_registry::V1,
    ) -> Patient {
        // generate a random NIK using timestamp to ensure uniqueness
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();

        let mut nik_bytes = [0u8; 32];
        nik_bytes[..16].copy_from_slice(&timestamp.to_be_bytes());
        let nik = canister_common::common::H256::from(nik_bytes);

        let patient = Patient {
            principal: random_identity(),
            nik: nik.clone(),
        };

        // register patient
        let arg = patient_registry::RegisterPatientRequest {
            nik: nik.to_string(),
        };

        registries
            .patient
            .register_patient(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        // set initial patient info with provided V1 struct
        let arg = patient_registry::UpdatePatientInfoRequest { info };

        registries
            .patient
            .update_patient_info(
                &registries.ic,
                patient.principal.clone(),
                PatientCall::Update,
                arg,
            )
            .unwrap();

        patient
    }
}
