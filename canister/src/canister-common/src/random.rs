use std::{ cell::RefCell, fmt::{ Display, Formatter }, ops::Add };

use candid::CandidType;
use ic_cdk::api::call::RejectionCode;

use crate::common::traits::ScheduledTask;

pub trait RandomSource {
    #[allow(async_fn_in_trait)]
    async fn get_random_bytes<const N: usize>(&self) -> Result<[u8; N], CallError>;
}

#[derive(Default)]
pub struct CanisterRandomSource {
    rng: RefCell<Vec<u8>>,
    cycle_threshold: u64,
}

impl ScheduledTask for CanisterRandomSource {
    fn interval() -> std::time::Duration {
        // 2.5 seconds to account for update calls (~2 seconds) and latency (~0.5 seconds)
        std::time::Duration::from_secs(2) + std::time::Duration::from_millis(500)
    }

    fn update(&self) {
        let len = self.rng.borrow().len();
        let threshold = self.cycle_threshold;

        if len < (threshold as usize) {
            let rng = self.rng.clone();

            ic_cdk::spawn(async move {
                let rng = rng;
                let mut rng = rng.borrow_mut();

                match Self::refill_from_ic(rng.as_mut()).await {
                    Ok(_) => ic_cdk::eprintln!("refilled source"),
                    Err(e) => ic_cdk::eprintln!("error refilling canister random source :{}", e),
                }
            })
        }
    }
}

impl RandomSource for CanisterRandomSource {
    async fn get_random_bytes<const N: usize>(&self) -> Result<[u8; N], CallError> {
        self.get_random_bytes::<N>().await
    }
}

type Reason = String;
#[derive(CandidType, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallError(RejectionCode, Reason);

impl From<(RejectionCode, String)> for CallError {
    fn from((code, reason): (RejectionCode, String)) -> Self {
        Self(code, reason)
    }
}

impl From<CallError> for String {
    fn from(value: CallError) -> Self {
        value.to_string()
    }
}

impl Display for CallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Error while calling ic management canister with code : {:?} and reason : {} ",
            self.0,
            self.1
        )
    }
}

impl CanisterRandomSource {
    pub fn new(threshold: u64) -> Self {
        Self {
            rng: Default::default(),
            cycle_threshold: threshold,
        }
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
    const RANDOM_THRESHOLD: u64 = 30000;

    #[test]
    fn test_get_random() {
        let rng = CanisterRandomSource::new(RANDOM_THRESHOLD);

        rng.rng.borrow_mut().extend([1_u8].repeat(32));

        let bytes = rng.try_get_random_bytes::<32>().unwrap();

        assert_eq!(bytes.len(), 32);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub const RANDOM_THRESHOLD: u64 = 30000;

    #[test]
    fn test_new() {
        let rng = CanisterRandomSource::new(RANDOM_THRESHOLD);
        assert!(rng.rng.borrow().is_empty());
    }

    #[test]
    fn test_refill_from_raw() {
        let mut rng = CanisterRandomSource::new(RANDOM_THRESHOLD);
        CanisterRandomSource::refill_from_raw(
            rng.rng.borrow_mut().as_mut(),
            vec![1, 2, 3, 4].into_iter()
        );
        assert_eq!(*rng.rng.borrow(), vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_try_get_random_bytes() {
        let rng = CanisterRandomSource::new(RANDOM_THRESHOLD);
        rng.rng.borrow_mut().extend([1_u8].repeat(32));

        let bytes = rng.try_get_random_bytes::<16>().unwrap();
        assert_eq!(bytes.len(), 16);
        assert_eq!(bytes, [1_u8; 16]);

        // After draining, there should be 16 bytes left
        assert_eq!(rng.rng.borrow().len(), 16);
    }

    #[test]
    fn test_drain_source() {
        let mut rng = vec![1_u8, 2, 3, 4, 5, 6, 7, 8];
        let bytes = CanisterRandomSource::drain_source::<4>(&mut rng);
        assert_eq!(bytes, [1_u8, 2, 3, 4]);

        // After draining, there should be 4 bytes left
        assert_eq!(rng, vec![5, 6, 7, 8]);
    }
}
