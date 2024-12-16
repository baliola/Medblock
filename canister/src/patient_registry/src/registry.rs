use std::str::FromStr;

use candid::{CandidType, Principal};
use canister_common::{
    common::{AsciiRecordsKey, EmrHeader, Get, Id, Timestamp, UserId, H256},
    impl_max_size, impl_mem_bound, impl_range_bound, metrics,
    mmgr::MemoryManager,
    opaque_metrics,
    random::{self, CallError, CanisterRandomSource, RandomSource},
    stable::{Candid, Memory, Stable, StableSet, ToStable},
    statistics::traits::{Metrics, OpaqueMetrics},
};

use ic_stable_structures::memory_manager::MemoryId;
use serde::Deserialize;

use crate::{api::ReadEmrByIdRequest, declarations};

/// Limit the number of members in a group to 16 to prevent memory overflow, realistically no group should have more than 16 members but we might need to increase this in the future depending on the use case.
pub const MAX_GROUP_MEMBERS: usize = 16;

pub struct PatientRegistry {
    pub owner_map: OwnerMap,
    pub admin_map: AdminMap,
    pub group_map: GroupMap,
    pub group_consent_map: GroupConsentMap,
    pub emr_binding_map: EmrBindingMap,
    pub info_map: InfoMap,
    pub header_status_map: HeaderStatusMap,
    pub group_access_map: GroupAccessMap,
}

impl PatientRegistry {
    pub fn construct_get_provider_batch_args(
        principals: Vec<Principal>,
    ) -> declarations::provider_registry::ProviderInfoRequest {
        declarations::provider_registry::ProviderInfoRequest {
            provider: principals,
        }
    }

    pub async fn do_call_get_provider_batch(
        arg: declarations::provider_registry::ProviderInfoRequest,
        registry: declarations::provider_registry::ProviderRegistry,
    ) -> declarations::provider_registry::ProviderInfoResponse {
        match registry
            .get_provider_info_with_principal(arg)
            .await
            .map_err(CallError::from)
        {
            Ok((response,)) => response,
            Err(e) => {
                ic_cdk::trap(&format!("ERROR: Error calling get_provider_batch: {:?}", e));
            }
        }
    }
}

impl PatientRegistry {
    // modified update_patient_info to handle both initial and subsequent updates
    // prerequisite: nik must be bound to an owner first
    pub fn update_patient_info(
        &mut self,
        patient_principal: Principal,
        patient: Patient,
    ) -> PatientBindingMapResult {
        // Prevent anonymous principals from updating patient info
        if patient_principal == Principal::anonymous() {
            return Err(PatientRegistryError::UserDoesNotExist);
        }

        let nik = self.owner_map.get_nik(&patient_principal)?;
        let nik = nik.into_inner();

        // If entry doesn't exist, create it. If it exists, update it.
        // This handles both first-time registration and updates
        if self.info_map.0.contains_key(&nik.clone().to_stable()) {
            self.info_map.update(nik.clone(), patient)
        } else {
            self.info_map.set(nik, patient)
        }
    }

    pub fn issue_for(&mut self, nik: NIK, header: EmrHeader) -> PatientBindingMapResult {
        self.emr_binding_map.issue_for(nik, header.clone())?;
        self.header_status_map.add(header)?;
        Ok(())
    }

    pub fn get_patient_info_with_principal(
        &self,
        patient_principal: Principal,
    ) -> PatientBindingMapResult<(Patient, NIK)> {
        let nik = self.owner_map.get_nik(&patient_principal)?;
        let patient = self.info_map.get(nik.clone().into_inner())?;
        Ok((patient, nik.into_inner()))
    }

    pub fn get_patient_info(&self, patient: NIK) -> PatientBindingMapResult<Patient> {
        self.info_map.get(patient)
    }
}

impl OpaqueMetrics for PatientRegistry {
    fn update(&self) {
        // no-op
    }

    fn measure(&self) -> String {
        [
            opaque_metrics!(self.emr_binding_map),
            opaque_metrics!(self.owner_map),
        ]
        .join("\n")
    }
}

impl PatientRegistry {
    pub fn construct_args_read_emr(
        &self,
        arg: ReadEmrByIdRequest,
        user_principal: &ic_principal::Principal,
    ) -> PatientBindingMapResult<crate::declarations::emr_registry::ReadEmrByIdRequest> {
        let user_id = self
            .owner_map
            .get_nik(user_principal)?
            .into_inner()
            .to_string();

        Ok(crate::declarations::emr_registry::ReadEmrByIdRequest {
            provider_id: arg.provider_id.to_string(),
            emr_id: arg.emr_id.to_string(),
            user_id,
        })
    }

    pub async fn do_call_read_emr(
        arg: crate::declarations::emr_registry::ReadEmrByIdRequest,
        registry: crate::declarations::emr_registry::EmrRegistry,
    ) -> crate::declarations::emr_registry::ReadEmrByIdResponse {
        match registry.read_emr_by_id(arg).await.map_err(CallError::from) {
            Ok((response,)) => response,
            Err(e) => {
                ic_cdk::trap(&format!("ERROR: Error calling read_emr_by_id: {:?}", e));
            }
        }
    }
}

impl PatientRegistry {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self {
            owner_map: OwnerMap::init(memory_manager),
            admin_map: AdminMap::init(memory_manager),
            group_map: GroupMap::init(memory_manager),
            group_consent_map: GroupConsentMap::init(memory_manager),
            emr_binding_map: EmrBindingMap::init(memory_manager),
            info_map: InfoMap::init(memory_manager),
            header_status_map: HeaderStatusMap::init(memory_manager),
            group_access_map: GroupAccessMap::init(memory_manager),
        }
    }
}

impl AsRef<OwnerMap> for PatientRegistry {
    fn as_ref(&self) -> &OwnerMap {
        &self.owner_map
    }
}

impl AsMut<OwnerMap> for PatientRegistry {
    fn as_mut(&mut self) -> &mut OwnerMap {
        &mut self.owner_map
    }
}

