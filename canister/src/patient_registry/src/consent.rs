use core::str;
use std::{ str::FromStr, time::Duration };

use candid::{ CandidType };
use canister_common::{
    common::{ Id, ProviderId },
    deref,
    id_generator::IdGenerator,
    impl_max_size,
    impl_mem_bound,
    impl_range_bound,
    log,
    metrics,
    mmgr::MemoryManager,
    opaque_metrics,
    random::{ CanisterRandomSource, RandomSource },
    stable::{ Stable, StableSet, ToStable },
    statistics::traits::Metrics,
};
use serde::Deserialize;

use crate::{ registry::{ PatientRegistry, NIK }, with_state };

thread_local! {
    pub static CONSENTS: std::cell::RefCell<Option<ConsentMap>> = const {
        std::cell::RefCell::new(None)
    };
    pub static INIT_FLAG: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn with_consent_mut<R>(f: impl FnOnce(&mut ConsentMap) -> R) -> R {
    CONSENTS.with(|cell| f(cell.borrow_mut().as_mut().expect("consents not initialized")))
}

pub fn ensure_initialized() {
    if !INIT_FLAG.with(|cell| cell.get()) {
        panic!("consents not initialized");
    }
}

pub fn with_consent<R>(f: impl FnOnce(&ConsentMap) -> R) -> R {
    CONSENTS.with(|cell| f(cell.borrow().as_ref().expect("consents not initialized")))
}

// change this if you want to change the expiry time of the consent code
const EXPIRY: Duration = Duration::from_secs(60 * 60); // 1 hour

// change this if you want to change the length of the consent code
const CODE_LEN: usize = 6;

// change this if you want to change the allowed characters in the consent code
const ALLOWED_CHAR: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

/// A consent code is a 6 digit code that is used to identify a consent
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ConsentCode([u8; CODE_LEN]);
// benchmarked for candid encoding
impl_max_size!(for ConsentCode: 14);
impl_mem_bound!(for ConsentCode: bounded; fixed_size:false);
impl_range_bound!(ConsentCode);

#[cfg(test)]
mod encode_test_consent_code {
    use super::*;

    #[test]
    fn test_len_encoded() {
        use candid::{ Encode, Decode };
        let code = ConsentCode([b'1', b'2', b'3', b'4', b'5', b'6']);
        let encoded = Encode!(&code).unwrap();
        println!("encoded: {:?}", encoded.len());
        let decoded: ConsentCode = Decode!(&encoded, ConsentCode).unwrap();

        assert_eq!(code, decoded);
    }
}

impl ConsentCode {
    /// the u32 is assumed to be random and unique
    // for now, we are using the last 6 digits of the u32
    pub fn from_u64(u: u64) -> Self {
        let str = u.to_string();
        let str = &str[str.len() - CODE_LEN..];
        let mut code = [0; CODE_LEN];

        for (i, c) in str.chars().enumerate() {
            code[i] = c as u8;
        }

        ConsentCode(code)
    }
}

impl FromStr for ConsentCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != CODE_LEN {
            return Err("invalid length".to_string());
        }

        let mut code = [0; CODE_LEN];

        for (i, c) in s.chars().enumerate() {
            if !ALLOWED_CHAR.contains(&c) {
                return Err("invalid character".to_string());
            }

            code[i] = c as u8;
        }

        Ok(ConsentCode(code))
    }
}

impl ConsentCode {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }

    pub fn from_text(text: &str) -> Result<Self, String> {
        Self::from_str(text)
    }
}

