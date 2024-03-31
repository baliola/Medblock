use core::str;
use std::{ str::FromStr, time::Duration };

use candid::CandidType;
use canister_common::{
    common::{ EmrHeader, Id },
    id_generator::IdGenerator,
    log,
    random::{ CanisterRandomSource, RandomSource },
};
use serde::Deserialize;

use crate::registry::{ PatientRegistry, NIK };

thread_local! {
    pub static CONSENTS: std::cell::RefCell<Option<ConsentMap>> = std::cell::RefCell::new(None);
    pub static INIT_FLAG: std::cell::Cell<bool> = std::cell::Cell::new(false);
}

pub fn with_consent_mut<R>(f: impl FnOnce(&mut ConsentMap) -> R) -> R {
    CONSENTS.with(|cell| f(&mut cell.borrow_mut().as_mut().expect("consents not initialized")))
}

pub fn ensure_initialized() {
    if !INIT_FLAG.with(|cell| cell.get()) {
        panic!("consents not initialized");
    }
}

pub fn with_consent<R>(f: impl FnOnce(&ConsentMap) -> R) -> R {
    CONSENTS.with(|cell| f(&cell.borrow().as_ref().expect("consents not initialized")))
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
        Ok(ConsentCode::from_str(&s).map_err(serde::de::Error::custom)?)
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
    pub fn resolve_session(session_id: &SessionId) -> Option<Consent> {
        ensure_initialized();
        with_consent_mut(|consents| consents.resolve_session(session_id))
    }

    fn remove_consent_after_expiry(code: ConsentCode) {
        ensure_initialized();

        ic_cdk_timers::set_timer(EXPIRY, move || {
            let Some(consent) = with_consent_mut(|consent| consent.remove_consent(&code)) else {
                return;
            };

            match consent.session_id {
                Some(ref session) => with_consent_mut(|consent| consent.finish_session(session)),
                None => (),
            }
        });
    }

    pub fn generate_consent(nik: NIK, allowed: Vec<EmrHeader>) -> ConsentCode {
        ensure_initialized();

        let partial = PartialConsent::new(nik, allowed);
        let code = with_consent_mut(|consents| { consents.add_consent(partial) });
        Self::remove_consent_after_expiry(code);
        code
    }

    pub fn revoke_consent(code: &ConsentCode) {
        ensure_initialized();
        with_consent_mut(|consents| consents.remove_consent(code));
    }

    pub fn is_header_exists_and_allowed(
        code: &ConsentCode,
        header: &EmrHeader,
        session_id: &Id
    ) -> bool {
        ensure_initialized();
        with_consent(|consents| consents.ensure_header_exists_and_allowed(code, header, session_id))
    }

    pub fn is_session_allowed(code: &ConsentCode, session_id: &Id) -> bool {
        ensure_initialized();
        with_consent(|consents| consents.ensure_session_allowed(code, session_id))
    }

    pub fn claim_consent(code: &ConsentCode) -> Option<SessionId> {
        ensure_initialized();
        let session = with_consent_mut(|consents| consents.claim_consent(code));

        session
    }

    pub fn emr_list_with_session(session_id: &Id) -> Result<Vec<EmrHeader>, String> {
        ensure_initialized();
        let consent = with_consent_mut(|consents| { consents.resolve_session(session_id) });

        match consent {
            Some(consent) => Ok(consent.allowed_headers),
            None => Err("invalid session".to_string()),
        }
    }

    pub fn finish_sesion(session_id: &Id) {
        ensure_initialized();
        with_consent_mut(|consents| consents.finish_session(session_id));
    }

    pub async fn read_emr_with_session(
        session_id: &SessionId,
        req: crate::api::ReadEmrByIdRequest
    ) -> Result<crate::declarations::emr_registry::ReadEmrByIdResponse, String> {
        ensure_initialized();
        let consent = with_consent_mut(|consents| { consents.resolve_session(session_id) });

        match consent {
            Some(consent) => Ok(PatientRegistry::do_call_read_emr(req.to_args(consent.nik)).await),
            None => {
                return Err("invalid session".to_string());
            }
        }
    }

    /// call this function in the init method of the canister
    pub fn init() {
        ConsentMap::init();
    }
}

