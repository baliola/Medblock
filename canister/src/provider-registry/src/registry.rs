use std::cell::RefCell;
use std::ops::{ Add, Deref, RangeBounds };

use candid::{ CandidType };

use canister_common::stable::CBOR;
use canister_common::statistics::traits::{ Metrics, OpaqueMetrics };
use canister_common::common::PrincipalBytes;
use ic_stable_structures::{ memory_manager, BTreeMap };
use parity_scale_codec::{ Decode, Encode };
use serde::{ Deserialize, Serialize };

use canister_common::{
    deref,
    impl_max_size,
    impl_mem_bound,
    impl_range_bound,
    metrics,
    zero_sized_state,
};
use canister_common::{
    common::{ AsciiRecordsKey, Id, Timestamp },
    stable::{ Memory, Stable, StableSet, ToStable },
    mmgr::MemoryManager,
};

type EmrId = Id;
type Principal = ic_principal::Principal;

metrics!(AllocationMetrics, LengthMetrics);

#[derive(CandidType, Deserialize, Debug, Encode, Decode, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub enum Status {
    Active,
    Suspended,
}

impl_max_size!(for Status: Status);
impl_mem_bound!(for Status: bounded; fixed_size: true);

impl Status {
    /// Returns `true` if the status is [`Suspended`].
    ///
    /// [`Suspended`]: Status::Suspended
    #[must_use]
    pub fn is_suspended(&self) -> bool {
        matches!(self, Self::Suspended)
    }

    /// Returns `true` if the status is [`Active`].
    ///
    /// [`Active`]: Status::Active
    #[must_use]
    pub fn is_active(&self) -> bool {
        matches!(self, Self::Active)
    }
}

#[derive(thiserror::Error, Debug, CandidType, serde::Deserialize)]
pub enum RegistryError {
    #[error(transparent)] IssueMapError(#[from] IssueMapError),
    #[error(transparent)] ProviderBindingMapError(#[from] ProviderBindingMapError),
}

pub type ProviderRegistryResult<T = ()> = Result<T, RegistryError>;

pub struct ProviderRegistry {
    providers: Providers,
    providers_bindings: ProvidersBindings,
    issued: Issued,
}

impl ProviderRegistry {
    pub fn new(memory_manager: &MemoryManager) -> Self {
        let providers = Providers::new(memory_manager);
        let providers_bindings = ProvidersBindings::new(memory_manager);
        let issued = Issued::new(memory_manager);

        Self { providers, providers_bindings, issued }
    }

    /// check a given emr id is validly issued by some provider principal, this function uses internal provider id to resolve the given provider.
    /// so even if the provider principal is changed, this function will still return true if the provider is the one that issued the emr.
    pub fn is_issued_by(
        &self,
        provider: &ProviderPrincipal,
        emr_id: Id,
        canister_id: Principal
    ) -> bool {
        let Ok(id) = self.providers_bindings.get_internal_id(provider) else {
            return false;
        };

        self.issued.is_issued_by(id.into_inner(), emr_id, canister_id)
    }

    pub fn issue_emr(
        &mut self,
        provider: &Principal,
        emr_id: Id,
        canister_id: Principal
    ) -> ProviderRegistryResult<()> {
        match self.providers_bindings.get(provider) {
            Some(id) => {
                self.providers.try_mutate(
                    id.into_inner(),
                    |provider| -> ProviderRegistryResult<()> {
                        provider.increment_session();
                        Ok(self.issued.issue_emr(&provider.internal_id, emr_id, canister_id)?)
                    }
                )?
            }
            None => Err(ProviderBindingMapError::ProviderDoesNotExist)?,
        }
    }

    /// check a given principal is valid and registered as provider
    pub fn is_valid_provider(&self, provider: &ProviderPrincipal) -> bool {
        self.providers_bindings.contains_key(provider)
    }

