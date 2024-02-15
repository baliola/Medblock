use std::{ cell::{ RefCell }, fmt::{ Display, Formatter } };

use candid::CandidType;
use ic_cdk::api::call::RejectionCode;



#[derive(Default)]
pub struct CanisterRandomSource {
    rng: RefCell<Vec<u8>>,
}

type Reason = String;
#[derive(CandidType, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallError(RejectionCode, Reason);

impl From<(RejectionCode, String)> for CallError {
    fn from((code, reason): (RejectionCode, String)) -> Self {
        Self(code, reason)
    }
}

impl Display for CallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error while calling canister with code : {:?} and reason : {} ", self.0, self.1)
    }
}

impl CanisterRandomSource {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn refill_from_ic(buf: &mut Vec<u8>) -> Result<(), CallError> {
        let (source,) = ic_cdk::api::management_canister::main
            ::raw_rand().await
            .map_err(CallError::from)?;

        Self::refill_from_raw(buf, source);
        Ok(())
    }

    pub fn refill_from_raw(buf: &mut Vec<u8>, raw: impl IntoIterator<Item = u8>) {
        buf.extend(raw);
    }

    /// try to get random bytes from the rng source with specified length, if the rng source is not enough, returns None
    pub fn try_get_random_bytes<const N: usize>(&self) -> Option<[u8; N]> {
        let mut rng = self.rng.borrow_mut();

        // insufficient entropy
        if rng.len() < N {
            return None;
        }

        Some(Self::drain_source(rng.as_mut()))
    }

    /// get random bytes from the rng source with specified length, if the rng source is not enough, will refill the rng source
    /// fetching random bytes from the ic
    pub async fn get_random_bytes<const N: usize>(&self) -> Result<[u8; N], CallError> {
        let mut rng = self.rng.borrow_mut();

        // insufficient entropy
        if rng.len() < N {
            Self::refill_from_ic(rng.as_mut()).await?;
        }

        Ok(Self::drain_source(rng.as_mut()))
    }

    fn drain_source<const N: usize>(rng: &mut Vec<u8>) -> [u8; N] {
        let mut bytes = [0u8; N];

        // drain rng source and fill bytes
        rng.drain(0..N)
            .enumerate()
            .for_each(|(i, b)| {
                bytes[i] = b;
            });

        bytes
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_random() {
        let rng = CanisterRandomSource::new();

        rng.rng.borrow_mut().extend([1_u8].repeat(32));

        let bytes = rng.try_get_random_bytes::<32>().unwrap();

        assert_eq!(bytes.len(), 32);
    }
}
