use std::ops::{ Add };

use candid::{ CandidType };

use canister_common::random::CallError;
use canister_common::stable::Candid;
use canister_common::statistics::traits::{ Metrics };
use canister_common::common::{ self, EmrHeader, EmrId, PrincipalBytes };
use ic_principal::Principal;
use ic_stable_structures::{ BTreeMap };
use parity_scale_codec::{ Decode, Encode };
use serde::{ Deserialize, Serialize };
use provider::attr::*;

use canister_common::{
    deref,
    impl_max_size,
    impl_mem_bound,
    impl_range_bound,
    metrics,
    opaque_metrics,
};
use canister_common::{
    common::{ AsciiRecordsKey, Id, Timestamp },
    stable::{ Memory, Stable, StableSet, ToStable },
    mmgr::MemoryManager,
};

use crate::api::{ IssueEmrRequest };
use crate::declarations::emr_registry::{ CreateEmrRequest, CreateEmrResponse };
use crate::declarations::patient_registry::IssueRequest;

use self::provider::{ Provider, V1 };

#[derive(thiserror::Error, Debug, CandidType)]
pub enum RegistryError {
    #[error(transparent)] IssueMapError(#[from] IssueMapError),
    #[error(transparent)] ProviderBindingMapError(#[from] ProviderBindingMapError),
    #[error("{0}")] ExternalCallError(#[from] CallError),
}

pub type ProviderRegistryResult<T = ()> = Result<T, RegistryError>;

pub struct ProviderRegistry {
    providers: Providers,
    providers_bindings: ProvidersBindings,
    issued: Issued,
}

impl ProviderRegistry {
    pub fn provider_info_with_principal(
        &self,
        principal: &Principal
    ) -> ProviderRegistryResult<Provider> {
        let internal_id = self.providers_bindings.get_internal_id(principal)?;

        Ok(
            self.providers
                .get_provider(internal_id.into_inner())
                .ok_or(
                    RegistryError::ProviderBindingMapError(
                        ProviderBindingMapError::ProviderDoesNotExist
                    )
                )?
                .into_inner()
        )
    }
}

// update emr inter-canister call
impl ProviderRegistry {
    pub async fn do_call_update_emr(
        req: crate::api::UpdateEmrRequest,
        emr_registry: crate::declarations::emr_registry::EmrRegistry,
        patient_registry: crate::declarations::patient_registry::PatientRegistry
    ) {
        ic_cdk::spawn(async move {
            let header = req.header.clone();
            let header = crate::declarations::patient_registry::EmrHeader {
                provider_id: header.provider_id.to_string(),
                user_id: header.user_id.to_string(),
                emr_id: header.emr_id.to_string(),
                registry_id: header.registry_id.to_principal(),
            };

            let args = req.to_args();

            let result = emr_registry.update_emr(args).await.map_err(CallError::from);

            match result {
                Ok(_) => (),
                Err(e) => ic_cdk::trap(&format!("ERROR: error calling update_emr : {}", e)),
            }

            let args = IssueRequest { header};
            let result = patient_registry.notify_updated(args).await.map_err(CallError::from);

            match result {
                Ok(_) => (),
                Err(e) => ic_cdk::trap(&format!("ERROR: error calling update_emr : {}", e)),
            }
        });
    }
}

// issue emr inter-canister call
impl ProviderRegistry {
    pub fn build_args_call_emr_canister(
        &self,
        req: IssueEmrRequest
    ) -> ProviderRegistryResult<CreateEmrRequest> {
        // safe to unwrap since the public api calling this api should have already verified the caller using guard functions
        let provider_principal = common::guard::verified_caller().unwrap();
        let provider = self.providers_bindings.get_internal_id(&provider_principal)?;

        // assemble args and call emr canister to issue emr
        Ok(req.to_args(provider.into_inner()))
    }