    /// register a new provider, this function will create a new provider and bind the principal to the internal id.
    pub fn register_new_provider(
        &mut self,
        provider_principal: ProviderPrincipal,
        display_name: AsciiRecordsKey<64>,
        id: Id
    ) -> ProviderRegistryResult<()> {
        // create a new provider, note that this might change version depending on the version of the emr used.
        let provider = Provider::new(display_name, id);

        // bind the principal to the internal id
        self.providers_bindings.bind(provider_principal, provider.internal_id.clone())?;

        // add the provider to the provider map
        self.providers.add_provider(provider)?;

        Ok(())
    }

    /// suspend a provider, this function will change the provider activation status to suspended.
    /// suspended provider can't do things such as issuing, and reading emr
    pub fn suspend_provider(
        &mut self,
        provider_principal: ProviderPrincipal
    ) -> ProviderRegistryResult<()> {
        match self.providers_bindings.get(&provider_principal) {
            Some(id) => {
                Ok(
                    self.providers.try_mutate(id.into_inner(), |provider| {
                        provider.suspend();
                    })?
                )
            }
            None => Err(ProviderBindingMapError::ProviderDoesNotExist)?,
        }
    }

    /// unsuspend a provider
    pub fn unsuspend_provider(
        &mut self,
        provider: &ProviderPrincipal
    ) -> ProviderRegistryResult<()> {
        match self.providers_bindings.get(provider) {
            Some(id) => {
                Ok(
                    self.providers.try_mutate(id.into_inner(), |provider| {
                        provider.unsuspend();
                    })?
                )
            }
            None => Err(ProviderBindingMapError::ProviderDoesNotExist)?,
        }
    }

    /// check if a provider is suspended
    pub fn is_provider_suspended(&self, provider: &Principal) -> ProviderRegistryResult<bool> {
        match self.providers_bindings.get(provider) {
            Some(id) => {
                Ok(
                    self.providers
                        .get_provider(id.into_inner())
                        .map(|provider| provider.activation_status.is_suspended())
                        .ok_or(ProviderBindingMapError::ProviderDoesNotExist)?
                )
            }
            None => Err(ProviderBindingMapError::ProviderDoesNotExist)?,
        }
    }

    /// get issued emr by a provider, this function will return a vector of emr id issued by a provider.
    ///
    /// `provider`: the provider principal
    ///
    /// `anchor`: the anchor, this is used to paginate the result. the result will be returned starting from the anchor.
    /// so for example if the anchor is 0, the result will be returned starting from the first emr issued by the provider.
    /// and if the anchor is 10, the result will be returned starting from the 10th emr issued by the provider.
    ///
    /// `max`: the maximum number of emr to be returned.
    pub fn get_issued(
        &self,
        provider: &ProviderPrincipal,
        page: u64,
        limit: u64
    ) -> ProviderRegistryResult<Vec<EmrId>> {
        let internal_id = self.providers_bindings.get_internal_id(provider)?;

        Ok(
            self.issued.get_issued(internal_id.into_inner(), page, limit).map(|ids|
                ids
                    .into_iter()
                    .map(|ids| ids.into_inner())
                    .collect::<Vec<_>>()
            )?
        )
    }
}

pub type InternalProviderId = Id;
pub type ProviderPrincipal = ic_principal::Principal;

#[derive(Debug, thiserror::Error, CandidType, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum IssueMapError {
    #[error("provider not found")]
    ProviderNotFound,

    #[error("emr already issued")]
    AlreadyIssued,

    #[error("emr not found")]
    EmrNotFound,
}
#[derive(
    Debug,
    Clone,
    Encode,
    Decode,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    CandidType,
    Deserialize,
    Default
)]
pub struct Emr {
    canister_id: PrincipalBytes,
    id: EmrId,
}
impl_max_size!(for Emr: EmrId, Principal);
impl_mem_bound!(for Emr: bounded; fixed_size: true);
impl_range_bound!(Emr);

pub type IssueMapResult<T> = Result<T, IssueMapError>;
/// Issued emr map. used to track emr issued by a particular provider.
pub struct Issued(StableSet<Stable<InternalProviderId>, Stable<Emr>>);
deref!(mut Issued: StableSet<Stable<InternalProviderId>, Stable<Emr>>);