impl AsRef<EmrBindingMap> for PatientRegistry {
    fn as_ref(&self) -> &EmrBindingMap {
        &self.emr_binding_map
    }
}

impl AsMut<EmrBindingMap> for PatientRegistry {
    fn as_mut(&mut self) -> &mut EmrBindingMap {
        &mut self.emr_binding_map
    }
}

pub type NIK = H256;
/// Principal to NIK Map. meant to enforce 1:1 relationship between principal and NIK.
/// used to claim emrs ownership. This level of inderction is needed because principal that map to a particular BindingKey effectively owns
/// all the emrs that it's BindingKey map to.
pub type Owner = ic_principal::Principal;
pub struct OwnerMap(ic_stable_structures::BTreeMap<Owner, Stable<NIK>, Memory>);
metrics!(OwnerMap: Owners);

impl Metrics<Owners> for OwnerMap {
    fn metrics_name() -> &'static str {
        "owner_map"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
}

#[derive(
    Debug, thiserror::Error, CandidType, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum PatientRegistryError {
    #[error("user exists")]
    UserExist,
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error("emr exists")]
    EmrExists,
    #[error("group has reached maximum member limit of {0}")]
    GroupFull(usize),
    #[error("NIK is already registered")]
    DuplicateNIK,
    #[error("user already in group")]
    UserAlreadyInGroup,
    #[error("group not found")]
    GroupNotFound,
    #[error("group already exists")]
    GroupAlreadyExists,
    #[error("user has no emrs")]
    UserNoEmrs,
}

// !! TODO WE REALLY NEED TO MOVE GROUP CONSENT TO A DIFFERENT FILE
#[derive(
    Debug, thiserror::Error, CandidType, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum GroupConsentMapError {
    #[error("group consent code not found")]
    GroupConsentCodeNotFound,
}

#[derive(
    Debug, thiserror::Error, CandidType, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub enum AdminMapError {
    #[error("user already bound")]
    UserAlreadyBound,
    #[error("user does not exist")]
    UserDoesNotExist,
}
pub struct GroupAccessMap(ic_stable_structures::BTreeMap<GroupAccessKey, Stable<GroupId>, Memory>);

/// Represents a key in the group access map, which is a tuple of two NIKs:
/// - The granter, who is granting access
/// - The grantee, who is receiving access
/// Granter cannot see the grantee's EMRs, but grantee can see the granter's EMRs
type GroupAccessKey = (Stable<NIK>, Stable<NIK>);

impl Get<MemoryId> for GroupAccessMap {
    fn get() -> MemoryId {
        MemoryId::new(20)
    }
}

metrics!(GroupAccessMap: GroupAccesses);

impl Metrics<GroupAccesses> for GroupAccessMap {
    fn metrics_name() -> &'static str {
        "group_access_map"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
}

impl GroupAccessMap {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    /// grants EMR access from granter to grantee within a specific group context
    pub fn grant_access(
        &mut self,
        granter: NIK,
        grantee: NIK,
        group_id: GroupId,
    ) -> Result<(), String> {
        let key = (Stable::from(granter), Stable::from(grantee));
        self.0.insert(key, Stable::from(group_id));
        Ok(())
    }

    pub fn revoke_access(&mut self, granter: NIK, grantee: NIK) -> Result<(), String> {
        let key = (Stable::from(granter), Stable::from(grantee));
        self.0.remove(&key);
        Ok(())
    }

    pub fn has_access(&self, granter: &NIK, grantee: &NIK) -> bool {
        let key = (Stable::from(granter.clone()), Stable::from(grantee.clone()));
        let result = self.0.contains_key(&key);
        println!(
            "[GroupAccessMap] has_access: {:?}, result: {:?}",
            key, result
        );
        result
    }

    /// gets the group ID in which the EMR access was granted
    pub fn get_access_group(&self, granter: &NIK, grantee: &NIK) -> Option<GroupId> {
        let key = (granter.clone().to_stable(), grantee.clone().to_stable());
        self.0
            .get(&key)
            .map(|group_id| group_id.into_inner())
            .inspect(|group_id| {
                println!(
                    "[GroupAccessMap] get_access_group: {:?}, group_id: {:?}",
                    key, group_id
                )
            })
    }

    /// gets all access pairs for a specific group
    pub fn get_group_access_pairs(&self, group_id: GroupId) -> Vec<(NIK, NIK)> {
        self.0
            .iter()
            .filter(|(_, gid)| gid.as_ref() == &group_id)
            .map(|(key, _)| (key.0.into_inner(), key.1.into_inner()))
            .collect()
    }
}

// #[cfg(test)]
// mod test_group_access_map {
//     use super::*;
//     use canister_common::memory_manager;

//     #[test]
//     fn test_grant_and_revoke_access() {
//         // cant really test this as we need to create a group first to get its id
//         todo!()
//     }

//     #[test]
//     fn test_access_verification() {
//         // cant really test this as we need to create a group first to get its id
//         todo!()
//     }
// }

pub type PatientBindingMapResult<T = ()> = Result<T, PatientRegistryError>;
pub type AdminMapResult<T = ()> = Result<T, AdminMapError>;

impl OwnerMap {
    pub fn revoke(&mut self, owner: &Owner) -> PatientBindingMapResult {
        self.0
            .remove(owner)
            .map(|_| ())
            .ok_or(PatientRegistryError::UserDoesNotExist)
    }

    pub fn bind(&mut self, owner: Owner, nik: NIK) -> PatientBindingMapResult {
        if self.get_nik(&owner).is_ok() {
            return Err(PatientRegistryError::UserExist);
        }

        if self.is_nik_in_use(&nik) {
            return Err(PatientRegistryError::DuplicateNIK);
        }

        let _ = self.0.insert(owner, nik.to_stable());
        Ok(())
    }

    pub fn rebind(&mut self, owner: Owner, nik: NIK) -> PatientBindingMapResult {
        if self.get_nik(&owner).is_err() {
            return Err(PatientRegistryError::UserDoesNotExist);
        }

        let _ = self.0.insert(owner, nik.to_stable());
        Ok(())
    }

