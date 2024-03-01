//! crate related utils, there might be some function available only on tests environment

/// hash anything, uses sha3. currently only available on tests
#[cfg(test)]
pub fn hash(data: impl AsRef<[u8]>) -> [u8; 32] {
    use tiny_keccak::Hasher;
    let mut out = [0u8; 32];

    let mut hasher = tiny_keccak::Sha3::v512();

    hasher.update(data.as_ref());

    hasher.finalize(&mut out);

    out
}
