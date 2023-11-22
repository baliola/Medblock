//! Blind Index Modules
//! all values used as index must be hashed first to prevent data leaks.

use ic_stable_structures::Storable;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    auto_deref, bounded,
    types::{Map, Set, Users, EmrId},
    wrapper::{Bounded, Stable},
};
pub struct Index<K, V>(Set<(K, V)>)
where
    K: Storable + Ord + Clone,
    V: Storable + Ord + Clone;

#[derive(thiserror::Error, Debug)]
pub enum NikError {
    #[error("invalid length, NIK must be 16 digit")]
    InvalidLength,
    #[error("invalid format, input contains non digit character")]
    InvalidFormat,
}
/// hashed representation of 16 digit National Identification number(NIK) of indonesian citizens
#[derive(Serialize, Deserialize, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub struct NIK([u8; 32]);

impl TryFrom<String> for NIK {
    type Error = NikError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.len() != 16 {
            return Err(NikError::InvalidLength);
        }

        let mut buffer = [0_u8; 32];

        value.chars().enumerate().try_for_each(|(i, c)| {
            let digit = c.to_digit(10).ok_or(NikError::InvalidFormat)?;

            buffer[i] = digit as u8;

            Ok(())
        })?;

        Ok(Self(buffer))
    }
}
pub struct NikBlindIndex(Index<Stable<NIK>, Stable<EmrId>>);

impl NikBlindIndex {
    
}
pub struct OwnerMap(Map<Stable<Users>, Stable<NIK>>);

auto_deref! {
    OwnerMap: Map<Stable<Users>, Stable<NIK>>;
    NIK: [u8; 16];
}

bounded! {
    NIK:{
    max_size: std::mem::size_of::<NIK>() as u32,
    is_fixed: true,
};
}