    /// will return an error if owner does not exists
    pub fn get_nik(&self, owner: &Owner) -> PatientBindingMapResult<Stable<UserId>> {
        self.0
            .get(owner)
            .ok_or(PatientRegistryError::UserDoesNotExist)
    }

    /// returns a list of all NIKs in the owner map
    pub fn get_all_nik(&self) -> Vec<Stable<NIK>> {
        self.0.iter().map(|(_, nik)| nik.clone()).collect()
    }

    /// gets the principal associated with a NIK by iterating through the map
    pub fn get_principal(&self, nik: &NIK) -> PatientBindingMapResult<Owner> {
        self.0
            .iter()
            .find(|(_, stored_nik)| stored_nik.as_ref() == nik)
            .map(|(principal, _)| principal.clone())
            .ok_or(PatientRegistryError::UserDoesNotExist)
    }

    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    pub fn is_valid_owner(&self, owner: &Owner) -> bool {
        self.0.contains_key(owner)
    }

    pub fn is_nik_in_use(&self, nik: &NIK) -> bool {
        self.0
            .iter()
            .any(|(_, stored_nik)| stored_nik.as_ref() == nik)
    }
}

#[cfg(test)]
mod test_owner_map {
    use super::*;

    #[test]
    fn test_bind() {
        let mut owner_map = OwnerMap::init(&MemoryManager::init());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(
            owner_map.bind(owner, nik.clone()).unwrap_err(),
            PatientRegistryError::UserExist
        );
    }

    #[test]
    fn test_rebind() {
        let mut owner_map = OwnerMap::init(&MemoryManager::init());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            owner_map.rebind(owner, nik.clone()).unwrap_err(),
            PatientRegistryError::UserDoesNotExist
        );
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(owner_map.rebind(owner, nik.clone()).unwrap(), ());
    }

    #[test]
    fn test_revoke() {
        let mut owner_map = OwnerMap::init(&MemoryManager::init());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            owner_map.revoke(&owner).unwrap_err(),
            PatientRegistryError::UserDoesNotExist
        );
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(owner_map.revoke(&owner).unwrap(), ());
    }

    #[test]
    fn test_get_nik() {
        let mut owner_map = OwnerMap::init(&MemoryManager::init());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            owner_map.get_nik(&owner).unwrap_err(),
            PatientRegistryError::UserDoesNotExist
        );
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(owner_map.get_nik(&owner).unwrap(), nik.to_stable());
    }

    #[test]
    fn test_is_valid_owner() {
        let mut owner_map = OwnerMap::init(&MemoryManager::init());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert!(!owner_map.is_valid_owner(&owner));
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert!(owner_map.is_valid_owner(&owner));
    }

    #[test]
    fn test_get_all_nik() {
        let mut owner_map = OwnerMap::init(&MemoryManager::init());

        // create test data
        let owner1 = ic_principal::Principal::from_text("2vxsx-fae").unwrap();
        let owner2 = ic_principal::Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let nik1 = NIK::from([1u8; 32]);
        let nik2 = NIK::from([2u8; 32]);

        // initially should be empty
        assert!(owner_map.get_all_nik().is_empty());

        // add two owners with different NIKs
        owner_map.bind(owner1, nik1.clone()).unwrap();
        owner_map.bind(owner2, nik2.clone()).unwrap();

        // get all NIKs
        let all_niks = owner_map.get_all_nik();

        // should have exactly 2 NIKs
        assert_eq!(all_niks.len(), 2);

        // should contain both NIKs
        assert!(all_niks.iter().any(|n| n.as_ref() == &nik1));
        assert!(all_niks.iter().any(|n| n.as_ref() == &nik2));

        // test after removing an owner
        owner_map.revoke(&owner1).unwrap();
        let remaining_niks = owner_map.get_all_nik();
        assert_eq!(remaining_niks.len(), 1);
        assert_eq!(remaining_niks[0].as_ref(), &nik2);
    }
}

/// track emr issued for a particular user by storing it's emr id in this map. also used as blind index for emr search.
///
/// we don't use the principal directly because we want users to be able to change it's internet identity
/// and still be able to own and access their emr.
///
/// NIK MUST be hashed offchain before being used as key.
pub struct EmrBindingMap(StableSet<Stable<NIK>, Stable<EmrHeader>>);
metrics!(EmrBindingMap: EmrsIssued);

impl Metrics<EmrsIssued> for EmrBindingMap {
    fn metrics_name() -> &'static str {
        "issued_emrs"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
}

impl EmrBindingMap {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(StableSet::init::<Self>(memory_manager))
    }

    pub fn is_owner_of(&self, nik: NIK, header: EmrHeader) -> bool {
        self.0.contains_key(nik.to_stable(), header.to_stable())
    }

    pub fn emr_list(
        &self,
        nik: &NIK,
        page: u8,
        limit: u8,
    ) -> PatientBindingMapResult<Vec<Stable<EmrHeader>>> {
        // first check if the user exists and has any EMRs
        if !self.0.range_key_exists(&nik.clone().to_stable()) {
            return Err(PatientRegistryError::UserNoEmrs);
        }

        // get the paginated results
        match self.0.get_set_associated_by_key_paged(&nik.clone().to_stable(), page as u64, limit as u64) {
            Some(emrs) if !emrs.is_empty() => Ok(emrs),
            _ => Err(PatientRegistryError::UserNoEmrs)
        }
    }

    pub fn issue_for(&mut self, nik: NIK, header: EmrHeader) -> PatientBindingMapResult<()> {
        if self.is_owner_of(nik.clone(), header.clone()) {
            return Err(PatientRegistryError::EmrExists);
        }

        self.0.insert(nik.to_stable(), header.to_stable());
        Ok(())
    }
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct HeaderStatus {
    created_at: Timestamp,
    updated_at: Timestamp,
}

impl_max_size!(for HeaderStatus: 40);
impl_mem_bound!(for HeaderStatus: bounded; fixed_size: false);
impl_range_bound!(HeaderStatus);