impl Issued {
    fn new(memory_manager: &MemoryManager) -> Self {
        Self(StableSet::new(memory_manager))
    }
    fn provider_exists(&self, provider: InternalProviderId) -> bool {
        self.range_key_exists(&provider.to_stable())
    }

    pub fn is_issued_by(
        &self,
        provider: InternalProviderId,
        emr_id: Id,
        canister_id: Principal
    ) -> bool {
        let key = provider.to_stable();
        let value = (Emr {
            canister_id: PrincipalBytes::from(canister_id),
            id: emr_id,
        }).to_stable();
        self.0.contains_key(key, value)
    }

    pub fn issue_emr(
        &mut self,
        provider: &InternalProviderId,
        emr_id: Id,
        canister_id: Principal
    ) -> IssueMapResult<()> {
        if self.is_issued_by(provider.clone(), emr_id.clone(), canister_id.clone()) {
            return Err(IssueMapError::AlreadyIssued);
        }
        let emr = (Emr {
            canister_id: PrincipalBytes::from(canister_id),
            id: emr_id,
        }).to_stable();

        self.0.insert(provider.clone().to_stable(), emr);
        Ok(())
    }

    pub fn get_issued(
        &self,
        provider: InternalProviderId,
        page: u64,
        limit: u64
    ) -> IssueMapResult<Vec<Stable<EmrId>>> {
        if !self.provider_exists(provider.clone()) {
            return Err(IssueMapError::ProviderNotFound);
        }

        match self.get_set_associated_by_key_paged(&provider.to_stable(), page, limit) {
            Some(emrs) =>
                Ok(
                    emrs
                        .into_iter()
                        .map(|emr| emr.into_inner().id.to_stable())
                        .collect()
                ),
            None => Err(IssueMapError::EmrNotFound),
        }
    }
}

/// Healthcare principal to internal provider id map. used to track healthcare providers using [ProviderPrincipal] as key. resolve to that provider's [InternalProviderId].
/// this is used to track healthcare providers using their principal. this is needed because we want to be able to change the principal without costly update. we can just update the principal here.
pub struct ProvidersBindings(BTreeMap<ProviderPrincipal, Stable<InternalProviderId>, Memory>);
deref!(mut ProvidersBindings: BTreeMap<ProviderPrincipal, Stable<InternalProviderId>, Memory>);
#[derive(Debug, thiserror::Error, CandidType, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProviderBindingMapError {
    #[error("operation not permitted, provider exists")]
    ProviderExist,
    #[error("operation not permitted, provider does not exist")]
    ProviderDoesNotExist,
}

pub type ProviderBindingMapResult<T = ()> = Result<T, ProviderBindingMapError>;

impl ProvidersBindings {
    pub fn revoke(&mut self, principal: &ProviderPrincipal) -> ProviderBindingMapResult {
        self.0
            .remove(principal)
            .map(|_| ())
            .ok_or(ProviderBindingMapError::ProviderDoesNotExist)
    }

    pub fn bind(
        &mut self,
        provider: ProviderPrincipal,
        internal_id: InternalProviderId
    ) -> ProviderBindingMapResult {
        if self.get_internal_id(&provider).is_ok() {
            return Err(ProviderBindingMapError::ProviderExist);
        }

        let _ = self.0.insert(provider, internal_id.to_stable());
        Ok(())
    }

    pub fn rebind(
        &mut self,
        provider: ProviderPrincipal,
        internal_id: InternalProviderId
    ) -> ProviderBindingMapResult {
        if self.get_internal_id(&provider).is_err() {
            return Err(ProviderBindingMapError::ProviderDoesNotExist);
        }

        let _ = self.0.insert(provider, internal_id.to_stable());
        Ok(())
    }

    /// will return an error if owner does not exists
    pub fn get_internal_id(
        &self,
        provider: &ProviderPrincipal
    ) -> ProviderBindingMapResult<Stable<InternalProviderId>> {
        self.0.get(provider).ok_or(ProviderBindingMapError::ProviderDoesNotExist)
    }

    pub fn new(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory(ic_stable_structures::BTreeMap::new))
    }

    pub fn is_valid_owner(&self, provider: &ProviderPrincipal) -> bool {
        self.0.contains_key(provider)
    }
}

