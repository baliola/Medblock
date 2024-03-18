use tiny_keccak::Hasher;


pub fn hash(data: impl AsRef<[u8]>) -> [u8; 32] {
    let mut out = [0u8; 32];

    let mut hasher = tiny_keccak::Keccak::v512();

    hasher.update(data.as_ref());

    hasher.finalize(&mut out);

    out
}