#[cfg(test)]
mod header_status_test {
    use super::*;

    #[test]
    fn test_header_status() {
        use candid::{Decode, Encode};

        let header_status = HeaderStatus {
            created_at: Timestamp::new(),
            updated_at: Timestamp::new(),
        };

        let encoded = Encode!(&header_status).unwrap();

        println!("encoded: {:?}", encoded.len());

        let decoded = Decode!(&encoded, HeaderStatus).unwrap();

        assert_eq!(header_status, decoded);
    }
}

///  this is used to track the status of the emr header. Must be updated every update.
pub struct HeaderStatusMap(
    ic_stable_structures::BTreeMap<Stable<EmrHeader>, Stable<HeaderStatus, Candid>, Memory>,
);

impl HeaderStatusMap {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    pub fn add(&mut self, header: EmrHeader) -> PatientBindingMapResult {
        let key = header.to_stable();

        if self.0.contains_key(&key) {
            return Err(PatientRegistryError::EmrExists);
        }

        let status = HeaderStatus {
            created_at: Timestamp::new(),
            updated_at: Timestamp::new(),
        };

        let _ = self.0.insert(key, status.to_stable());
        Ok(())
    }

    pub fn update(&mut self, header: EmrHeader) -> PatientBindingMapResult {
        let key = header.to_stable();

        let status = self
            .0
            .get(&key)
            .ok_or(PatientRegistryError::UserDoesNotExist)?;

        let status = HeaderStatus {
            created_at: status.created_at,
            updated_at: Timestamp::new(),
        };

        let _ = self.0.insert(key, status.to_stable());
        Ok(())
    }

    pub fn get(&self, header: &Stable<EmrHeader>) -> Option<Stable<HeaderStatus, Candid>> {
        self.0.get(header)
    }
}

#[cfg(test)]
mod test_emr_binding_map {
    use candid::Principal;
    use canister_common::id;

    use super::*;

    #[test]
    fn test_issue_for() {
        let mut emr_binding_map = EmrBindingMap::init(&MemoryManager::init());
        let nik = NIK::from([0u8; 32]);

        let emr_id = id!("92fa73e0-0450-4b73-9cc2-dbd703b99f56");
        let provider_id = id!("92fa73e0-0450-4b73-9cc2-dbd703b99f56");
        let user_id = UserId::default();

        let header = EmrHeader::new(user_id, emr_id, provider_id, Principal::anonymous());

        emr_binding_map.issue_for(nik.clone(), header.clone());
        assert!(emr_binding_map.is_owner_of(nik.clone(), header.clone()));
    }

    #[test]
    fn test_emr_list() {
        let mut emr_binding_map = EmrBindingMap::init(&MemoryManager::init());
        let nik = NIK::from([0u8; 32]);

        let emr_id = id!("92fa73e0-0450-4b73-9cc2-dbd703b99f56");
        let provider_id = id!("92fa73e0-0450-4b73-9cc2-dbd703b99f56");
        let user_id = UserId::default();

        let header = EmrHeader::new(user_id, emr_id, provider_id, Principal::anonymous());

        emr_binding_map.issue_for(nik.clone(), header.clone());
        assert_eq!(
            emr_binding_map.emr_list(&nik, 0, 3).unwrap(),
            vec![header.to_stable()]
        );
    }
}

pub struct InfoMap(ic_stable_structures::BTreeMap<Stable<NIK>, Stable<Patient, Candid>, Memory>);

impl InfoMap {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    pub fn get(&self, nik: NIK) -> PatientBindingMapResult<Patient> {
        let key = nik.to_stable();

        self.0
            .get(&key)
            .ok_or(PatientRegistryError::UserDoesNotExist)
            .map(|patient| patient.into_inner())
    }

    pub fn set(&mut self, nik: NIK, patient: Patient) -> PatientBindingMapResult {
        let key = nik.to_stable();
        if self.0.contains_key(&key) {
            return Err(PatientRegistryError::UserExist);
        }

        let result = self.0.insert(key, patient.to_stable());
        assert!(result.is_none(), "info should not exist");

        Ok(())
    }

    pub fn update(&mut self, nik: NIK, patient: Patient) -> PatientBindingMapResult {
        let key = nik.to_stable();
        if !self.0.contains_key(&key) {
            return Err(PatientRegistryError::UserDoesNotExist);
        }

        let result = self.0.insert(key, patient.to_stable());
        assert!(result.is_some(), "info should exist");

        Ok(())
    }
}

#[cfg(test)]
mod info_test {
    use super::*;

    #[test]
    fn test_set() {
        let mut info_map = InfoMap::init(&MemoryManager::init());
        let nik = NIK::from([0u8; 32]);

        let patient = Patient::V1(V1::default());

        assert_eq!(info_map.set(nik.clone(), patient.clone()).unwrap(), ());
        assert_eq!(
            info_map.set(nik.clone(), patient.clone()).unwrap_err(),
            PatientRegistryError::UserExist
        );

        assert_eq!(info_map.get(nik.clone()).unwrap(), patient);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, PartialOrd, Ord)]
pub enum Patient {
    V1(V1),
}

impl Patient {
    pub fn name(&self) -> &AsciiRecordsKey<64> {
        match self {
            Self::V1(v1) => &v1.name,
        }
    }

    pub fn kyc_status(&self) -> &KycStatus {
        match self {
            Self::V1(v1) => &v1.kyc_status,
        }
    }

    pub fn update_kyc_status(&mut self, kyc_status: KycStatus) {
        match self {
            Self::V1(v1) => v1.kyc_status = kyc_status,
        }
    }
}

impl From<V1> for Patient {
    fn from(v1: V1) -> Self {
        Self::V1(v1)
    }
}