#[derive(Default)]
pub struct ProviderMetrics {
    approx_allocation_bytes: u128,
}

impl ProviderMetrics {
    pub fn record_allocation(&mut self, bytes: u128) {
        self.approx_allocation_bytes += bytes;
    }

    pub fn record_deallocation(&mut self, bytes: u128) {
        self.approx_allocation_bytes -= bytes;
    }
}

pub struct Providers {
    map: BTreeMap<Stable<InternalProviderId>, Stable<Provider>, Memory>,
    metrics: RefCell<ProviderMetrics>,
}

impl std::fmt::Debug for Providers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let provider = self.map
            .iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<Vec<_>>();

        f.debug_struct("Providers")
            .field("approx_allocation_bytes", &self.metrics.borrow().approx_allocation_bytes)
            .field("map", &provider)
            .finish()
    }
}

impl OpaqueMetrics for Providers {
    fn measure(&self) -> String {
        [Metrics::<LengthMetrics>::measure(self), Metrics::<AllocationMetrics>::measure(self)].join(
            "\n"
        )
    }

    fn update(&self) {
        todo!()
    }
}

impl Metrics<LengthMetrics> for Providers {
    fn metrics_name() -> &'static str {
        "providers"
    }

    fn metrics_measurements() -> &'static str {
        "length"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.map.len().to_string()
    }
}

impl Metrics<AllocationMetrics> for Providers {
    fn metrics_name() -> &'static str {
        "providers_approx_allocation"
    }

    fn metrics_measurements() -> &'static str {
        "bytes"
    }

    fn update_measurements(&self) {
        // no-op, will be updated incrementally
    }

    fn get_measurements(&self) -> String {
        self.metrics.borrow().approx_allocation_bytes.to_string()
    }
}

impl Providers {
    pub fn add_provider(&mut self, provider: Provider) -> ProviderBindingMapResult<()> {
        match self.is_exist(provider.internal_id.clone()) {
            true => Err(ProviderBindingMapError::ProviderExist),
            false => {
                let bytes_allocated_approx =
                    std::mem::size_of_val(&provider.internal_id) + std::mem::size_of_val(&provider);

                let result = self.map
                    .insert(provider.internal_id.clone().to_stable(), provider.to_stable())
                    .map(|_| ());

                assert_eq!(result.is_none(), true, "provider does not exist, this is a bug");

                self.metrics.borrow_mut().record_allocation(bytes_allocated_approx as u128);

                Ok(())
            }
        }
    }

    fn update_unchecked(&mut self, provider: Stable<Provider>) {
        let _ = self.map.insert(provider.internal_id.clone().to_stable(), provider);
    }

    /// try mutate a provider, will return [ProviderBindingMapError::ProviderDoesNotExist] if the provider does not exist
    pub fn try_mutate<T>(
        &mut self,
        provider: InternalProviderId,
        f: impl FnOnce(&mut Stable<Provider>) -> T
    ) -> ProviderBindingMapResult<T> {
        let raw = self.map.get(&provider.to_stable());

        match raw {
            Some(mut provider) => {
                let result = f(&mut provider);

                self.update_unchecked(provider);

                Ok(result)
            }
            None => Err(ProviderBindingMapError::ProviderDoesNotExist),
        }
    }

    pub fn is_exist(&self, provider: InternalProviderId) -> bool {
        self.map.contains_key(&provider.to_stable())
    }

    pub fn new(memory_manager: &MemoryManager) -> Self {
        let mem = memory_manager.get_memory(ic_stable_structures::BTreeMap::new);
        let metrics = ProviderMetrics::default();

        Self { map: mem, metrics: RefCell::new(metrics) }
    }

    pub fn get_provider(&self, provider: InternalProviderId) -> Option<Stable<Provider>> {
        self.map.get(&provider.to_stable())
    }
}

#[cfg(test)]
mod provider_test {
    use super::*;

