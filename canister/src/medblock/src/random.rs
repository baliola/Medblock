use std::{ num::NonZeroU32, cell::RefCell, borrow::Borrow };

const RANDOMNESS_ERROR: u32 = getrandom::Error::CUSTOM_START + 42;
pub fn fetch_random_bytes_from_ic(buf: &mut [u8]) -> Result<(), getrandom::Error> {
    let random_bytes = Randomness::random_bytes_from_ic();

    match random_bytes {
        Ok(bytes) => {
            buf.copy_from_slice(&bytes);
            Ok(())
        }
        Err(_) => Err(getrandom::Error::from(NonZeroU32::new(RANDOMNESS_ERROR).unwrap())),
    }
}

getrandom::register_custom_getrandom!(fetch_random_bytes_from_ic);

struct Randomness;

impl Randomness {
    pub fn random_bytes_from_ic() -> Result<Vec<u8>, ic_cdk::api::call::RejectionCode> {
        let tmp_buf = RefCell::default();

        let ref_buf = tmp_buf.clone();
        let task = async move {
            *ref_buf.borrow_mut() = Some(ic_cdk::api::management_canister::main::raw_rand().await);
        };

        let task = ic_cdk::spawn(task);

        tmp_buf
            .into_inner()
            .unwrap()
            .map(|(bytes,)| bytes)
            .map_err(|(code, _)| code)
    }
}
