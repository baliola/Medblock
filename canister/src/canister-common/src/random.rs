use std::{ fmt::{ Display, Formatter } };

use candid::CandidType;
use ic_cdk::api::call::RejectionCode;
use rand::{ rngs::StdRng, Rng, SeedableRng };
use tiny_keccak::Hasher;

pub trait RandomSource {
    fn get_random_bytes(&mut self) -> [u8; 32];

    fn raw_random_u64(&mut self) -> u64;

    #[allow(async_fn_in_trait)]
    async fn reseed(&mut self) {}

    fn state(&self) -> u128 {
        0
    }
}

pub struct CanisterRandomSource {
    rng: StdRng,
}

impl RandomSource for CanisterRandomSource {
    fn get_random_bytes(&mut self) -> [u8; 32] {
        self.random_bytes()
    }

    async fn reseed(&mut self) {
        self.reseed().await;
    }

    fn raw_random_u64(&mut self) -> u64 {
        // to ensure that the random number is more than 6 digit
        self.rng.gen_range(1_000_000_000_u64..u64::MAX)
    }
}

type Reason = String;
#[derive(CandidType, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct CallError(RejectionCode, Reason);

impl CallError {
    pub fn code(&self) -> RejectionCode {
        self.0
    }

    pub fn reason(&self) -> Reason {
        self.1.clone()
    }
}

impl std::error::Error for CallError {}

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
        write!(f, "Error while calling canister with code : {:?} and reason : {} ", self.0, self.1)
    }
}

impl CanisterRandomSource {
    pub async fn new() -> Self {
        let bytes = Self::random_ic_bytes().await.expect(
            "internal rng initialization should succeed"
        );

        let mut out = [0; 32];

        let mut hasher = tiny_keccak::Keccak::v512();
        hasher.update(&bytes);
        hasher.finalize(&mut out);

        let mut seed = [0; 8];
        seed.copy_from_slice(&out[0..8]);

        let rng = StdRng::seed_from_u64(u64::from_ne_bytes(seed));

        Self {
            rng,
        }
    }

    /// float prng, hashed with keccak
    pub fn random_bytes(&mut self) -> [u8; 32] {
        let rng = &mut self.rng;

        let mut bytes = [0; 32];
        rng.fill(&mut bytes);

        let mut out = [0; 32];

        let mut hasher = tiny_keccak::Keccak::v512();

        hasher.update(&bytes);

        hasher.finalize(&mut out);

        out
    }

    pub async fn reseed(&mut self) {
        let bytes = Self::random_ic_bytes().await.expect(
            "internal rng initialization should succeed"
        );

        let mut out = [0; 32];

        let mut hasher = tiny_keccak::Keccak::v512();
        hasher.update(&bytes);
        hasher.finalize(&mut out);

        let mut seed = [0; 8];
        seed.copy_from_slice(&out[0..8]);

        self.rng = StdRng::seed_from_u64(u64::from_ne_bytes(seed));
    }

    pub fn new_with_seed(seed: u64) -> Self {
        let rng = StdRng::seed_from_u64(seed);

        Self {
            rng,
        }
    }

    async fn random_ic_bytes() -> Result<[u8; 32], CallError> {
        let (source,) = ic_cdk::api::management_canister::main
            ::raw_rand().await
            .map_err(CallError::from)?;
        let mut bytes = [0; 32];

        bytes.copy_from_slice(&source);

        Ok(bytes)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const STATIC_SEED: u64 = 30000;

    #[test]
    fn test_random_bytes() {
        let mut source = CanisterRandomSource::new_with_seed(STATIC_SEED);

        let mut prev = source.get_random_bytes();

        for _ in 0..1000 {
            let bytes = source.get_random_bytes();

            assert_ne!(prev, bytes);

            prev = bytes;
        }
    }
}
