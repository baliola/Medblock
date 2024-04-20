use candid::{ CandidType, Principal };
use canister_common::{
    common::{ AsciiRecordsKey, EmrHeader, UserId, H256 },
    impl_max_size,
    impl_mem_bound,
    impl_range_bound,
    metrics,
    mmgr::MemoryManager,
    opaque_metrics,
    random::CallError,
    stable::{ Candid, Memory, Stable, StableSet, ToStable },
    statistics::traits::{ Metrics, OpaqueMetrics },
};
use ic_stable_structures::memory_manager;
use serde::Deserialize;

use crate::{ api::ReadEmrByIdRequest };

pub struct PatientRegistry {
    pub owner_map: OwnerMap,
    pub emr_binding_map: EmrBindingMap,
    pub info_map: InfoMap,
}

impl PatientRegistry {
    pub fn update_patient_info(
        &mut self,
        patient_principal: Principal,
        patient: Patient
    ) -> PatientBindingMapResult {
        let nik = self.owner_map.get_nik(&patient_principal)?;
        self.info_map.set(nik.into_inner(), patient)
    }

    pub fn get_patient_info_with_principal(
        &self,
        patient_principal: Principal
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
        [opaque_metrics!(self.emr_binding_map), opaque_metrics!(self.owner_map)].join("\n")
    }
}

impl PatientRegistry {
    pub fn construct_args_read_emr(
        &self,
        arg: ReadEmrByIdRequest,
        user_principal: &ic_principal::Principal
    ) -> PatientBindingMapResult<crate::declarations::emr_registry::ReadEmrByIdRequest> {
        let user_id = self.owner_map.get_nik(user_principal)?.into_inner().to_string();

        Ok(crate::declarations::emr_registry::ReadEmrByIdRequest {
            provider_id: arg.provider_id.to_string(),
            emr_id: arg.emr_id.to_string(),
            user_id,
        })
    }

    pub async fn do_call_read_emr(
        arg: crate::declarations::emr_registry::ReadEmrByIdRequest,
        registry: crate::declarations::emr_registry::EmrRegistry
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
            emr_binding_map: EmrBindingMap::init(memory_manager),
            info_map: InfoMap::init(memory_manager),
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

#[derive(Debug, thiserror::Error, CandidType, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatientRegistryError {
    #[error("user exists")]
    UserExist,
    #[error("user does not exist")]
    UserDoesNotExist,
    #[error("emr exists")]
    EmrExists,
}

pub type PatientBindingMapResult<T = ()> = Result<T, PatientRegistryError>;

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
        self.0.get(owner).ok_or(PatientRegistryError::UserDoesNotExist)
    }

    pub fn init(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory::<_, Self>(ic_stable_structures::BTreeMap::init))
    }

    pub fn is_valid_owner(&self, owner: &Owner) -> bool {
        self.0.contains_key(owner)
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

        assert_eq!(owner_map.revoke(&owner).unwrap_err(), PatientRegistryError::UserDoesNotExist);
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(owner_map.revoke(&owner).unwrap(), ());
    }

    #[test]
    fn test_get_nik() {
        let mut owner_map = OwnerMap::init(&MemoryManager::init());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(owner_map.get_nik(&owner).unwrap_err(), PatientRegistryError::UserDoesNotExist);
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
        limit: u8
    ) -> PatientBindingMapResult<Vec<Stable<EmrHeader>>> {
        self.0
            .get_set_associated_by_key_paged(&nik.clone().to_stable(), page as u64, limit as u64)
            .ok_or(PatientRegistryError::UserDoesNotExist)
    }

    pub fn issue_for(&mut self, nik: NIK, header: EmrHeader) -> PatientBindingMapResult<()> {
        if self.is_owner_of(nik.clone(), header.clone()) {
            return Err(PatientRegistryError::EmrExists);
        }

        self.0.insert(nik.to_stable(), header.to_stable());
        Ok(())
    }
}

#[cfg(test)]
mod test_emr_binding_map {
    use candid::{ Principal };
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
        assert_eq!(emr_binding_map.emr_list(&nik, 0, 3).unwrap(), vec![header.to_stable()]);
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

#[derive(Clone, Debug, PartialEq, Eq, CandidType, Deserialize, Default, PartialOrd, Ord)]
pub struct V1 {
    pub name: AsciiRecordsKey<64>,
    pub place_of_birth: AsciiRecordsKey,
    pub date_of_birth: AsciiRecordsKey,
    pub address: AsciiRecordsKey<64>,
    pub martial_status: AsciiRecordsKey<10>,
    pub gender: AsciiRecordsKey<10>,
}

// 270 to account for serialization overhead for using candid. max size is roughly ~190 bytes.
// benchmarked by tsting the encoded size of a struct with max size fields.
impl_max_size!(for V1: 200);
impl_mem_bound!(for V1: bounded; fixed_size: false);
impl_range_bound!(V1);

#[cfg(test)]
mod v1_test {
    use super::*;

    // ~270 bytes
    #[test]
    fn test_len_encoded() {
        use candid::Encode;
        use candid::Decode;

        let patient = V1 {
            name: AsciiRecordsKey::<64>::new("a".repeat(64)).unwrap(),
            place_of_birth: AsciiRecordsKey::<32>::new("a".repeat(32)).unwrap(),
            date_of_birth: AsciiRecordsKey::<32>::new("a".repeat(32)).unwrap(),
            address: AsciiRecordsKey::<64>::new("a".repeat(64)).unwrap(),
            martial_status: AsciiRecordsKey::<10>::new("a".repeat(10)).unwrap(),
            gender: AsciiRecordsKey::<10>::new("a".repeat(10)).unwrap(),
        };

        let encoded = Encode!(&patient).unwrap();
        println!("encoded: {:?}", encoded.len());
        let decoded = Decode!(&encoded, V1).unwrap();

        assert_eq!(patient, decoded);
    }
}