impl<'de> Deserialize<'de> for ConsentCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let s = String::deserialize(deserializer)?;
        ConsentCode::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl CandidType for ConsentCode {
    fn _ty() -> candid::types::Type {
        candid::types::TypeInner::Text.into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
        where S: candid::types::Serializer
    {
        self.as_str().idl_serialize(serializer)
    }
}

impl std::fmt::Display for ConsentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for ConsentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[cfg(test)]
mod tests_code {
    use super::*;

    #[test]
    fn test_consent_code() {
        let code = ConsentCode([b'1', b'2', b'3', b'4', b'5', b'6']);
        assert_eq!(code.as_str(), "123456");
    }

    #[test]
    fn test_consent_code_from_str() {
        let code = ConsentCode::from_str("123456").expect("failed to parse valid consent code");
        assert_eq!(code.as_str(), "123456");

        assert!(ConsentCode::from_str("12345").is_err());
        assert!(ConsentCode::from_str("1234567").is_err());
        assert!(ConsentCode::from_str("12345a").is_err());
    }

    #[test]
    fn test_consent_code_from_text() {
        let code = ConsentCode::from_text("123456").expect("failed to parse valid consent code");
        assert_eq!(code.as_str(), "123456");

        assert!(ConsentCode::from_text("12345").is_err());
        assert!(ConsentCode::from_text("1234567").is_err());
        assert!(ConsentCode::from_text("12345a").is_err());
    }

    #[test]
    fn test_from_u32() {
        let code = ConsentCode::from_u64(u64::MAX);
        assert_eq!(code.as_str(), "551615");
    }
}

pub struct ConsentsApi;

impl ConsentsApi {
    pub fn consent(code: &ConsentCode) -> Option<Consent> {
        ensure_initialized();
        with_consent(|consents| consents.consent(code).cloned())
    }

    pub fn is_session_user(user: &Stable<ProviderId>) -> bool {
        ensure_initialized();
        with_consent(|consents| consents.is_session_user(user))
    }

    pub fn metrics() -> String {
        ensure_initialized();
        with_consent(|consents| opaque_metrics!(consents))
    }

    pub fn resolve_session(session_id: &SessionId, session_user: &ProviderId) -> Option<Consent> {
        ensure_initialized();
        with_consent_mut(|consents| consents.resolve_session(session_id, session_user))
    }

    pub fn resolve_session_with_code(code: &ConsentCode, patient: &NIK) -> Option<Consent> {
        ensure_initialized();
        with_consent(|consents| consents.resolve_session_with_code(code, patient))
    }

    pub fn list_consent_with_patient(user: &NIK) -> Vec<Consent> {
        ensure_initialized();
        with_consent(|consents| consents.list_consent_with_patient(user))
    }

    // we dont have expiry for now
    // fn remove_consent_after_expiry(code: ConsentCode) {
    //     ensure_initialized();

    //     ic_cdk_timers::set_timer(EXPIRY, move || {
    //         let Some(consent) = with_consent_mut(|consent| consent.remove_consent(&code)) else {
    //             return;
    //         };

    //         match consent.session_id {
    //             Some(ref session) =>
    //                 with_consent_mut(|consent| consent.finish_session_unchecked(session)),
    //             None => (),
    //         }
    //     });
    // }

    pub fn generate_consent(nik: NIK) -> ConsentCode {
        ensure_initialized();

        let partial = PartialConsent::new(nik);

        // we dont have expiry for now
        // Self::remove_consent_after_expiry(code);
        with_consent_mut(|consents| { consents.add_consent(partial) })
    }

    pub fn revoke_consent(code: &ConsentCode) {
        ensure_initialized();
        with_consent_mut(|consents| consents.remove_consent(code));
    }

    pub fn is_session_allowed(code: &ConsentCode, session_id: &Id) -> bool {
        ensure_initialized();
        with_consent(|consents| consents.ensure_session_allowed(code, session_id))
    }

    pub fn is_claimed(code: &ConsentCode, patient: &NIK) -> bool {
        ensure_initialized();
        with_consent(|consents| consents.is_claimed(code, patient))
    }

    // TODO: temporary function to get patient list
    pub fn user_list_with_consent(user: &ProviderId) -> Vec<Consent> {
        ensure_initialized();
        with_consent(|consents| consents.consent_list_with_user(user))
    }

    pub fn claim_consent(
        code: &ConsentCode,
        session_user: ProviderId
    ) -> Option<(Id, canister_common::common::H256)> {
        ensure_initialized();

        with_consent_mut(|consents| consents.claim_consent(code, session_user))
    }

    pub fn finish_sesion(session_id: &Id, session_user: &ProviderId) {
        ensure_initialized();
        with_consent_mut(|consents| consents.finish_session(session_id, session_user));
    }

    pub async fn read_emr_with_session(
        session_id: &SessionId,
        req: crate::api::ReadEmrByIdRequest,
        registry: crate::declarations::emr_registry::EmrRegistry,
        session_user: &ProviderId
    ) -> Result<crate::declarations::emr_registry::ReadEmrByIdResponse, String> {
        ensure_initialized();
        let consent = with_consent_mut(|consents| {
            consents.resolve_session(session_id, session_user)
        });

        match consent {
            Some(consent) =>
                Ok(PatientRegistry::do_call_read_emr(req.to_args(consent.nik), registry).await),
            None => { Err("invalid session".to_string()) }
        }
    }

    /// call this function in the init method of the canister
    pub fn init() {
        ConsentMap::init();
    }
}

pub type SessionId = Id;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Consent {
    pub code: ConsentCode,
    pub nik: NIK,
    pub claimed: bool,
    pub session_id: Option<SessionId>,
    pub session_user: Option<ProviderId>,
}
#[cfg(test)]
mod encode_test_consent {
    use super::*;

    #[test]
    fn test_len_encoded() {
        use candid::{ Encode, Decode };
        let code = Consent {
            code: ConsentCode([b'1', b'2', b'3', b'4', b'5', b'6']),
            nik: NIK::from_str(
                "3fe93da886732fd563ba71f136f10dffc6a8955f911b36064b9e01b32f8af709"
            ).unwrap(),
            claimed: false,
            session_id: None,
            session_user: None,
        };
        let encoded = Encode!(&code).unwrap();
        println!("encoded: {:?}", encoded.len());
        let decoded: Consent = Decode!(&encoded, Consent).unwrap();

        assert_eq!(code, decoded);
    }
}

// ~117 bytes benchmarked for candid encoding
impl_max_size!(for Consent: 120);
impl_mem_bound!(for Consent: bounded; fixed_size:false);
impl_range_bound!(Consent);

impl Consent {
    pub fn from_partial(partial: PartialConsent, code: ConsentCode) -> Self {
        Consent {
            session_id: None,
            claimed: false,
            code,
            nik: partial.nik,
            session_user: None,
        }
    }
}

#[derive(CandidType, Debug, Deserialize, Clone)]
pub struct PartialConsent {
    nik: NIK,
}

impl PartialConsent {
    pub fn new(nik: NIK) -> Self {
        Self { nik }
    }
}

pub struct ProviderConsentSet(StableSet<Stable<ProviderId>, Stable<SessionId>>);

impl ProviderConsentSet {
    pub fn is_session_user(&self, user: &Stable<ProviderId>) -> bool {
        self.range_key_exists(user)
    }
}

impl ProviderConsentSet {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        let set = StableSet::init::<Self>(memory_manager);

        ProviderConsentSet(set)
    }
}
deref!(mut ProviderConsentSet: StableSet<Stable<ProviderId>, Stable<SessionId>>);