pub type SessionId = Id;
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Consent {
    pub code: ConsentCode,
    pub nik: NIK,
    pub allowed_headers: Vec<EmrHeader>,
    pub claimed: bool,
    pub session_id: Option<SessionId>,
}

impl Consent {
    pub fn from_partial(partial: PartialConsent, code: ConsentCode) -> Self {
        Consent {
            session_id: None,
            claimed: false,
            code,
            nik: partial.nik,
            allowed_headers: partial.allowed_headers,
        }
    }
}

#[derive(CandidType, Debug, Deserialize, Clone)]
pub struct PartialConsent {
    nik: NIK,
    allowed_headers: Vec<EmrHeader>,
}

impl PartialConsent {
    pub fn new(nik: NIK, allowed_headers: Vec<EmrHeader>) -> Self {
        Self { nik, allowed_headers }
    }
}

// we intentionally use heap memory here as we dont need the persistance of stable memory here.
struct ConsentMap {
    inner: std::collections::HashMap<ConsentCode, Consent>,
    sessions: std::collections::HashMap<SessionId, ConsentCode>,
    rng: CanisterRandomSource,
}

impl ConsentMap {
    pub fn new_with_seed(seed: u128) -> Self {
        ConsentMap {
            sessions: std::collections::HashMap::new(),
            inner: std::collections::HashMap::new(),
            rng: CanisterRandomSource::new_with_seed(seed),
        }
    }

    pub async fn new() -> Self {
        ConsentMap {
            sessions: std::collections::HashMap::new(),
            inner: std::collections::HashMap::new(),
            rng: CanisterRandomSource::new().await,
        }
    }

    /// call this every canister initialization
    pub fn init() {
        ic_cdk_timers::set_timer(Duration::from_secs(3), || {
            ic_cdk::spawn(async move {
                let new_cell = ConsentMap::new().await;

                CONSENTS.replace(Some(new_cell));

                log!("consents map initialized");

                INIT_FLAG.replace(true);
            });
        });
    }

    pub fn add_consent(&mut self, partial: PartialConsent) -> ConsentCode {
        let random = self.rng.raw_random_u64();

        let code = ConsentCode::from_u64(random);
        let consent = Consent::from_partial(partial, code.clone());

        assert!(self.inner.insert(code, consent).is_none());

        code
    }

    pub fn ensure_header_exists_and_allowed(
        &self,
        code: &ConsentCode,
        header: &EmrHeader,
        session_id: &Id
    ) -> bool {
        match self.inner.get(code) {
            Some(consent) =>
                consent.allowed_headers.contains(header) &&
                    consent.claimed &&
                    consent.session_id.as_ref().eq(&Some(session_id)),
            None => false,
        }
    }

    pub fn ensure_session_allowed(&self, code: &ConsentCode, session_id: &Id) -> bool {
        match self.inner.get(code) {
            Some(consent) => consent.claimed && consent.session_id.as_ref().eq(&Some(session_id)),
            None => false,
        }
    }

    pub fn emr_list(&self, code: &ConsentCode, session_id: &Id) -> Result<Vec<EmrHeader>, String> {
        if !self.ensure_session_allowed(code, session_id) {
            return Err("invalid session".to_string());
        }

        let consent = self.inner.get(code).unwrap();

        Ok(consent.allowed_headers.clone())
    }

    pub fn get_consent(&self, code: &ConsentCode) -> Option<&Consent> {
        self.inner.get(code)
    }

    pub fn remove_consent(&mut self, code: &ConsentCode) -> Option<Consent> {
        self.inner.remove(code)
    }

    pub fn finish_session(&mut self, session_id: &SessionId) {
        let code = self.sessions.remove(session_id);

        // remove the consent if the session is finished, no-op if the consent is already removed
        if let Some(ref code) = code {
            self.remove_consent(&code);
        }
    }

