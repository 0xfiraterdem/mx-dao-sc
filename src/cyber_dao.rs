#![no_std]

#[allow(unused_imports)]
use multiversx_sc::imports::*;

mod storage;
mod owner;

#[multiversx_sc::contract]
pub trait CyberDao:
    owner::OwnerModule
    + storage::StorageModule {

    #[init]
    fn init(&self) {
        self.is_started().set(false);
        self.is_finished().set(false);
        self.is_paused().set(false);
    }

    #[upgrade]
    fn upgrade(&self) {
        self.is_started().set(self.is_started().get());
        self.is_finished().set(self.is_finished().get());
        self.is_paused().set(self.is_paused().get());
    }

    #[endpoint(vote)]
    fn vote(&self, option: ManagedBuffer) {


        let current_time = self.blockchain().get_block_timestamp();
        let end_time = self.end_time().get();
        let start_time = self.start_time().get();

        require!(current_time < end_time && !self.is_finished().get(), "Voting period has ended");
        require!(current_time >= start_time || self.is_started().get(), "Voting period has not started yet");
        require!(!self.is_paused().get(), "Voting is paused");

        let caller = self.blockchain().get_caller();
        let rights = self.dao_rights().get(&caller).unwrap_or(0);

        require!(rights > 0, "You have no voting rights");


        let current_votes = self.votes(&option).get();
        self.votes(&option).set(current_votes + rights);
        
        self.dao_rights().insert(caller.clone(), rights - rights);
    }

    #[view(getOptionPercentage)]
    fn get_option_percentage(&self, option: ManagedBuffer) -> BigUint {
        let total_votes: u64 = self.options().iter().map(|opt| self.votes(&opt).get()).sum();
        let option_votes = self.votes(&option).get();

        if total_votes > 0 {
            BigUint::from(option_votes) * 100u64 / BigUint::from(total_votes)
        } else {
            BigUint::zero()
        }
    }

    #[view(getAllVotes)]
    fn get_all_votes(&self) -> MultiValueEncoded<(ManagedBuffer, u64)> {
        let mut result = MultiValueEncoded::new();

        for option in self.options().iter() {
            let votes = self.votes(&option).get();
            result.push((option, votes));
        }

        result
    }

    #[view(getUsedVotesAndUnusedVotes)]
    fn get_used_votes_rights_and_unused_votes(&self) -> (BigUint, BigUint) {
        let mut unused_votes = BigUint::zero();
        let mut used_votes = BigUint::zero();

        for (_, rights) in self.dao_rights().iter() {
            unused_votes += BigUint::from(rights);
        }

        for option in self.options().iter() {
            let votes = self.votes(&option).get();
            used_votes += BigUint::from(votes);
        }

        (unused_votes, used_votes)
    }

    #[view(getUserVoteRights)]
    fn get_user_vote_rights(&self, user: ManagedAddress) -> u64 {
        self.dao_rights().get(&user).unwrap_or(0)
    }

    #[view(getRemainingTime)]
    fn get_remaining_time(&self) -> u64 {
        let current_time = self.blockchain().get_block_timestamp();
        let start_time = self.start_time().get();
    
        let remaining_time = if start_time > current_time {
            start_time - current_time
        } else {
            0
        };
    
        remaining_time
    }

}
