// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           21
// Async Callback (empty):               1
// Total number of exported functions:  24

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    cyber_dao
    (
        init => init
        upgrade => upgrade
        vote => vote
        getOptionPercentage => get_option_percentage
        getAllVotes => get_all_votes
        getUsedVotesAndUnusedVotes => get_used_votes_rights_and_unused_votes
        getUserVoteRights => get_user_vote_rights
        getRemainingTime => get_remaining_time
        addDaoRights => add_dao_rights
        addOption => add_option
        setStartTime => set_start_time
        setEndTime => set_end_time
        setStartState => set_start_state
        endVoting => end_voting
        setPauseState => set_pause_state
        getDaoRights => dao_rights
        getOptions => options
        getVotes => votes
        getIsStarted => is_started
        getIsPaused => is_paused
        getIsFinished => is_finished
        getStartTime => start_time
        getEndTime => end_time
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