    /// wil return none if consent is already claimed or does not exist
    pub fn claim_consent(&mut self, code: &ConsentCode) -> Option<SessionId> {
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

        assert!(self.sessions.get(&session_id).is_none(), "session id already exists");
        assert!(self.sessions.insert(session_id.clone(), code.to_owned()).is_none());

        Some(session_id)
    }

    /// resolve a given session id to consent if it has been claimed,
    /// will remove the session if the consent is already removed.
    pub fn resolve_session(&mut self, session_id: &SessionId) -> Option<Consent> {
        let code = self.sessions.get(session_id);

        let Some(code) = code else {
            return None;
        };

        // return the consent if it exists
        self.inner.get(code).cloned()
    }
}

#[cfg(test)]
mod tests {
    use candid::Principal;
    use canister_common::id;

    use super::*;

    #[test]
    fn test_consent() {
        let mut consents = ConsentMap::new_with_seed(0);

        let nik = NIK::from_str(
            &"9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c".to_string()
        ).unwrap();

        let partial = PartialConsent::new(nik.clone(), vec![]);

        let code = consents.add_consent(partial.clone());

        assert_eq!(consents.get_consent(&code).unwrap().nik, nik);

        consents.remove_consent(&code);

        assert_eq!(consents.get_consent(&code), None);
    }

    #[test]
    fn test_claim_consent() {
        let mut consents = ConsentMap::new_with_seed(0);

        let nik = NIK::from_str(
            &"9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c".to_string()
        ).unwrap();

        let partial = PartialConsent::new(nik.clone(), vec![]);

        let code = consents.add_consent(partial.clone());

        let session_id = consents.claim_consent(&code).unwrap();

        assert_eq!(consents.get_consent(&code).unwrap().session_id.as_ref().unwrap(), &session_id);

        assert!(consents.claim_consent(&code).is_none());
    }

    #[test]
    fn test_claim_ensure_allowed() {
        let mut consents = ConsentMap::new_with_seed(0);

        let nik = NIK::from_str(
            &"9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c".to_string()
        ).unwrap();

        let header = EmrHeader {
            user_id: nik.clone(),
            emr_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            provider_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            registry_id: Principal::anonymous().into(),
        };

        let partial = PartialConsent::new(nik.clone(), vec![header.clone()]);

        let code = consents.add_consent(partial.clone());

        let session_id = consents.claim_consent(&code).unwrap();

        assert!(consents.ensure_header_exists_and_allowed(&code, &header, &session_id));
    }
    #[test]
    fn test_claim_ensure_allowed_fail() {
        let mut consents = ConsentMap::new_with_seed(0);

        let nik = NIK::from_str(
            &"9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c".to_string()
        ).unwrap();

        let header = EmrHeader {
            user_id: nik.clone(),
            emr_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            provider_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            registry_id: Principal::anonymous().into(),
        };

        let partial = PartialConsent::new(nik.clone(), vec![header.clone()]);
        let code = consents.add_consent(partial.clone());

        let session_id = consents.claim_consent(&code).unwrap();

        let header = EmrHeader {
            user_id: nik.clone(),
            emr_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            provider_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            registry_id: Principal::from_text("s55qq-oqaaa-aaaaa-aaakq-cai").unwrap().into(),
        };

        assert!(!consents.ensure_header_exists_and_allowed(&code, &header, &session_id));
    }

    #[test]
    fn session() {
        let mut consents = ConsentMap::new_with_seed(0);

        let nik = NIK::from_str(
            &"9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c".to_string()
        ).unwrap();

        let header = EmrHeader {
            user_id: nik.clone(),
            emr_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            provider_id: id!("97780ca3-a626-4fc5-b150-7fa8bc665df6"),
            registry_id: Principal::anonymous().into(),
        };

        let partial = PartialConsent::new(nik.clone(), vec![header.clone()]);
        let code = consents.add_consent(partial.clone());

        let session_id = consents.claim_consent(&code).unwrap();

        assert!(consents.ensure_session_allowed(&code, &session_id));

        consents.finish_session(&session_id);

        assert!(!consents.ensure_session_allowed(&code, &session_id));
        assert!(consents.resolve_session(&session_id).is_none());
        assert!(consents.get_consent(&code).is_none());
    }
}