impl Default for Patient {
    fn default() -> Self {
        // change this if upgrading to a new version
        Self::V1(Default::default())
    }
}
impl_mem_bound!(for Patient: bounded; fixed_size: false);
impl_range_bound!(Patient);
impl Patient {
    pub const fn max_size() -> usize {
        V1::max_size()
    }
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum KycStatus {
    Denied,
    Pending,
    Approved,
}

impl Default for KycStatus {
    fn default() -> Self {
        Self::Pending
    }
}

#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, Default, PartialOrd, Ord)]
pub struct V1 {
    pub name: AsciiRecordsKey<64>,
    pub place_of_birth: AsciiRecordsKey,
    pub date_of_birth: AsciiRecordsKey,
    pub address: AsciiRecordsKey<64>,
    pub martial_status: AsciiRecordsKey<10>,
    pub gender: AsciiRecordsKey<10>,
    pub kyc_status: KycStatus,
    pub kyc_date: AsciiRecordsKey<32>,
}

// 270 to account for serialization overhead for using candid. max size is roughly ~190 bytes.
// benchmarked by tsting the encoded size of a struct with max size fields.
impl_max_size!(for V1: 512);
impl_mem_bound!(for V1: bounded; fixed_size: false);
impl_range_bound!(V1);

#[cfg(test)]
mod v1_test {
    use super::*;

    // ~270 bytes
    #[test]
    fn test_len_encoded() {
        use candid::Decode;
        use candid::Encode;

        let patient = V1 {
            name: AsciiRecordsKey::<64>::new("a".repeat(64)).unwrap(),
            place_of_birth: AsciiRecordsKey::<32>::new("a".repeat(32)).unwrap(),
            date_of_birth: AsciiRecordsKey::<32>::new("a".repeat(32)).unwrap(),
            address: AsciiRecordsKey::<64>::new("a".repeat(64)).unwrap(),
            martial_status: AsciiRecordsKey::<10>::new("a".repeat(10)).unwrap(),
            gender: AsciiRecordsKey::<10>::new("a".repeat(10)).unwrap(),
            kyc_status: KycStatus::Pending,
            kyc_date: AsciiRecordsKey::<32>::new("a".repeat(32)).unwrap(),
        };

        let encoded = Encode!(&patient).unwrap();
        println!("encoded: {:?}", encoded.len());
        let decoded = Decode!(&encoded, V1).unwrap();

        assert_eq!(patient, decoded);
    }
}

pub type Admin = ic_principal::Principal;
pub struct AdminMap(ic_stable_structures::BTreeMap<Admin, Stable<NIK>, Memory>);
metrics!(AdminMap: Admins);

impl Metrics<Admins> for AdminMap {
    fn metrics_name() -> &'static str {
        "admins"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
}

/// Admin map only cares about your principal. So you dont have to be a Patient to be an Admin. Any principal can be bound as admin.
impl AdminMap {
    pub fn revoke(&mut self, admin: &Admin) -> AdminMapResult {
        self.0
            .remove(admin)
            .map(|_| ())
            .ok_or(AdminMapError::UserDoesNotExist)
    }

    pub fn bind(&mut self, admin: Admin, nik: NIK) -> AdminMapResult {
        if self.get_nik(&admin).is_ok() {
            return Err(AdminMapError::UserAlreadyBound);
        }

        let _ = self.0.insert(admin, nik.to_stable());
        Ok(())
    }

    /// !!! In this case, we assume that the Principal is not bound to a NIK yet. So we will be using a random NIK for the admin.
    /// This is useful for the case where we want to add an admin without having to go through the KYC process.
    /// TODO! NEED TO CHECK FOR EDGE CASES DUE TO THIS ASSUMPTION.
    pub fn principal_only_bind(&mut self, admin: Admin) -> AdminMapResult {
        let random_nik = NIK::from([0u8; 32]).to_stable();

        let _ = self.0.insert(admin, random_nik);
        Ok(())
    }

    pub fn rebind(&mut self, admin: Admin, nik: NIK) -> AdminMapResult {
        if self.get_nik(&admin).is_err() {
            return Err(AdminMapError::UserDoesNotExist);
        }

        let _ = self.0.insert(admin, nik.to_stable());
        Ok(())
    }

    /// will return an error if owner does not exists
    pub fn get_nik(&self, admin: &Admin) -> AdminMapResult<Stable<NIK>> {
        self.0.get(admin).ok_or(AdminMapError::UserDoesNotExist)
    }

    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    pub fn is_valid_admin(&self, admin: &Admin) -> bool {
        self.0.contains_key(admin)
    }
}

#[cfg(test)]
mod test_admin_map {
    use super::*;

    #[test]
    fn test_bind() {
        let mut admin_map = AdminMap::init(&MemoryManager::init());
        let admin = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(admin_map.bind(admin, nik.clone()).unwrap(), ());

        assert_eq!(
            admin_map.bind(admin, nik).unwrap_err(),
            AdminMapError::UserAlreadyBound
        );
    }

    #[test]
    fn test_rebind() {
        let mut admin_map = AdminMap::init(&MemoryManager::init());
        let admin = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            admin_map.rebind(admin, nik.clone()).unwrap_err(),
            AdminMapError::UserDoesNotExist
        );
        assert_eq!(admin_map.bind(admin, nik.clone()).unwrap(), ());
        assert_eq!(admin_map.rebind(admin, nik.clone()).unwrap(), ());
    }

    #[test]
    fn test_revoke() {
        let mut admin_map = AdminMap::init(&MemoryManager::init());
        let admin = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            admin_map.revoke(&admin).unwrap_err(),
            AdminMapError::UserDoesNotExist
        );
        assert_eq!(admin_map.bind(admin, nik.clone()).unwrap(), ());
        assert_eq!(admin_map.revoke(&admin).unwrap(), ());
    }

    #[test]
    fn test_get_nik() {
        let mut admin_map = AdminMap::init(&MemoryManager::init());
        let admin = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            admin_map.get_nik(&admin).unwrap_err(),
            AdminMapError::UserDoesNotExist
        );
        assert_eq!(admin_map.bind(admin, nik.clone()).unwrap(), ());
        assert_eq!(admin_map.get_nik(&admin).unwrap(), nik.to_stable());
    }

    #[test]
    fn test_is_valid_owner() {
        let mut admin_map = AdminMap::init(&MemoryManager::init());
        let admin = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert!(!admin_map.is_valid_admin(&admin));
        assert_eq!(admin_map.bind(admin, nik.clone()).unwrap(), ());
        assert!(admin_map.is_valid_admin(&admin));
    }
}
#[cfg(test)]
mod test_kyc {
    use super::*;
    use canister_common::{id, memory_manager};
    use ic_principal::Principal;

