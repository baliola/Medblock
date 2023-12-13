use candid::{CandidType, Principal};
use ic_stable_memory::{
    collections::SBTreeMap,
    derive::{AsFixedSizeBytes, StableType},
    primitive::s_ref::SRef,
    SBox,
};
use serde::Deserialize;

use crate::{
    deref,
    types::{Id, Timestamp},
};

use super::binding::EmrIdCollection;

#[non_exhaustive]
#[derive(StableType, AsFixedSizeBytes, Deserialize, CandidType, Debug)]
pub enum Status {
    Verified,
    Suspended,
}

impl Status {
    /// Returns `true` if the status is [`Verified`].
    ///
    /// [`Verified`]: Status::Verified
    #[must_use]
    pub fn is_verified(&self) -> bool {
        matches!(self, Self::Verified)
    }

    /// Returns `true` if the status is [`Suspended`].
    ///
    /// [`Suspended`]: Status::Suspended
    #[must_use]
    pub fn is_suspended(&self) -> bool {
        matches!(self, Self::Suspended)
    }
}

pub struct ProviderRegistry {
    providers: Providers,
    providers_bindings: ProvidersBindings,
    issued: Issued,
}

impl ProviderRegistry {
    pub fn is_issued_by(&self, provider: &Principal, emr_id: &Id) -> bool {
        let Some(id) = self.providers_bindings.get_internal_id(provider) else {
            return false;
        };

        self.issued.is_issued_by(&id, emr_id)
    }
}

pub type InternalProviderId = Id;
pub type ProviderPrincipal = Principal;
/// Issued emr map. used to track emr issued by a particular provider.
pub struct Issued(SBTreeMap<InternalProviderId, EmrIdCollection>);

impl Issued {
    pub fn is_issued_by(&self, provider: &InternalProviderId, emr_id: &Id) -> bool {
        self.contains_key(provider)
    }
}
deref!(Issued: SBTreeMap<InternalProviderId, EmrIdCollection>);

pub struct ProvidersBindings(SBTreeMap<ProviderPrincipal, InternalProviderId>);
deref!(ProvidersBindings: SBTreeMap<ProviderPrincipal, InternalProviderId>);

impl ProvidersBindings {
    pub fn get_internal_id(
        &self,
        principal: &ProviderPrincipal,
    ) -> Option<SRef<'_, InternalProviderId>> {
        self.get(principal)
    }
}

/// Healthcare provider map. used to track healthcare providers.
pub struct Providers(SBTreeMap<InternalProviderId, Provider>);
deref!(Providers: SBTreeMap<InternalProviderId, Provider>);

#[derive(StableType, AsFixedSizeBytes, Debug)]
pub enum Provider {
    V001(ProviderV001),
}

impl EssentialProviderAttributes for Provider {
    fn internal_id(&self) -> &InternalProviderId {
        match self {
            Provider::V001(provider) => provider.internal_id(),
        }
    }
}

impl std::cmp::PartialEq for Provider {
    fn eq(&self, other: &Self) -> bool {
        self.internal_id().eq(other.internal_id())
    }
}

impl std::cmp::Eq for Provider {}

impl std::cmp::PartialOrd for Provider {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.internal_id().partial_cmp(other.internal_id())
    }
}

impl std::cmp::Ord for Provider {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.internal_id().cmp(other.internal_id())
    }
}

/// Essential provider attributes, this trait must be implemented for all [Provider] enum members.
/// also used to automatically derive [PartialEq], [PartialOrd], [Ord] and [Eq] for [Provider] enum members.
pub trait EssentialProviderAttributes {
    fn internal_id(&self) -> &InternalProviderId;
}

// START ------------------------------ PROVIDER V1 ------------------------------ START

/// Healthcare provider representaion which have and internal
/// canister identifier that is used to identify the provider. that means, whichever principal
/// that is associated with this provider internal id is the principal that can issue emr for this provider.
/// this also makes it possible to change the underlying principal without costly update.
#[derive(StableType, AsFixedSizeBytes, CandidType, Debug)]
pub struct ProviderV001 {
    /// provider activation status, this is used to track if the provider is still active
    /// can either be verified or suspended
    activation_status: Status,

    /// encrypted display name for health provider
    display_name: SBox<String>,

    /// internal identifier for this provider
    /// we separate this from the principal because we want to be able to change the principal
    /// incase the health provider lost or somehow can't access their underlying internet identity
    internal_id: InternalProviderId,

    /// provider associated principal, the principal that get set here effectively
    /// issues all the emr that this provider internal id issues.
    owner_principal: Principal,

    // TODO: discuss this. are we gonna make the billing automaticly onchain?
    // active_until:
    /// time when this provider was registered in nanosecond
    registered_at: Timestamp,

    /// time when this provider was last updated in nanosecond
    updated_at: Timestamp,
    // TODO : discuss this as to what data is gonna be collected
    // provider_details:
}

impl EssentialProviderAttributes for ProviderV001 {
    fn internal_id(&self) -> &InternalProviderId {
        &self.internal_id
    }
}

// END ------------------------------ PROVIDER V1 ------------------------------ END