// TODO: move all maps to stable memory
pub struct ConsentMap {
    provider_set: ProviderConsentSet,
    inner: std::collections::HashMap<ConsentCode, Consent>,
    sessions: std::collections::HashMap<SessionId, ConsentCode>,
    // TODO: remove this after demo, move all of the structure into stable memory
    // and then move the consent related functions to provider registry either all of them or part of it
    rng: CanisterRandomSource,
}

mod metrics {
    use super::*;

    metrics!(ConsentMap: OngoingSession);

    impl Metrics<OngoingSession> for ConsentMap {
        fn metrics_name() -> &'static str {
            "consent_map"
        }

        fn metrics_measurements() -> &'static str {
            "ongoing_session"
        }

        fn update_measurements(&self) {
            // no-op
        }

        fn get_measurements(&self) -> String {
            self.sessions.len().to_string()
        }
    }
}

impl ConsentMap {
    pub fn is_session_user(&self, user: &Stable<ProviderId>) -> bool {
        self.provider_set.is_session_user(user)
    }

    pub fn new_with_seed(seed: u64, memory_manager: &MemoryManager) -> Self {
        ConsentMap {
            provider_set: ProviderConsentSet::init(memory_manager),
            sessions: std::collections::HashMap::new(),
            inner: std::collections::HashMap::new(),
            rng: CanisterRandomSource::new_with_seed(seed),
        }
    }