    fn to_issue_request(
        req: &CreateEmrResponse
    ) -> crate::declarations::patient_registry::IssueRequest {
        let provider_id = req.header.provider_id.to_owned();
        let user_id = req.header.user_id.to_owned();
        let emr_id = req.header.emr_id.to_owned();
        let registry_id = req.header.registry_id.to_owned();

        crate::declarations::patient_registry::IssueRequest {
            header: crate::declarations::patient_registry::EmrHeader {
                provider_id,
                user_id,
                emr_id,
                registry_id,
            },
        }
    }

    pub async fn do_call_create_emr(
        args: CreateEmrRequest,
        emr_registry: crate::declarations::emr_registry::EmrRegistry,
        patient_registry: crate::declarations::patient_registry::PatientRegistry
    ) -> CreateEmrResponse {
        let create_emr_response = emr_registry.create_emr(args).await.map_err(CallError::from);

        // trap explicitly if not succeeded

        // TODO : further handle the error, to cover sys transient error described in : https://internetcomputer.org/docs/current/references/ic-interface-spec#reject-codes
        match create_emr_response {
            Ok((response,)) => {
                let issue_request = Self::to_issue_request(&response);

                match patient_registry.notify_issued(issue_request).await.map_err(CallError::from) {
                    Ok(_) => response,
                    Err(e) =>
                        ic_cdk::trap(&format!("ERROR: error calling patient canister : {}", e)),
                }
            }
            Err(e) => ic_cdk::trap(&format!("ERROR: error calling emr canister : {}", e)),
        }
    }
}

metrics!(ProviderRegistry: RegistryMetrics);

impl Metrics<RegistryMetrics> for ProviderRegistry {
    fn metrics_name() -> &'static str {
        "provider_registry"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        unimplemented!()
    }

    fn get_measurements(&self) -> String {
        [
            opaque_metrics!(self.providers),
            opaque_metrics!(self.providers_bindings),
            opaque_metrics!(self.issued),
        ].join("\n")
    }
}

impl ProviderRegistry {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        let providers = Providers::init(memory_manager);
        let providers_bindings = ProvidersBindings::init(memory_manager);
        let issued = Issued::init(memory_manager);

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