    #[test]
    fn test_admin_role() {
        let memory_manager = memory_manager!();
        let mut registry = PatientRegistry::init(&memory_manager);

        let admin = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
        let non_admin = Principal::from_text("aaaaa-aa").unwrap();
        let nik = NIK::from([0u8; 32]);

        // Test adding an admin
        assert_eq!(registry.admin_map.bind(admin, nik.clone()).unwrap(), ());
        assert!(registry.admin_map.is_valid_admin(&admin));
        assert!(!registry.admin_map.is_valid_admin(&non_admin));

        // Test removing an admin
        assert_eq!(registry.admin_map.revoke(&admin).unwrap(), ());
        assert!(!registry.admin_map.is_valid_admin(&admin));
    }

    #[test]
    #[should_panic(expected = "called `Result::unwrap()` on an `Err` value: UserDoesNotExist")]
    fn test_kyc_status() {
        let memory_manager = memory_manager!();
        let mut registry = PatientRegistry::init(&memory_manager);

        let nik = NIK::from([0u8; 32]);
        let mut patient = Patient::V1(V1::default());
        let non_anonymous_principal = Principal::from_text("2vxsx-fae").unwrap();
        let anonymous_principal = Principal::anonymous();

        // need to bind nik to owner first before we can register patient
        registry
            .owner_map
            .bind(non_anonymous_principal, nik.clone())
            .unwrap();
        assert_eq!(
            registry
                .owner_map
                .get_nik(&non_anonymous_principal)
                .unwrap(),
            nik.clone().to_stable()
        );

        // register patient info
        registry
            .update_patient_info(non_anonymous_principal, patient.clone())
            .unwrap();
        assert!(registry.info_map.get(nik.clone()).is_ok());

        // Check initial KYC status
        let initial_patient = registry.get_patient_info(nik.clone()).unwrap();
        assert_eq!(initial_patient.kyc_status(), &KycStatus::Pending);

        // Update KYC status
        patient.update_kyc_status(KycStatus::Approved);
        registry
            .update_patient_info(non_anonymous_principal, patient.clone())
            .unwrap();

        // Verify updated KYC status
        let verified_patient = registry.get_patient_info(nik.clone()).unwrap();
        assert_eq!(verified_patient.kyc_status(), &KycStatus::Approved);

        // This should panic with UserDoesNotExist
        registry
            .update_patient_info(anonymous_principal, patient.clone())
            .unwrap(); // This line will panic
    }
}

pub type GroupConsentMapResult<T = ()> = Result<T, GroupConsentMapError>;

pub struct InnerGroupConsentMap(
    ic_stable_structures::BTreeMap<Stable<GroupConsentCode, Candid>, Stable<NIK>, Memory>,
);

impl InnerGroupConsentMap {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        let map = memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::new);

        InnerGroupConsentMap(map)
    }
}

pub struct GroupConsentMap {
    inner_map: InnerGroupConsentMap,
    rng: CanisterRandomSource,
}
metrics!(GroupConsentMap: GroupConsents);
impl_mem_bound!(for GroupConsentMap: bounded; fixed_size: false);
impl_range_bound!(GroupConsentMap);
impl_max_size!(for GroupConsentMap: 256);

impl Metrics<GroupConsents> for GroupConsentMap {
    fn metrics_name() -> &'static str {
        "group_consents"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.inner_map.0.len().to_string()
    }
}

impl GroupConsentMap {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self {
            inner_map: InnerGroupConsentMap::init(memory_manager),
            rng: CanisterRandomSource::new_with_seed(123),
        }
    }

    pub fn bind(
        &mut self,
        group_consent_code: GroupConsentCode,
        nik: NIK,
    ) -> GroupConsentMapResult {
        let serialized: Stable<GroupConsentCode, Candid> = group_consent_code.to_stable();
        println!("serialized: {:?}", serialized);
        assert!(serialized.0.len() == 6);

        self.inner_map
            .0
            .insert(group_consent_code.to_stable(), nik.to_stable());
        Ok(())
    }

    pub fn get_nik(&self, group_consent_code: &GroupConsentCode) -> GroupConsentMapResult<NIK> {
        self.inner_map
            .0
            .get(&group_consent_code.to_stable())
            .map(|nik| nik.into_inner())
            .ok_or(GroupConsentMapError::GroupConsentCodeNotFound)
    }

    pub fn is_group_consent_code_valid(
        &self,
        group_consent_code: &GroupConsentCode,
    ) -> GroupConsentMapResult<bool> {
        Ok(self
            .inner_map
            .0
            .contains_key(&group_consent_code.to_stable()))
    }
}

const ALLOWED_CHAR: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const CODE_LEN: usize = 6;
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupConsentCode([u8; CODE_LEN]);
impl_max_size!(for GroupConsentCode: 14);
impl_mem_bound!(for GroupConsentCode: bounded; fixed_size:false);
impl_range_bound!(GroupConsentCode);

impl GroupConsentCode {
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap()
    }

    pub fn from_text(text: &str) -> Result<Self, String> {
        Self::from_str(text)
    }

    pub fn from_u64(u: u64) -> Self {
        let str = u.to_string();
        let str = &str[str.len() - CODE_LEN..];
        let mut code = [0; CODE_LEN];

        for (i, c) in str.chars().enumerate() {
            code[i] = c as u8;
        }

        GroupConsentCode(code)
    }
}

impl FromStr for GroupConsentCode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != CODE_LEN {
            return Err("invalid length".to_string());
        }

        let mut code = [0; CODE_LEN];

        for (i, c) in s.chars().enumerate() {
            if !ALLOWED_CHAR.contains(&c) {
                return Err("invalid character".to_string());
            }

            code[i] = c as u8;
        }

        Ok(GroupConsentCode(code))
    }
}