    //TODO: temporary function to get patient list
    pub fn consent_list_with_user(&self, user: &ProviderId) -> Vec<Consent> {
        self.inner
            .values()
            .filter(|consent| consent.session_user.as_ref().eq(&Some(user)))
            .cloned()
            .collect()
    }

    pub fn new(rng: CanisterRandomSource, memory_manager: &MemoryManager) -> Self {
        ConsentMap {
            provider_set: ProviderConsentSet::init(memory_manager),
            sessions: std::collections::HashMap::new(),
            inner: std::collections::HashMap::new(),
            rng,
        }
    }

    /// call this every canister initialization
    pub fn init() {
        ic_cdk_timers::set_timer(Duration::from_secs(3), || {
            ic_cdk::spawn(async move {
                let rng = CanisterRandomSource::new().await;

                let new_cell = with_state(|s| {
                    let memory_manager = &s.memory_manager;
                    ConsentMap::new(rng, memory_manager)
                });

                CONSENTS.replace(Some(new_cell));

                log!("consents map initialized");

                INIT_FLAG.replace(true);
            });
        });
    }

    pub fn consent(&self, code: &ConsentCode) -> Option<&Consent> {
        self.inner.get(code)
    }

    pub fn is_claimed(&self, code: &ConsentCode, patient: &NIK) -> bool {
        match self.inner.get(code) {
            Some(consent) => consent.claimed && consent.nik.eq(patient),
            None => false,
        }
    }

    pub fn add_consent(&mut self, partial: PartialConsent) -> ConsentCode {
        let random = self.rng.raw_random_u64();

        let code = ConsentCode::from_u64(random);
        let consent = Consent::from_partial(partial, code);

        assert!(self.inner.insert(code, consent).is_none());

        code
    }

    pub fn ensure_session_allowed(&self, code: &ConsentCode, session_id: &Id) -> bool {
        match self.inner.get(code) {
            Some(consent) => consent.claimed && consent.session_id.as_ref().eq(&Some(session_id)),
            None => false,
        }
    }

    pub fn get_consent_uncheked(&self, code: &ConsentCode) -> Option<&Consent> {
        self.inner.get(code)
    }

    pub fn safe_get_consent_for(
        &self,
        code: &ConsentCode,
        session_user: &ProviderId
    ) -> Option<&Consent> {
        let consent = self.get_consent_uncheked(code);

        match consent {
            Some(consent) => {
                if consent.session_user.as_ref().eq(&Some(session_user)) {
                    return Some(consent);
                }
            }
            None => (),
        }

        None
    }

    pub fn remove_consent(&mut self, code: &ConsentCode) -> Option<Consent> {
        self.inner.remove(code)
    }

    pub fn ensure_correct_session_owner(&self, session_id: &Id, session_user: &ProviderId) -> bool {
        match self.sessions.get(session_id) {
            Some(consent) => {
                match self.inner.get(consent) {
                    Some(consent) => consent.session_user.as_ref().eq(&Some(session_user)),
                    None => false,
                }
            }
            None => false,
        }
    }

    /// finish a session, will remove the session if the consent is already removed, panic if session is not owned by the user
    pub fn finish_session(&mut self, session_id: &SessionId, session_user: &ProviderId) {
        if !self.ensure_correct_session_owner(session_id, session_user) {
            ic_cdk::trap("invalid session owner");
        }

        self.finish_session_unchecked(session_id)
    }

    pub fn finish_session_unchecked(&mut self, session_id: &SessionId) {
        let code = self.sessions.remove(session_id);

        // remove the consent if the session is finished, no-op if the consent is already removed
        if let Some(ref code) = code {
            self.remove_consent(code);
        }
    }

    pub fn list_consent_with_patient(&self, user: &NIK) -> Vec<Consent> {
        self.inner
            .iter()
            .filter(|(_, c)| c.nik.eq(user))
            .map(|(_, consent)| consent)
            .cloned()
            .collect()
    }

