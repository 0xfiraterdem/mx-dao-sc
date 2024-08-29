multiversx_sc::imports!();
multiversx_sc::derive_imports!();

#[multiversx_sc::module]
pub trait OwnerModule: crate::storage::StorageModule {

    #[only_owner]
    #[endpoint(addDaoRights)]
    fn add_dao_rights(&self, addresses: MultiValueEncoded<(ManagedAddress, u64)>) {
        for (address, rights) in addresses.into_iter() {
            let current_rights = self.dao_rights().get(&address).unwrap_or_default();
            self.dao_rights().insert(address.clone(), current_rights + rights);
        }
    }

    #[only_owner]
    #[endpoint(addOption)]
    fn add_option(&self, option: ManagedBuffer) {
        self.options().insert(option);
    }

    #[only_owner]
    #[endpoint(setStartTime)]
    fn set_start_time(&self, timestamp: u64) {
        self.start_time().set(timestamp);
    }

    #[only_owner]
    #[endpoint(setEndTime)]
    fn set_end_time(&self, timestamp: u64) {
        self.end_time().set(timestamp);
    }

    #[only_owner]
    #[endpoint(setStartState)]
    fn set_start_state(&self, started: bool) {
        self.is_started().set(started);
    }

    #[only_owner]
    #[endpoint(endVoting)]  
    fn end_voting(&self) {
        self.is_finished().set(true);
    }


    #[only_owner]
    #[endpoint(setPauseState)]
    fn set_pause_state(&self, paused: bool) {
        self.is_paused().set(paused);
    }
}