impl<'de> Deserialize<'de> for GroupConsentCode {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        GroupConsentCode::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl CandidType for GroupConsentCode {
    fn _ty() -> candid::types::Type {
        candid::types::TypeInner::Text.into()
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        self.as_str().idl_serialize(serializer)
    }
}

impl std::fmt::Display for GroupConsentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::fmt::Debug for GroupConsentCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl GroupConsentMap {
    pub fn generate_code(&mut self) -> GroupConsentCode {
        let random = self.rng.raw_random_u64();

        GroupConsentCode::from_u64(random)
    }
}

pub type GroupId = Id;

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Relation {
    Parent,
    Spouse,
    Child,
    Sibling,
    Other,
}

#[derive(Clone, Debug, CandidType, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group {
    pub id: GroupId,
    pub name: AsciiRecordsKey<64>,
    pub leader: NIK,
    pub members: Vec<NIK>,
    pub member_relations: Vec<(NIK, Relation)>,
}

pub type GroupBindingMapResult<T = ()> = Result<T, PatientRegistryError>;

impl_mem_bound!(for Group: bounded; fixed_size: false);
impl_range_bound!(Group);
impl_max_size!(for Group: 1024); // this is pretty expensive. might be worth looking into a more efficient way to store this. - milo
pub struct GroupMap(ic_stable_structures::BTreeMap<Stable<GroupId>, Stable<Group, Candid>, Memory>);
metrics!(GroupMap: Groups);
impl GroupMap {
    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    // create group works in a way that it creates a new entry of Group in the BtreeMap and is linked and bound to a GroupId
    pub fn create_group(
        &mut self,
        name: AsciiRecordsKey<64>,
        leader: NIK,
    ) -> GroupBindingMapResult<GroupId> {
        const UUID_MAX_SOURCE_LEN: usize = 10;
        let group_id = Id::new(&[0u8; UUID_MAX_SOURCE_LEN]);

        println!("randomized group id: {:?}", group_id);
        println!("len of group map: {:?}", self.0.len());

        let group = Group {
            id: group_id.clone(),
            name,
            leader: leader.clone(),
            members: vec![leader.clone()],
            member_relations: vec![(leader, Relation::Parent)],
        };

        if self.0.contains_key(&group_id.clone().to_stable()) {
            return Err(PatientRegistryError::GroupAlreadyExists);
        }

        self.0
            .insert(group_id.clone().to_stable(), group.to_stable());

        println!("group id from create_group registry: {:?}", group_id);

        Ok(group_id)
    }

    pub fn add_member(
        &mut self,
        group_id: GroupId,
        member: NIK,
        relation: Relation,
    ) -> GroupBindingMapResult {
        let key = group_id.clone().to_stable();

        // find the group with that group id first
        let group = self
            .0
            .get(&key)
            .ok_or(PatientRegistryError::GroupNotFound)?;

        // then access the inner group
        let mut group = group.into_inner();

        if group.members.len() >= MAX_GROUP_MEMBERS {
            return Err(PatientRegistryError::GroupFull(MAX_GROUP_MEMBERS));
        }

        if group.members.contains(&member) {
            return Err(PatientRegistryError::UserAlreadyInGroup);
        }

        group.members.push(member.clone());
        group.member_relations.push((member, relation));
        self.0.insert(key, group.to_stable());

        Ok(())
    }

    pub fn remove_member(&mut self, group_id: GroupId, member: &NIK) -> PatientBindingMapResult {
        let key = group_id.to_stable();
        let mut group = self
            .0
            .get(&key)
            .ok_or(PatientRegistryError::UserDoesNotExist)?
            .into_inner();

        // Check if member exists in group
        if !group.members.contains(member) {
            return Err(PatientRegistryError::UserDoesNotExist);
        }

        // If the member is the leader and the only member, dissolve the group
        if group.leader == *member && group.members.len() == 1 {
            self.0.remove(&key);
            return Ok(());
        }

        // If the leaving member is the leader and there are other members,
        // transfer leadership to another random member
        if group.leader == *member && group.members.len() > 1 {
            // Get a random member that isn't the leader
            let new_leader = group
                .members
                .iter()
                .find(|&m| m != member)
                .cloned()
                .expect("There should be at least one other member");
            group.leader = new_leader;
        }

        group.members.retain(|nik| nik != member);
        group.member_relations.retain(|(nik, _)| nik != member);

        // Update the group
        self.0.insert(key, group.to_stable());

        Ok(())
    }

    pub fn get_group(&self, group_id: GroupId) -> Option<Group> {
        println!("group id from get_group registry: {:?}", group_id);
        let group = self.0.get(&group_id.to_stable()).map(|g| g.into_inner());
        // caller must be in the group
        println!("group data found: {:?}", group);
        group
    }

    pub fn get_user_groups(&self, nik: &NIK) -> Vec<Group> {
        self.0
            .iter()
            .map(|(_, group)| group.into_inner())
            .filter(|group| group.members.contains(nik))
            .collect()
    }

    pub fn is_group_leader(&self, group_id: GroupId, nik: &NIK) -> bool {
        self.get_group(group_id)
            .map(|group| group.leader == *nik)
            .unwrap_or(false)
    }

    pub fn get_member_relation(&self, group_id: GroupId, member: &NIK) -> Option<Relation> {
        self.get_group(group_id).and_then(|group| {
            group
                .member_relations
                .iter()
                .find(|(nik, _)| nik == member)
                .map(|(_, relation)| relation.clone())
        })
    }

    pub fn transfer_leadership(
        &mut self,
        group_id: GroupId,
        new_leader: &NIK,
    ) -> Result<(), String> {
        let key = group_id.to_stable();
        let mut group = self.0.get(&key).ok_or("Group not found")?.into_inner();

        if !group.members.contains(new_leader) {
            return Err("New leader must be a group member".to_string());
        }

        group.leader = new_leader.clone();
        self.0.insert(key, group.to_stable());
        Ok(())
    }

