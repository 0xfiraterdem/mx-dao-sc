multiversx_sc::imports!();

#[multiversx_sc::module]
pub trait StorageModule {

    #[view(getDaoRights)]
    #[storage_mapper("dao_rights")]
    fn dao_rights(&self) -> MapMapper<ManagedAddress, u64>;

    #[view(getOptions)]
    #[storage_mapper("options")]
    fn options(&self) -> UnorderedSetMapper<ManagedBuffer>;

    #[view(getVotes)]
    #[storage_mapper("votes")]
    fn votes(&self, option: &ManagedBuffer) -> SingleValueMapper<u64>;

    #[view(getIsStarted)]
    #[storage_mapper("is_started")]
    fn is_started(&self) -> SingleValueMapper<bool>;

    #[view(getIsPaused)]
    #[storage_mapper("is_paused")]
    fn is_paused(&self) -> SingleValueMapper<bool>;

    #[view(getIsFinished)]
    #[storage_mapper("is_finished")]
    fn is_finished(&self) -> SingleValueMapper<bool>;

    #[view(getStartTime)]
    #[storage_mapper("start_time")]
    fn start_time(&self) -> SingleValueMapper<u64>;

    #[view(getEndTime)]
    #[storage_mapper("end_time")]
    fn end_time(&self) -> SingleValueMapper<u64>;

}