    #[test]
    fn test_add_read() {
        let mut memory_manager = MemoryManager::new();
        let mut providers = Providers::new(&memory_manager);

        let mut id_bytes = [0; 10];
        id_bytes.fill(0);

        let internal_id = Id::new(&id_bytes);
        let provider = Provider::new(
            AsciiRecordsKey::<64>::new("test".to_string()).unwrap(),
            internal_id.clone()
        );

        println!("{:?}", providers);
        let _ = providers.add_provider(provider.clone());
        println!("{:?}", providers);
        let provider = providers.get_provider(internal_id).unwrap();

        assert_eq!(provider, provider);
    }

    #[test]
    fn test_metrics() {
        let mut memory_manager = MemoryManager::new();
        let mut providers = Providers::new(&memory_manager);

        let mut id_bytes = [0; 10];
        id_bytes.fill(0);

        let internal_id = Id::new(&id_bytes);
        let provider = Provider::new(
            AsciiRecordsKey::<64>::new("test".to_string()).unwrap(),
            internal_id.clone()
        );

        let bytes_allocated_approx =
            std::mem::size_of_val(&internal_id) + std::mem::size_of_val(&provider);

        providers.add_provider(provider.clone()).unwrap();

        assert_eq!(
            providers.metrics.borrow().approx_allocation_bytes,
            bytes_allocated_approx as u128
        );
        println!("{:?}", <Providers as OpaqueMetrics>::measure(&providers));
    }
}

pub trait Billable {
    /// returns immutable session for this provider
    fn session(&self) -> Session;

    /// returns mutable session for this provider
    fn session_mut(&mut self) -> &mut Session;

    /// increment the session for this provider, call this when issuing emr
    fn increment_session(&mut self) {
        self.session_mut().increment_session();
    }

    fn reset_session(&mut self) {
        self.session_mut().reset_session();
    }
}

// START ------------------------------ SESSION ------------------------------ START

/// Provider session, 1 session is equal to 1 emr issued by a provider. used to bill the provider.
#[derive(
    Deserialize,
    CandidType,
    Debug,
    Default,
    Clone,
    Copy,
    Encode,
    Decode,
    PartialEq,
    Eq,
    PartialOrd,
    Ord
)]
pub struct Session(u64);

// blanket impl for session
impl<T> From<T> for Session where T: Into<u64> {
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
        let _ = self.0.add(1);
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

// START ------------------------------ PROVIDER------------------------------ START

/// Healthcare provider representaion which have and internal
/// canister identifier that is used to identify the provider. that means, whichever principal
/// that is associated with this provider internal id is the principal that can issue emr for this provider.
/// this also makes it possible to change the underlying principal without costly update.
#[derive(Clone, Encode, Decode, Debug, PartialEq, Eq, CandidType, Deserialize)]
pub struct Provider {
    /// provider activation status, this is used to track if the provider is still active
    /// can either be verified or suspended
    activation_status: Status,

    /// encrypted display name for health provider
    display_name: AsciiRecordsKey<64>,

    /// internal identifier for this provider
    /// we separate this from the principal because we want to be able to change the principal
    /// incase the health provider lost or somehow can't access their underlying internet identity
    internal_id: InternalProviderId,

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

impl Provider {
    pub fn new(display_name: AsciiRecordsKey<64>, internal_id: InternalProviderId) -> Self {
        Self {
            activation_status: Status::Active,
            display_name,
            internal_id,
            session: Session::default(),
            registered_at: Timestamp::new(),
            updated_at: Timestamp::new(),
        }
    }

    pub fn suspend(&mut self) {
        self.activation_status = Status::Suspended;
    }

    pub fn unsuspend(&mut self) {
        self.activation_status = Status::Active;
    }

    pub fn incretment_session(&mut self) {
        self.session.increment_session();
    }

    pub fn reset_session(&mut self) {
        self.session.reset_session();
    }
}

impl_max_size!(for Provider: Provider);
impl_mem_bound!(for Provider: bounded; fixed_size: false);

impl Billable for Provider {
    fn session(&self) -> Session {
        self.session
    }

    fn session_mut(&mut self) -> &mut Session {
        &mut self.session
    }
}

// END ------------------------------ PROVIDER V1 ------------------------------ END
