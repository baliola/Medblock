use core::str;
use std::{ str::FromStr, time::Duration };

use candid::CandidType;
use canister_common::{ common::Id, log, random::{ CanisterRandomSource, RandomSource } };
use serde::Deserialize;

use crate::registry::NIK;

thread_local! {
    pub static CONSENTS: std::cell::RefCell<Option<Consents>> = std::cell::RefCell::new(None);
}

pub fn with_consent_mut<R>(f: impl FnOnce(&mut Consents) -> R) -> R {
    CONSENTS.with(|cell| f(&mut cell.borrow_mut().as_mut().expect("consents not initialized")))
}
// change this if you want to change the expiry time of the consent code
const EXPIRY: Duration = Duration::from_secs(60 * 5); // 5 minutes

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
    pub fn remove_after_expiry(code: ConsentCode) {
        ic_cdk_timers::set_timer(EXPIRY, move || {
            with_consent_mut(|consent| consent.remove_consent(&code))
        });
    }

    pub fn generate_consent_code(nik: NIK) -> ConsentCode {
        let code = with_consent_mut(|consents| { consents.add_consent(nik) });
        Self::remove_after_expiry(code);
        code
    }
}

// we intentionally use heap memory here as we dont need the persistance of stable memory here.
struct Consents {
    map: std::collections::HashMap<ConsentCode, NIK>,
    rng: CanisterRandomSource,
}

impl Consents {
    pub fn new_with_seed(seed: u128) -> Self {
        Consents {
            map: std::collections::HashMap::new(),
            rng: CanisterRandomSource::new_with_seed(seed),
        }
    }

    pub async fn new() -> Self {
        Consents {
            map: std::collections::HashMap::new(),
            rng: CanisterRandomSource::new().await,
        }
    }

    /// call every canister initialization
    pub async fn init() {
        ic_cdk_timers::set_timer(Duration::from_secs(3), || {
            ic_cdk::spawn(async move {
                let new_cell = Consents::new().await;

                CONSENTS.replace(Some(new_cell));

                log!("consents map initialized");
            });
        });
    }

    pub fn add_consent(&mut self, nik: NIK) -> ConsentCode {
        let random = self.rng.raw_random_u64();

        let code = ConsentCode::from_u64(random);

        assert!(self.map.insert(code.clone(), nik).is_none());

        code
    }

    // get consent and remove it from the map, just better sematics than the remove_consent
    pub fn get_consent(&mut self, code: &ConsentCode) -> Option<NIK> {
        self.map.remove(code)
    }

    pub fn remove_consent(&mut self, code: &ConsentCode) {
        self.map.remove(code);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent() {
        let mut consents = Consents::new_with_seed(0);

        let nik = NIK::from_str(&"9b11530da02ee90864b5d8ef14c95782e9c75548e4877e9396394ab33e7c9e9c".to_string()).unwrap();

        let code = consents.add_consent(nik.clone());

        assert_eq!(consents.get_consent(&code), Some(nik.clone()));

        consents.remove_consent(&code);

        assert_eq!(consents.get_consent(&code), None);
    }
}
