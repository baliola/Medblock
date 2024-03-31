use candid::{ CandidType };
use canister_common::{
    common::{ EmrHeader, UserId, H256 },
    metrics,
    mmgr::MemoryManager,
    opaque_metrics,
    random::CallError,
    stable::{ Memory, Stable, StableSet, ToStable },
    statistics::traits::{ Metrics, OpaqueMetrics },
};

use crate::{ api::ReadEmrByIdRequest, declarations::emr_registry::emr_registry };

pub struct PatientRegistry {
    pub owner_map: OwnerMap,
    pub emr_binding_map: EmrBindingMap,
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
        arg: crate::declarations::emr_registry::ReadEmrByIdRequest
    ) -> crate::declarations::emr_registry::ReadEmrByIdResponse {
        match emr_registry.read_emr_by_id(arg).await.map_err(CallError::from) {
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

        Ok(self.0.insert(nik.to_stable(), header.to_stable()))
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
