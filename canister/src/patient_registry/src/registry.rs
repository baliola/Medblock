use candid::{ CandidType };
use canister_common::{common::{EmrId, Id, UserId, H256}, mmgr::MemoryManager, stable::{Memory, Stable, StableSet, ToStable}};





pub type NIK = H256;

/// Principal to NIK Map. meant to enforce 1:1 relationship between principal and NIK.
/// used to claim emrs ownership. This level of inderction is needed because principal that map to a particular BindingKey effectively owns
/// all the emrs that it's BindingKey map to.
pub type Owner = ic_principal::Principal;
pub struct OwnerMap(ic_stable_structures::BTreeMap<Owner, Stable<NIK>, Memory>);

#[derive(Debug, thiserror::Error, CandidType, serde::Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PatientBindingMapError {
    #[error("operation not permitted, user exists")]
    UserExist,
    #[error("operation not permitted, user does not exist")]
    UserDoesNotExist,
}

pub type PatientBindingMapResult<T = ()> = Result<T, PatientBindingMapError>;

impl OwnerMap {
    pub fn revoke(&mut self, owner: &Owner) -> PatientBindingMapResult {
        self.0
            .remove(owner)
            .map(|_| ())
            .ok_or(PatientBindingMapError::UserDoesNotExist)
    }

    pub fn bind(&mut self, owner: Owner, nik: NIK) -> PatientBindingMapResult {
        if self.get_nik(&owner).is_ok() {
            return Err(PatientBindingMapError::UserExist);
        }

        let _ = self.0.insert(owner, nik.to_stable());
        Ok(())
    }

    pub fn rebind(&mut self, owner: Owner, nik: NIK) -> PatientBindingMapResult {
        if self.get_nik(&owner).is_err() {
            return Err(PatientBindingMapError::UserDoesNotExist);
        }

        let _ = self.0.insert(owner, nik.to_stable());
        Ok(())
    }

    /// will return an error if owner does not exists
    pub fn get_nik(&self, owner: &Owner) -> PatientBindingMapResult<Stable<UserId>> {
        self.0.get(owner).ok_or(PatientBindingMapError::UserDoesNotExist)
    }

    pub fn new(memory_manager: &MemoryManager) -> Self {
        Self(memory_manager.get_memory(ic_stable_structures::BTreeMap::new))
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
        let mut owner_map = OwnerMap::new(&MemoryManager::new());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(
            owner_map.bind(owner, nik.clone()).unwrap_err(),
            PatientBindingMapError::UserExist
        );
    }

    #[test]
    fn test_rebind() {
        let mut owner_map = OwnerMap::new(&MemoryManager::new());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            owner_map.rebind(owner, nik.clone()).unwrap_err(),
            PatientBindingMapError::UserDoesNotExist
        );
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(owner_map.rebind(owner, nik.clone()).unwrap(), ());
    }

    #[test]
    fn test_revoke() {
        let mut owner_map = OwnerMap::new(&MemoryManager::new());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(owner_map.revoke(&owner).unwrap_err(), PatientBindingMapError::UserDoesNotExist);
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(owner_map.revoke(&owner).unwrap(), ());
    }

    #[test]
    fn test_get_nik() {
        let mut owner_map = OwnerMap::new(&MemoryManager::new());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert_eq!(
            owner_map.get_nik(&owner).unwrap_err(),
            PatientBindingMapError::UserDoesNotExist
        );
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert_eq!(owner_map.get_nik(&owner).unwrap(), nik.to_stable());
    }

    #[test]
    fn test_is_valid_owner() {
        let mut owner_map = OwnerMap::new(&MemoryManager::new());
        let owner = ic_principal::Principal::anonymous();
        let nik = NIK::from([0u8; 32]);

        assert!(!owner_map.is_valid_owner(&owner));
        assert_eq!(owner_map.bind(owner, nik.clone()).unwrap(), ());
        assert!(owner_map.is_valid_owner(&owner));
    }
}

/// track emr issued for a particular user by storing it's emr id in this map. also used as blind index for emr search.
/// we use hashed (SHA3-256) NIK as key and emr id as value.
///
/// we don't use the principal directly because we want users to be able to change it's internet identity
/// and still be able to own and access their emr.
///
/// NIK MUST be hashed offchain before being used as key.
pub struct EmrBindingMap(StableSet<Stable<NIK>, Stable<Id>>);

impl EmrBindingMap {
    pub fn new(memory_manager: &MemoryManager) -> Self {
        Self(StableSet::new(memory_manager))
    }

    pub fn is_owner_of(&self, nik: NIK, emr_id: EmrId) -> bool {
        self.0.contains_key(nik.to_stable(), emr_id.to_stable())
    }

    pub fn emr_list(&self, nik: &NIK) -> PatientBindingMapResult<Vec<Stable<EmrId>>> {
        self.0
            .get_set_associated_by_key(&nik.clone().to_stable())
            .ok_or(PatientBindingMapError::UserDoesNotExist)
    }

    pub fn issue_for(&mut self, nik: NIK, emr_id: EmrId) {
        self.0.insert(nik.to_stable(), emr_id.to_stable());
    }
}

#[cfg(test)]
mod test_emr_binding_map {

    use super::*;

    #[test]
    fn test_issue_for() {
        let mut emr_binding_map = EmrBindingMap::new(&MemoryManager::new());
        let nik = NIK::from([0u8; 32]);

        let mut random = [0u8; 10];
        random.fill(0);
        let emr_id = EmrId::new(&random);

        emr_binding_map.issue_for(nik.clone(), emr_id.clone());
        assert!(emr_binding_map.is_owner_of(nik.clone(), emr_id.clone()));
    }

    #[test]
    fn test_emr_list() {
        let mut emr_binding_map = EmrBindingMap::new(&MemoryManager::new());
        let nik = NIK::from([0u8; 32]);
        let mut random = [0u8; 10];
        random.fill(0);
        let emr_id = EmrId::new(&random);

        emr_binding_map.issue_for(nik.clone(), emr_id.clone());
        assert_eq!(emr_binding_map.emr_list(&nik).unwrap(), vec![emr_id.to_stable()]);
    }
}