    fn populate_issue_map(
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
                        Ok(self.issued.issue_emr(provider.internal_id(), emr_id, canister_id)?)
                    }
                )?
            }
            None => Err(ProviderBindingMapError::ProviderDoesNotExist)?,
        }
    }

    // TODO : add a test for this. can only be tested using ic specific testing framework, as this function does intercanister call
    pub fn issue_emr(
        &mut self,
        emr_id: EmrId,
        provider_principal: &Principal
    ) -> ProviderRegistryResult<()> {
        // TODO : handle if we're using multiple emr canister
        self.populate_issue_map(
            provider_principal,
            emr_id,
            crate::declarations::emr_registry::CANISTER_ID
        )?;

        Ok(())
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
        address: AsciiRecordsKey<64>,
        id: Id
    ) -> ProviderRegistryResult<()> {
        // IMPORTANT: dont forget to change to newer version if updating provider version.

        // create a new provider, note that this might change version depending on the version of the emr used.
        let provider = V1::new(display_name, address, id).to_provider();

        // bind the principal to the internal id
        self.providers_bindings.bind(provider_principal, provider.internal_id().clone())?;

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
                        .map(|provider| provider.activation_status().is_suspended())
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
    /// `page`: the anchor, this is used to paginate the result. the result will be returned starting from the anchor.
    /// so for example if the anchor is 0, the result will be returned starting from the first emr issued by the provider.
    /// and if the anchor is 10, the result will be returned starting from the 10th emr issued by the provider.
    ///
    /// `limit`: the maximum number of emr to be returned.
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
pub type ProviderPrincipal = Principal;

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
impl_mem_bound!(for Emr: bounded; fixed_size: false);
impl_range_bound!(Emr);

pub type IssueMapResult<T> = Result<T, IssueMapError>;
// TODO : change to emr header
/// Issued emr map. used to track emr issued by a particular provider.
pub struct Issued(StableSet<Stable<InternalProviderId>, Stable<Emr>>);
deref!(mut Issued: StableSet<Stable<InternalProviderId>, Stable<Emr>>);
metrics!(Issued: SetLength);

impl Metrics<SetLength> for Issued {
    fn metrics_name() -> &'static str {
        "provider_issued"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        unimplemented!()
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
}

impl Issued {
    fn init(memory_manager: &MemoryManager) -> Self {
        Self(StableSet::init::<Self>(memory_manager))
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
        if self.is_issued_by(provider.clone(), emr_id.clone(), canister_id) {
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
metrics!(ProvidersBindings: BindingsMapLength);

impl Metrics<BindingsMapLength> for ProvidersBindings {
    fn metrics_name() -> &'static str {
        "providers_bindings"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        unimplemented!()
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
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

    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    pub fn is_valid_owner(&self, provider: &ProviderPrincipal) -> bool {
        self.0.contains_key(provider)
    }
}

pub struct Providers {
    map: BTreeMap<Stable<InternalProviderId>, Stable<Provider, Candid>, Memory>,
}

metrics!(Providers: LengthMetrics);

impl std::fmt::Debug for Providers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let provider = self.map
            .iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect::<Vec<_>>();

        f.debug_struct("Providers").field("map", &provider).finish()
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
        unimplemented!()
    }

    fn get_measurements(&self) -> String {
        self.map.len().to_string()
    }
}

impl Providers {
    pub fn add_provider(&mut self, provider: Provider) -> ProviderBindingMapResult<()> {
        match self.is_exist(provider.internal_id().clone()) {
            true => Err(ProviderBindingMapError::ProviderExist),
            false => {
                let _bytes_allocated_approx =
                    std::mem::size_of_val(&provider.internal_id()) +
                    std::mem::size_of_val(&provider);

                let result = self.map
                    .insert(provider.internal_id().clone().to_stable(), provider.to_stable())
                    .map(|_| ());

                assert!(result.is_none(), "provider does not exist, this is a bug");

                Ok(())
            }
        }
    }

    fn update_unchecked(&mut self, provider: Stable<Provider, Candid>) {
        let _ = self.map.insert(provider.internal_id().clone().to_stable(), provider);
    }

    /// try mutate a provider, will return [ProviderBindingMapError::ProviderDoesNotExist] if the provider does not exist
    pub fn try_mutate<T>(
        &mut self,
        provider: InternalProviderId,
        f: impl FnOnce(&mut Stable<Provider, Candid>) -> T
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

    pub fn init(memory_manager: &MemoryManager) -> Self {
        let mem = memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init);

        Self { map: mem }
    }

    pub fn get_provider(&self, provider: InternalProviderId) -> Option<Stable<Provider, Candid>> {
        self.map.get(&provider.to_stable())
    }
}

#[cfg(test)]
mod provider_test {
    use candid::Encode;
    use canister_common::statistics::traits::OpaqueMetrics;

    use super::*;

    #[test]
    fn test_add_read_v1() {
        let memory_manager = MemoryManager::init();
        let mut providers = Providers::init(&memory_manager);

        let mut id_bytes = [0; 10];
        id_bytes.fill(0);

        let internal_id = Id::new(&id_bytes);
        let name = "a".repeat(64);
        let provider = V1::new(
            AsciiRecordsKey::<64>::new(name.clone()).unwrap(),
            AsciiRecordsKey::<64>::new(name).unwrap(),
            internal_id.clone()
        ).to_provider();

        let _encoded_provider_size = Encode!(&provider).unwrap();
        let _ = providers.add_provider(provider.clone());

        let provider = providers.get_provider(internal_id).unwrap();

        assert_eq!(provider, provider);
    }

    #[test]
    fn test_metrics() {
        let memory_manager = MemoryManager::init();
        let mut providers = Providers::init(&memory_manager);

        let mut id_bytes = [0; 10];
        id_bytes.fill(0);

        let internal_id = Id::new(&id_bytes);
        let provider = V1::new(
            AsciiRecordsKey::<64>::new("test").unwrap(),
            AsciiRecordsKey::<64>::new("test").unwrap(),
            internal_id.clone()
        ).to_provider();

        let _bytes_allocated_approx =
            std::mem::size_of_val(&internal_id) + std::mem::size_of_val(&provider);

        providers.add_provider(provider.clone()).unwrap();

        println!("{:?}", <Providers as OpaqueMetrics>::measure(&providers));
    }
}

// TODO : make a documentation for updating provider version.
// write new data structure -> impl basic provider traits -> add test to measure encoded size v
// impl max size -> impl mem bound -> add test_add_read, update the inserted version at registry
pub mod provider {
    use super::*;

    pub mod attr {
        use super::super::*;

        #[derive(CandidType, Deserialize, Debug, Serialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
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

        pub trait ActivationSatus {
            fn activation_status(&self) -> &Status;

            fn activation_status_mut(&mut self) -> &mut Status;

            fn suspend(&mut self) {
                *self.activation_status_mut() = Status::Suspended;
            }

            fn unsuspend(&mut self) {
                *self.activation_status_mut() = Status::Active;
            }
        }

        // START ------------------------------ SESSION ------------------------------ START

        // TODO:  make this a proc macro later on

        /// Provider session, 1 session is equal to 1 emr issued by a provider. used to bill the provider.
        #[derive(
            Deserialize,
            CandidType,
            Debug,
            Default,
            Clone,
            Copy,
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

        pub trait BasicProviderAttributes {
            fn display_name(&self) -> &AsciiRecordsKey<64>;
            fn display_name_mut(&mut self) -> &mut AsciiRecordsKey<64>;

            fn internal_id(&self) -> &InternalProviderId;
            fn internal_id_mut(&mut self) -> &mut InternalProviderId;

            fn registered_at(&self) -> &Timestamp;

            fn updated_at(&self) -> &Timestamp;
            fn updated_at_mut(&mut self) -> &mut Timestamp;
        }

        pub trait ToProvider {
            fn to_provider(self) -> super::Provider;
        }

        pub trait BasicProvider: Billable +
            ActivationSatus +
            BasicProviderAttributes +
            ToProvider {}
        impl<P> BasicProvider
            for P
            where P: Billable + ActivationSatus + BasicProviderAttributes + ToProvider {}
    }
    // END ------------------------------ SESSION ------------------------------ END

    //  TODO : make this a derive macro to auto impl the necessary basic provider traits later on.

    /// Healthcare provider representaion which have and internal
    /// canister identifier that is used to identify the provider. that means, whichever principal
    /// that is associated with this provider internal id is the principal that can issue emr for this provider.
    /// this also makes it possible to change the underlying principal without costly update.
    #[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
    pub enum Provider {
        V1(V1),
    }

    impl Billable for Provider {
        fn session(&self) -> Session {
            match self {
                Self::V1(v1) => v1.session(),
            }
        }

        fn session_mut(&mut self) -> &mut Session {
            match self {
                Self::V1(v1) => v1.session_mut(),
            }
        }
    }

    impl ActivationSatus for Provider {
        fn activation_status(&self) -> &Status {
            match self {
                Self::V1(v1) => v1.activation_status(),
            }
        }

        fn activation_status_mut(&mut self) -> &mut Status {
            match self {
                Self::V1(v1) => v1.activation_status_mut(),
            }
        }
    }

    impl BasicProviderAttributes for Provider {
        fn display_name(&self) -> &AsciiRecordsKey<64> {
            match self {
                Self::V1(v1) => v1.display_name(),
            }
        }

        fn display_name_mut(&mut self) -> &mut AsciiRecordsKey<64> {
            match self {
                Self::V1(v1) => v1.display_name_mut(),
            }
        }

        fn internal_id(&self) -> &InternalProviderId {
            match self {
                Self::V1(v1) => v1.internal_id(),
            }
        }

        fn internal_id_mut(&mut self) -> &mut InternalProviderId {
            match self {
                Self::V1(v1) => v1.internal_id_mut(),
            }
        }

        fn registered_at(&self) -> &Timestamp {
            match self {
                Self::V1(v1) => v1.registered_at(),
            }
        }

        fn updated_at(&self) -> &Timestamp {
            match self {
                Self::V1(v1) => v1.updated_at(),
            }
        }

        fn updated_at_mut(&mut self) -> &mut Timestamp {
            match self {
                Self::V1(v1) => v1.updated_at_mut(),
            }
        }
    }

    impl Provider {
        pub const fn max_size() -> usize {
            V1::max_size()
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize)]
    pub struct V1 {
        /// provider activation status, this is used to track if the provider is still active
        /// can either be verified or suspended
        activation_status: Status,

        /// display name for health provider
        display_name: AsciiRecordsKey<64>,

        /// address for health provider
        address: AsciiRecordsKey<64>,

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

    impl V1 {
        pub fn new(
            display_name: AsciiRecordsKey<64>,
            address: AsciiRecordsKey<64>,
            internal_id: InternalProviderId
        ) -> Self {
            Self {
                activation_status: Status::Active,
                address,
                display_name,
                internal_id,
                session: Session::default(),
                registered_at: Timestamp::new(),
                updated_at: Timestamp::new(),
            }
        }

        pub fn incretment_session(&mut self) {
            self.session.increment_session();
        }

        pub fn reset_session(&mut self) {
            self.session.reset_session();
        }
    }

    // make this a macro also, for testing encoded len
    #[cfg(test)]
    mod v1_test {
        use canister_common::id;

        use super::*;

        #[test]
        fn test_len_encoded() {
            use candid::{ Encode, Decode };

            let name = AsciiRecordsKey::<64>::new("a".repeat(64)).unwrap();
            let s = V1::new(name.clone(), name, id!("12a1bd26-4954-4cf4-87ac-57b4f9585987"));
            let encoded = Encode!(&s).unwrap();

            println!("encoded len: {}", encoded.len());

            let decoded = Decode!(&encoded, V1).unwrap();

            assert_eq!(decoded, s);
        }
    }

    // 260 to account for serialization overhead for using candid. max size is roughly ~190 bytes.
    // all provider should make a test like [self::v1_test::test_len_encoded] to make sure the encoded size is within the limit.
    impl_max_size!(for V1: 260);
    impl_mem_bound!(for Provider: bounded; fixed_size: false);

    impl Billable for V1 {
        fn session(&self) -> Session {
            self.session
        }

        fn session_mut(&mut self) -> &mut Session {
            &mut self.session
        }
    }

    impl ActivationSatus for V1 {
        fn activation_status(&self) -> &Status {
            &self.activation_status
        }

        fn activation_status_mut(&mut self) -> &mut Status {
            &mut self.activation_status
        }
    }

    impl BasicProviderAttributes for V1 {
        fn display_name(&self) -> &AsciiRecordsKey<64> {
            &self.display_name
        }

        fn display_name_mut(&mut self) -> &mut AsciiRecordsKey<64> {
            &mut self.display_name
        }

        fn internal_id(&self) -> &InternalProviderId {
            &self.internal_id
        }

        fn internal_id_mut(&mut self) -> &mut InternalProviderId {
            &mut self.internal_id
        }

        fn registered_at(&self) -> &Timestamp {
            &self.registered_at
        }

        fn updated_at(&self) -> &Timestamp {
            &self.updated_at
        }

        fn updated_at_mut(&mut self) -> &mut Timestamp {
            &mut self.updated_at
        }
    }

    impl ToProvider for V1 {
        fn to_provider(self) -> provider::Provider {
            provider::Provider::V1(self)
        }
    }
}