    pub fn dissolve_group(&mut self, group_id: GroupId) -> Result<(), String> {
        let key = group_id.to_stable();
        // first verify the group exists
        if !self.0.contains_key(&key) {
            return Err("Group not found".to_string());
        }
        // remove the group entirely
        self.0.remove(&key);
        Ok(())
    }
}

impl Metrics<Groups> for GroupMap {
    fn metrics_name() -> &'static str {
        "groups"
    }

    fn metrics_measurements() -> &'static str {
        "len"
    }

    fn update_measurements(&self) {
        // no-op
    }

    fn get_measurements(&self) -> String {
        self.0.len().to_string()
    }
}

#[cfg(test)]
mod test_group_map {
    use super::*;
    use canister_common::memory_manager;

    #[test]
    fn test_create_group() {
        let memory_manager = memory_manager!();
        let mut group_map = GroupMap::init(&memory_manager);

        let name = AsciiRecordsKey::<64>::new("test_group".to_string()).unwrap();
        let leader = NIK::from([0u8; 32]);

        let result = group_map.create_group(name.clone(), leader.clone());

        let group_id = result.unwrap();

        let group = group_map.get_group(group_id.clone()).unwrap();
        assert_eq!(group.id, group_id);
        assert_eq!(group.name, name);
        assert_eq!(group.leader, leader);
        assert_eq!(group.members, vec![leader]);
    }

    #[test]
    fn test_add_member() {
        let memory_manager = memory_manager!();
        let mut group_map = GroupMap::init(&memory_manager);

        let name = AsciiRecordsKey::<64>::new("test_group".to_string()).unwrap();
        let leader = NIK::from([0u8; 32]);
        let member = NIK::from([1u8; 32]);

        let result = group_map.create_group(name, leader.clone());
        let group_id = result.unwrap();

        // test adding member
        assert!(group_map
            .add_member(group_id.clone(), member.clone(), Relation::Parent)
            .is_ok());

        let group = group_map.get_group(group_id.clone()).unwrap();
        assert!(group.members.contains(&member));

        // test adding same member again (should be idempotent)
        // ! this is erroring and we need to debug why
        match group_map.add_member(group_id.clone(), member.clone(), Relation::Parent) {
            Ok(_) => panic!("Expected error when adding same member again"),
            Err(e) => assert_eq!(e, PatientRegistryError::UserAlreadyInGroup),
        }

        assert_eq!(group_map.get_group(group_id).unwrap().members.len(), 2);
    }

    #[test]
    fn test_get_user_groups() {
        let memory_manager = memory_manager!();
        let mut group_map = GroupMap::init(&memory_manager);

        let leader = NIK::from([0u8; 32]);
        let member = NIK::from([1u8; 32]);
        let non_member = NIK::from([2u8; 32]);

        // create two groups
        let group1_result = group_map.create_group(
            AsciiRecordsKey::<64>::new("group1".to_string()).unwrap(),
            leader.clone(),
        );
        let group1_id = group1_result.unwrap();

        // wait for 1 second
        std::thread::sleep(std::time::Duration::from_secs(1));

        let group2_result = group_map.create_group(
            AsciiRecordsKey::<64>::new("group2".to_string()).unwrap(),
            leader.clone(),
        );
        let group2_id = group2_result.unwrap();

        // verify that group map now has two groups
        assert_eq!(group_map.0.len(), 2);

        // add member to first group only
        group_map
            .add_member(group1_id.clone(), member.clone(), Relation::Parent)
            .unwrap();

        // test group retrieval
        // leader should be in two grous
        let leader_groups = group_map.get_user_groups(&leader);
        assert_eq!(leader_groups.len(), 2);

        let member_groups = group_map.get_user_groups(&member);
        assert_eq!(member_groups.len(), 1);
        assert_eq!(member_groups[0].id, group1_id.clone());

        let non_member_groups = group_map.get_user_groups(&non_member);
        assert_eq!(non_member_groups.len(), 0);

        // verify group IDs instead of comparing whole groups
        let leader_group_ids: Vec<GroupId> = leader_groups.iter().map(|g| g.id.clone()).collect();
        assert!(leader_group_ids.contains(&group1_id));
        assert!(leader_group_ids.contains(&group2_id));
    }

    #[test]
    fn test_is_group_leader() {
        let memory_manager = memory_manager!();
        let mut group_map = GroupMap::init(&memory_manager);

        let leader = NIK::from([0u8; 32]);
        let non_leader = NIK::from([1u8; 32]);

        let group_id_result = group_map.create_group(
            AsciiRecordsKey::<64>::new("test_group".to_string()).unwrap(),
            leader.clone(),
        );
        let group_id = group_id_result.unwrap();

        assert!(group_map.is_group_leader(group_id.clone(), &leader));
        assert!(!group_map.is_group_leader(group_id.clone(), &non_leader));
    }

    #[test]
    fn test_leave_group() {
        let memory_manager = memory_manager!();
        let mut group_map = GroupMap::init(&memory_manager);

        let name = AsciiRecordsKey::<64>::new("test_group".to_string()).unwrap();
        let leader = NIK::from([0u8; 32]);
        let member = NIK::from([1u8; 32]);

        let group_id = group_map.create_group(name, leader.clone());
        let group_id = group_id.unwrap();

        // add member to group
        assert!(group_map
            .add_member(group_id.clone(), member.clone(), Relation::Parent)
            .is_ok());
        assert_eq!(
            group_map.get_group(group_id.clone()).unwrap().members.len(),
            2
        );

        // test member leaving group
        assert!(group_map.remove_member(group_id.clone(), &member).is_ok());
        assert_eq!(
            group_map.get_group(group_id.clone()).unwrap().members.len(),
            1
        );
        assert!(!group_map
            .get_group(group_id.clone())
            .unwrap()
            .members
            .contains(&member));

        // test leader leaving group (should dissolve the group)
        assert!(group_map.remove_member(group_id.clone(), &leader).is_ok());
        assert!(group_map.get_group(group_id.clone()).is_none()); // group should be dissolved
    }
}
