use std::ops::Add;

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

use super::{binding::EmrIdCollection, OutOfMemory};

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

#[derive(Default)]
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

    pub fn register_new_provider(
        &mut self,
        provider_principal: ProviderPrincipal,
        display_name: String,
    ) -> Result<(), OutOfMemory> {
        // create a new provider, note that this might change version depending on the version of the emr used.
        let provider = ProviderV001::new(display_name, provider_principal.clone())?;

        // bind the principal to the internal id
        self.providers_bindings
            .bind(provider_principal, provider.internal_id().clone())?;

        // add the provider to the provider map
        self.providers.add_provider(provider.into())?;

        Ok(())
    }

    pub fn suspend_provider(
        &mut self,
        provider_principal: ProviderPrincipal,
    ) -> Result<(), &'static str> {
        let Some(internal_id) = self.providers_bindings.get_internal_id(&provider_principal) else {
            return Err("provider not found");
        };

        let Some(mut provider) = self.providers.get_mut(&internal_id) else {
            return Err("provider not found");
        };

        match *provider {
            Provider::V001(ref mut provider) => {
                provider.activation_status = Status::Suspended;
            }
        }

        Ok(())
    }
}

pub type InternalProviderId = Id;
pub type ProviderPrincipal = Principal;
/// Issued emr map. used to track emr issued by a particular provider.
#[derive(Default)]
pub struct Issued(SBTreeMap<InternalProviderId, EmrIdCollection>);
deref!(Issued: SBTreeMap<InternalProviderId, EmrIdCollection>);

impl Issued {
    pub fn is_issued_by(&self, provider: &InternalProviderId, emr_id: &Id) -> bool {
        self.contains_key(provider)
    }
}

/// Healthcare principal to internal provider id map. used to track healthcare providers using [ProviderPrincipal] as key. resolve to that provider's [InternalProviderId].
/// this is used to track healthcare providers using their principal. this is needed because we want to be able to change the principal without costly update. we can just update the principal here.
#[derive(Default)]
pub struct ProvidersBindings(SBTreeMap<ProviderPrincipal, InternalProviderId>);
deref!(mut ProvidersBindings: SBTreeMap<ProviderPrincipal, InternalProviderId>);

impl ProvidersBindings {
    pub fn bind(
        &mut self,
        principal: ProviderPrincipal,
        internal_id: InternalProviderId,
    ) -> Result<(), OutOfMemory> {
        Ok(self.insert(principal, internal_id).map(|_| ())?)
    }
}

impl ProvidersBindings {
    pub fn get_internal_id(
        &self,
        principal: &ProviderPrincipal,
    ) -> Option<SRef<'_, InternalProviderId>> {
        self.get(principal)
    }
}

/// Healthcare provider map. used to track healthcare providers using [InternalProviderId] as key. resolves to version aware [Provider].
#[derive(Default)]
pub struct Providers(SBTreeMap<InternalProviderId, Provider>);

impl Providers {
    pub fn add_provider(&mut self, provider: Provider) -> Result<(), OutOfMemory> {
        Ok(self
            .insert(provider.internal_id().clone(), provider)
            .map(|_| ())?)
    }
}

deref!(mut Providers: SBTreeMap<InternalProviderId, Provider>);

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

impl Billable for Provider {
    fn session(&self) -> Session {
        match self {
            Provider::V001(provider) => provider.session(),
        }
    }

    fn session_mut(&mut self) -> &mut Session {
        match self {
            Provider::V001(provider) => provider.session_mut(),
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
pub trait EssentialProviderAttributes {
    /// used to automatically derive [PartialEq], [PartialOrd], [Ord] and [Eq] for [Provider] enum members at enum level.
    fn internal_id(&self) -> &InternalProviderId;
}

/// Billable trait, this trait must be implemented for all [Provider] enum members.
pub trait Billable {
    /// returns immutable session for this provider
    fn session(&self) -> Session;

    /// returns mutable session for this provider
    fn session_mut(&mut self) -> &mut Session;

    /// increment the session for this provider, call this when issuing emr
    fn increment_session(&mut self) {
        self.session_mut().increment_session();
    }
}

// START ------------------------------ SESSION ------------------------------ START

/// Provider session, 1 session is equal to 1 emr issued by a provider. used to bill the provider.
#[derive(StableType, AsFixedSizeBytes, CandidType, Debug, Default, Clone, Copy)]
pub struct Session(u64);

// blanket impl for session
impl<T> From<T> for Session
where
    T: Into<u64>,
{
    fn from(session: T) -> Self {
        Self(session.into())
    }
}

impl Session {
    /// create a new session
    pub fn new() -> Self {
        Self::default()
    }

    /// increment the session
    pub fn increment_session(&mut self) {
        self.0.add(1);
    }

    /// reset the session, call this when the provider had settled their bill
    pub fn reset_session(&mut self) {
        self.0 = 0;
    }

    /// return inner session
    pub fn session(&self) -> u64 {
        self.0
    }
}

// END ------------------------------ SESSION ------------------------------ END

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

    /// provider session, 1 session is equal to 1 emr issued by a provider. used to bill the provider.
    session: Session,

    // TODO: discuss this. are we gonna make the billing automaticly onchain?
    // active_until:
    /// time when this provider was registered in nanosecond
    registered_at: Timestamp,

    /// time when this provider was last updated in nanosecond
    updated_at: Timestamp,
    // TODO : discuss this as to what data is gonna be collected
    // provider_details:
}
impl Billable for ProviderV001 {
    fn session(&self) -> Session {
        self.session
    }

    fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }
}

impl ProviderV001 {
    pub fn new(
        encrypted_display_name: String,
        initial_principal: Principal,
    ) -> Result<Self, OutOfMemory> {
        Ok(ProviderV001 {
            session: Session::new(),
            activation_status: Status::Verified,
            display_name: SBox::new(encrypted_display_name)?,
            internal_id: InternalProviderId::default(),
            owner_principal: initial_principal,
            registered_at: Timestamp::default(),
            updated_at: Timestamp::default(),
        })
    }
}

impl From<ProviderV001> for Provider {
    fn from(provider: ProviderV001) -> Self {
        Provider::V001(provider)
    }
}

impl EssentialProviderAttributes for ProviderV001 {
    fn internal_id(&self) -> &InternalProviderId {
        &self.internal_id
    }
}

// END ------------------------------ PROVIDER V1 ------------------------------ END