    /// wil return none if consent is already claimed or does not exist
    pub fn claim_consent(
        &mut self,
        code: &ConsentCode,
        session_user: ProviderId
    ) -> Option<(SessionId, NIK)> {
        let Some(consent) = self.inner.get_mut(code) else {
            return None;
        };

        if consent.claimed {
            return None;
        }

        let rng = &mut self.rng;
        let session_id = IdGenerator::<CanisterRandomSource>::generate_id_with_different_source(
            rng
        );

        consent.claimed = true;
        consent.session_id = Some(session_id.clone());
        consent.session_user = Some(session_user.clone());

        assert!(self.sessions.get(&session_id).is_none(), "session id already exists");
        assert!(self.sessions.insert(session_id.clone(), code.to_owned()).is_none());

        self.provider_set.insert(session_user.to_stable(), session_id.clone().into());

        let nik = consent.nik.clone();

        Some((session_id, nik))
    }

    /// resolve a given session id to consent if it has been claimed,
    /// will return [None] the session if the consent is already removed.
    pub fn resolve_session(
        &mut self,
        session_id: &SessionId,
        session_user: &ProviderId
    ) -> Option<Consent> {
        let code = self.sessions.get(session_id);

        let Some(code) = code else {
            return None;
        };

        // return the consent if it exists
        self.safe_get_consent_for(code, session_user).cloned()
    }

    pub fn resolve_session_with_code(&self, code: &ConsentCode, patient: &NIK) -> Option<Consent> {
        let consent = self.inner.get(code).cloned();

        match consent {
            Some(consent) => {
                if consent.nik.eq(patient) { Some(consent) } else { None }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use canister_common::{ memory_manager };

    use super::*;

    #[test]
    fn test_consent() {
        let memory_manager = memory_manager!();
        let mut consents = ConsentMap::new_with_seed(0, &memory_manager);

        let nik = NIK::from_str(
            "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c"
        ).unwrap();

        let partial = PartialConsent::new(nik.clone());

        let code = consents.add_consent(partial.clone());

        assert_eq!(consents.get_consent_uncheked(&code).unwrap().nik, nik);

        consents.remove_consent(&code);

        assert_eq!(consents.get_consent_uncheked(&code), None);
    }

    #[test]
    fn test_claim_consent() {
        let memory_manager = memory_manager!();
        let mut consents = ConsentMap::new_with_seed(0, &memory_manager);

        let nik = NIK::from_str(
            "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c"
        ).unwrap();

        let partial = PartialConsent::new(nik.clone());

        let code = consents.add_consent(partial.clone());

        let (session_id, _) = consents.claim_consent(&code, Principal::anonymous()).unwrap();

        assert_eq!(
            consents.get_consent_uncheked(&code).unwrap().session_id.as_ref().unwrap(),
            &session_id
        );

        assert!(consents.claim_consent(&code, Principal::anonymous()).is_none());
    }

    #[test]
    fn session() {
        let memory_manager = memory_manager!();
        let mut consents = ConsentMap::new_with_seed(0, &memory_manager);

        let nik = NIK::from_str(
            "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c"
        ).unwrap();

        let partial = PartialConsent::new(nik.clone());
        let code = consents.add_consent(partial.clone());

        let (session_id, _) = consents.claim_consent(&code, Principal::anonymous()).unwrap();

        assert!(consents.ensure_session_allowed(&code, &session_id));

        consents.finish_session_unchecked(&session_id);

        assert!(!consents.ensure_session_allowed(&code, &session_id));
        assert!(consents.resolve_session(&session_id, &Principal::anonymous()).is_none());
        assert!(consents.get_consent_uncheked(&code).is_none());
        assert!(
            consents.provider_set.contains_key(
                PrincipalBytes::from(Principal::anonymous()).into(),
                session_id.into()
            )
        )
    }

    #[test]
    #[should_panic]
    fn panic_wrong_session_user() {
        let memory_manager = memory_manager!();
        let mut consents = ConsentMap::new_with_seed(0, &memory_manager);

        let nik = NIK::from_str(
            "9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c"
        ).unwrap();

        let partial = PartialConsent::new(nik.clone());
        let code = consents.add_consent(partial.clone());

        let (session_id, _) = consents.claim_consent(&code, Principal::anonymous()).unwrap();

        consents.finish_session(
            &session_id,
            &Principal::from_text("s55qq-oqaaa-aaaaa-aaakq-cai").unwrap()
        );
    }
}
