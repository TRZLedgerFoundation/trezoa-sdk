//! Collection of all runtime features.
//!
//! Steps to add a new feature are outlined below. Note that these steps only cover
//! the process of getting a feature into the core Trezoa code.
//! - For features that are unambiguously good (ie bug fixes), these steps are sufficient.
//! - For features that should go up for community vote (ie fee structure changes), more
//!   information on the additional steps to follow can be found at:
//!   <https://tpl.trezoa.xyz/feature-proposal#feature-proposal-life-cycle>
//!
//! 1. Generate a new keypair with `trezoa-keygen new --outfile feature.json --no-passphrase`
//!    - Keypairs should be held by core contributors only. If you're a non-core contributor going
//!      through these steps, the PR process will facilitate a keypair holder being picked. That
//!      person will generate the keypair, provide pubkey for PR, and ultimately enable the feature.
//! 2. Add a public module for the feature, specifying keypair pubkey as the id with
//!    `trezoa_sdk::declare_id!()` within the module.
//!    Additionally, add an entry to `FEATURE_NAMES` map.
//! 3. Add desired logic to check for and switch on feature availability.
//!
//! For more information on how features are picked up, see comments for `Feature`.

use {
    lazy_static::lazy_static,
    trezoa_program::{epoch_schedule::EpochSchedule, stake_history::Epoch},
    trezoa_sdk::{
        clock::Slot,
        hash::{Hash, Hasher},
        pubkey::Pubkey,
    },
    std::collections::{HashMap, HashSet},
};

pub mod deprecate_rewards_sysvar {
    trezoa_sdk::declare_id!("HsaeUom4UuxCDdacurd1L6hiqSmb6xMTHUs4oRBeohPa");
}

pub mod pico_inflation {
    trezoa_sdk::declare_id!("PicoNatVzZA6xhpGNuAQNFvDN5WrTNGnY1QKKWMjz8R");
}

pub mod full_inflation {
    pub mod devnet_and_testnet {
        trezoa_sdk::declare_id!("DevTestDGrMz2ZUr3A7BfUfYKiQoNixfhQQjy9t45kL");
    }

    pub mod mainnet {
        pub mod certusone {
            pub mod vote {
                trezoa_sdk::declare_id!("Aya35EN6wXVV8mMVDuGdJvfJornfTYmYZruYn78uzz6");
            }
            pub mod enable {
                trezoa_sdk::declare_id!("56ocpWEY9F1WuPySM2AGBViGdYkRND6FFzaPoqSgAzvp");
            }
        }
    }
}

pub mod secp256k1_program_enabled {
    trezoa_sdk::declare_id!("8hbcjERBvydRFFULZtjFi2awzhBYo1stdh6w68CAMid3");
}

pub mod tpl_token_v2_multisig_fix {
    trezoa_sdk::declare_id!("7Du9vEjTYXA7BR3rNouG62YcCXwaahAEWgfnetjkY86X");
}

pub mod no_overflow_rent_distribution {
    trezoa_sdk::declare_id!("2GEjYctbyvX6VCqUNvQ8bENpTdXiXvRna16ZYsj5EZfK");
}

pub mod filter_stake_delegation_accounts {
    trezoa_sdk::declare_id!("F3cJ2uPXXNTGzpY6xMhYC92NsjVYqYPAtEJ978KK1p69");
}

pub mod require_custodian_for_locked_stake_authorize {
    trezoa_sdk::declare_id!("HNSp9tZx8Ctkq2bjfmc8bkQJHbFJYmLTR6Y5en8k45MJ");
}

pub mod tpl_token_v2_self_transfer_fix {
    trezoa_sdk::declare_id!("2KeGaSe23Cj6v3zVW3mhjLs4bL8CMgHKEoABv4f474ZM");
}

pub mod warp_timestamp_again {
    trezoa_sdk::declare_id!("4qxWL62Wyx4CaiSJzNNxdaEwFRiKN2JieosYwT1iCxAy");
}

pub mod check_init_vote_data {
    trezoa_sdk::declare_id!("9tQP9gtmjVwGiUh56NJdoySgywUucmZAF84phkEWVWDv");
}

pub mod secp256k1_recover_syscall_enabled {
    trezoa_sdk::declare_id!("7jZyvu4pnsiFLvHKoeUDE3naxYC4fCorPineBdK48x8a");
}

pub mod system_transfer_zero_check {
    trezoa_sdk::declare_id!("5ubeMqcn7nsUQGTGW7cj9TxmJ8ReJNNjgBBAk4d9Aezs");
}

pub mod blake3_syscall_enabled {
    trezoa_sdk::declare_id!("4YyaXeTpko6DnYomSMPRRC2mp9AtuCcR1QZZvYo93dV6");
}

pub mod dedupe_config_program_signers {
    trezoa_sdk::declare_id!("8Pb9s6wuHykE3rYBJsMj3L9WV2DF9MRBiTfHGSkznQxN");
}

pub mod verify_tx_signatures_len {
    trezoa_sdk::declare_id!("DLdzJr9EeYTbc2WzX4qgD9JTwDgPMgz1Ei9VGSsjXhkb");
}

pub mod vote_stake_checked_instructions {
    trezoa_sdk::declare_id!("J4CM2zubT4uvtFqdazL8PoLCkxkVWqNoGh6u34X3ocgB");
}

pub mod rent_for_sysvars {
    trezoa_sdk::declare_id!("6i7V9zbRs7hfURf5NejRDQsRov4b62e8SpZqauwtk5ZT");
}

pub mod libsecp256k1_0_5_upgrade_enabled {
    trezoa_sdk::declare_id!("DSTJV6cpaC2XE6A8qUdziWDtUDUxMCisdjr7gXtKPRry");
}

pub mod tx_wide_compute_cap {
    trezoa_sdk::declare_id!("3LhK6aRroZFNHjmLz61QAvRBadQ8hXqiT6kui1cjDCu6");
}

pub mod tpl_token_v2_set_authority_fix {
    trezoa_sdk::declare_id!("EEtsq3Pobc3i281LKTGz2sNZ6rNwhJCZxK96SufwD1cF");
}

pub mod merge_nonce_error_into_system_error {
    trezoa_sdk::declare_id!("9zeZYy16bdXqoXTqTTuHWA5ARUeM9EcAc8FxgqcoRXR1");
}

pub mod disable_fees_sysvar {
    trezoa_sdk::declare_id!("A5rzsE8eKfLGWQxuRGLg4ZXVBtMCACfZnPxtjy7yzEEd");
}

pub mod stake_merge_with_unmatched_credits_observed {
    trezoa_sdk::declare_id!("9zUq9RF2UfLuuiDrzCT1mGUs4DXu8SkFBS1rHdRoUTps");
}

pub mod zk_token_sdk_enabled {
    trezoa_sdk::declare_id!("7zWtUAEMB1EZdNCrNzGeyvprSyqtYDCAs7bPf2ygkKrR");
}

pub mod curve25519_syscall_enabled {
    trezoa_sdk::declare_id!("5kTQPZeBx2PUQKgD8TisHKW7bvnFpoxK2ZK6niYSjeYa");
}

pub mod curve25519_restrict_msm_length {
    trezoa_sdk::declare_id!("7LG8UVfGVUrniepL6Fr423g71b1kHJFvoo6b9gUJuLD2");
}

pub mod versioned_tx_message_enabled {
    trezoa_sdk::declare_id!("3fMJcV5pLAmj8JMgCWFW7N2XsnZxQ7PvAdudc23FZXtt");
}

pub mod libsecp256k1_fail_on_bad_count {
    trezoa_sdk::declare_id!("5p4qMsNUuPB9AecHGH3LTiU9VzwrAWgUdn9eCLatJZ6Q");
}

pub mod libsecp256k1_fail_on_bad_count2 {
    trezoa_sdk::declare_id!("4JHzHNmG6KgjVSjgV6oWPiYAajhJTruSjk1SMLt1xvG5");
}

pub mod instructions_sysvar_owned_by_sysvar {
    trezoa_sdk::declare_id!("EBRvnfcfxL3HXHP4c217KYwWcUFgHndZpQdiVjci5xw8");
}

pub mod stake_program_advance_activating_credits_observed {
    trezoa_sdk::declare_id!("DKHFucSKPgCRQ9Z8Di5QVCNSfAN28zT1EeGjbyrWpRx4");
}

pub mod credits_auto_rewind {
    trezoa_sdk::declare_id!("48TEF2Pu7zDb5i5WSkz752nNG1srpMpMV815B3HhTv8i");
}

pub mod demote_program_write_locks {
    trezoa_sdk::declare_id!("6MncZd376ooRoLv1YMzPbKM2wkiFzFEgw36zKwvoqYFA");
}

pub mod ed25519_program_enabled {
    trezoa_sdk::declare_id!("AqUbYgEQnT6MyAvFL4hNXNYNBKz4Z9NP1Vmm56p2AAja");
}

pub mod return_data_syscall_enabled {
    trezoa_sdk::declare_id!("8o1JTm9LAh5oBVPWDLQ42jtxVSsFR2d4LYQwnGFSw3WA");
}

pub mod reduce_required_deploy_balance {
    trezoa_sdk::declare_id!("AYfzK1qKdARBhsd8Cs7dk2c58tKKAMvi7d6QoNipa8xq");
}

pub mod trz_log_data_syscall_enabled {
    trezoa_sdk::declare_id!("BtgDcsE3vz1SejiGXtQvsi1rkdjnpSJAsZKovrEevxoa");
}

pub mod stakes_remove_delegation_if_inactive {
    trezoa_sdk::declare_id!("5PcPMJcARdWU7EcmZoDYCiwEZh6xgs6KRiK5AQateyop");
}

pub mod do_support_realloc {
    trezoa_sdk::declare_id!("Ca9PBaYZSRk9HxgcLRrp3wUKLHZDiATpVWYkKZHFvne");
}

pub mod prevent_calling_precompiles_as_programs {
    trezoa_sdk::declare_id!("9JSyJeiLSuThuo9v3wHYS7LEL59kuJCiuiRP7CqwZR7W");
}

pub mod optimize_epoch_boundary_updates {
    trezoa_sdk::declare_id!("8CM3oV5ozV8RYfWEwCynRTL6YcrCrk626e5bpUTsS44S");
}

pub mod remove_native_loader {
    trezoa_sdk::declare_id!("CmN1oTbsnDVeHczVYAQ8MpexsrVjhQ1CLavxuJTC5Lh4");
}

pub mod send_to_tpu_vote_port {
    trezoa_sdk::declare_id!("6pYs4sDvzzyWj86HTN4H3ZPkd1q7ZzFvyV1xgahAWUVz");
}

pub mod requestable_heap_size {
    trezoa_sdk::declare_id!("6rWoq15vBVzQ6rMAp9wa3VcTQT4cEd8VVRVVosWpav6M");
}

pub mod disable_fee_calculator {
    trezoa_sdk::declare_id!("EXQgvTbZUSdadm2xhSrqYrUS5JFmt5RfThQknJsnAanJ");
}

pub mod add_compute_budget_program {
    trezoa_sdk::declare_id!("ArmGMFB9jFVyEEzbZWhFYbadGYX5vS7X1WXDT7uAJfkC");
}

pub mod nonce_must_be_writable {
    trezoa_sdk::declare_id!("8XSRyNXtuRNUv72iZoLrofwArZNmSXhPrJJxDrDGGp7A");
}

pub mod tpl_token_v3_3_0_release {
    trezoa_sdk::declare_id!("EwrPAzuVxC9sVbQdvsue2NU2QpSqMNLTY78UfYUjGkPz");
}

pub mod leave_nonce_on_success {
    trezoa_sdk::declare_id!("4H9b1XNHKcZ13TPk8yNwy4oWjxM6tv4TgUt3bL7hA8XW");
}

pub mod reject_empty_instruction_without_program {
    trezoa_sdk::declare_id!("Ap3mFypnGuddcYYcnNWSpyajDD5E6qXk4FRYxyY2Q4X9");
}

pub mod fixed_memcpy_nonoverlapping_check {
    trezoa_sdk::declare_id!("7Wtd1B396yruYbBTPtAfczoh3Br6nLRV1oP4yaJqk8TL");
}

pub mod reject_non_rent_exempt_vote_withdraws {
    trezoa_sdk::declare_id!("2mDG3UAejEtTTo9pxj7qvDFPhDWc8nRrgvETeYBzFBdq");
}

pub mod evict_invalid_stakes_cache_entries {
    trezoa_sdk::declare_id!("EvictVkHvGx5QMLtS5jxwKZdUcEfKCMGmYgDoyzGKPuu");
}

pub mod allow_votes_to_directly_update_vote_state {
    trezoa_sdk::declare_id!("22tjpg6yJq5gLdVWpygQdBejywjotPaBcRkheU1ML671");
}

pub mod cap_accounts_data_len {
    trezoa_sdk::declare_id!("BCvUvDzfzJBW912yL1nj8DEUnqSFChYQjkg3Hnvo4PzC");
}

pub mod max_tx_account_locks {
    trezoa_sdk::declare_id!("DYJvGr1bvL2Jdqyk6V31SBjfmNAjWFRYYWS4bGzxZJDj");
}

pub mod require_rent_exempt_accounts {
    trezoa_sdk::declare_id!("J9jZacLLwehS6S2csgQChYffpeB8dBHkuJZ5hGDZ4McH");
}

pub mod filter_votes_outside_slot_hashes {
    trezoa_sdk::declare_id!("DJuAz7bfUXvMtgVkiANKRj3zgiUqfagAuZ2EkorMmmcV");
}

pub mod update_syscall_base_costs {
    trezoa_sdk::declare_id!("DMQcBjsmtG1xkJzjuFzkYUvfFWfNLa73uHo1avh19xZK");
}

pub mod stake_deactivate_delinquent_instruction {
    trezoa_sdk::declare_id!("2Gx7hn8r52kiLMWFwVFzHVjpimxqkV4Ke7ouDLGtEWjM");
}

pub mod stake_redelegate_instruction {
    trezoa_sdk::declare_id!("H8rrXexC3wWfZoaGPmmfQoSEJBejuRrdmaj6cv8LSXmS");
}

pub mod vote_withdraw_authority_may_change_authorized_voter {
    trezoa_sdk::declare_id!("9r4eEAVo9md6k2SXwtthQiHwXLNf1UDFP4H1eUjLHAAM");
}

pub mod tpl_associated_token_account_v1_0_4 {
    trezoa_sdk::declare_id!("D5NoYKvb2MX3d8sgxopQ8ejaXDjMcu8YAG1A4d1zmTvv");
}

pub mod reject_vote_account_close_unless_zero_credit_epoch {
    trezoa_sdk::declare_id!("9TCC9HpyxseeME6npvy1iqQdi9r1aoZt4uLGhZtCDQw9");
}

pub mod add_get_processed_sibling_instruction_syscall {
    trezoa_sdk::declare_id!("GkS3yEPmATs2Vutax4cMPtuNn4aCYeiSiT8BrvHsWbaA");
}

pub mod bank_transaction_count_fix {
    trezoa_sdk::declare_id!("5yMPoqB7U5W7JNouoMDjnCQhnmSQwUv2Xnbq2hyuoc5j");
}

pub mod disable_bpf_deprecated_load_instructions {
    trezoa_sdk::declare_id!("J4PzKPtdA3Lm84d2Fk7Me4rHEaepB1GPfyhytExiCQCE");
}

pub mod disable_bpf_unresolved_symbols_at_runtime {
    trezoa_sdk::declare_id!("3TtGnHfJNnJLiQPAuUuz5WTo244mX8mHcT61PUjRJpA3");
}

pub mod record_instruction_in_transaction_context_push {
    trezoa_sdk::declare_id!("DZwum3yzteEy5fGctohR74RVVZfmG5vvK3GxEnwEYaq6");
}

pub mod syscall_saturated_math {
    trezoa_sdk::declare_id!("3wFJEP7MtDyckHuGWRBDva6zPVABYuNgTJVCa8adDQNM");
}

pub mod check_physical_overlapping {
    trezoa_sdk::declare_id!("BSGXzf3bGCJAVk5nzboPTtPv24vJUVCHTtub883yBup2");
}

pub mod limit_secp256k1_recovery_id {
    trezoa_sdk::declare_id!("7pRzXjzMtUTem31UBhGnqByD5tXH7p8jPVJzZsJiacky");
}

pub mod disable_deprecated_loader {
    trezoa_sdk::declare_id!("nuwSAnsRuj6RdhrGcBis18Y3F423WaPFFPprxBStcES");
}

pub mod check_slice_translation_size {
    trezoa_sdk::declare_id!("HFpvdRJaLx5JjPAvoN5Se1GsMHNbdNrLp6DbG28VZ8GQ");
}

pub mod stake_split_uses_rent_sysvar {
    trezoa_sdk::declare_id!("HmauMBWRj3ZE1hQx2VWc3PioEwuUbwDrBwuEyKhFpx3Y");
}

pub mod add_get_minimum_delegation_instruction_to_stake_program {
    trezoa_sdk::declare_id!("EHaaGTV7yinZRCxYQyMsDv3XrceTcWrdW6DseGqTr1Kc");
}

pub mod error_on_syscall_bpf_function_hash_collisions {
    trezoa_sdk::declare_id!("B3Z2gEzVayzvLE4x2r1jjf8Ncm4mHNbqk8Hns9JWG37d");
}

pub mod reject_callx_r10 {
    trezoa_sdk::declare_id!("7Gdpmm3yqB9gFmnzu9Qq9kFXK4csSe6DDZX4chbevxmv");
}

pub mod drop_redundant_turbine_path {
    trezoa_sdk::declare_id!("HLkCWM2vbhoF6jMC8XpKYXeun4umNZrgRqTDMjhWzn38");
}

pub mod executables_incur_cpi_data_cost {
    trezoa_sdk::declare_id!("5VCkxtisJdugjjqdibqEEpYkD5TBfNytXsy3KKmTFg59");
}

pub mod fix_recent_blockhashes {
    trezoa_sdk::declare_id!("7g8sMg4o1iz2tMa4Em8cGbAV44MvfAKv7iKFPpFGum1w");
}

pub mod update_rewards_from_cached_accounts {
    trezoa_sdk::declare_id!("CaAgpNSDTQBXjGKoTAu43fxfP2vRr1sbNCAMm75N12ia");
}
pub mod enable_partitioned_epoch_reward {
    trezoa_sdk::declare_id!("2xWZdc2y7VPnCvq4FUZGF5mJ43JHVCsWhSxVrsJUhi99");
}

pub mod tpl_token_v3_4_0 {
    trezoa_sdk::declare_id!("4sYbW7qEG4Wrf2rTNjkGZE9vQ41XdPFQjMDf9Z6Yg7yG");
}

pub mod tpl_associated_token_account_v1_1_0 {
    trezoa_sdk::declare_id!("3eAcQKAhAhP3AoPoHWTu7FKfGSN7cU8GTBpUK5WeZL1u");
}

pub mod default_units_per_instruction {
    trezoa_sdk::declare_id!("B3HsMY9ntVJ2Yf9aTosVsdS1yi2axeJfWiCDApH8YXq1");
}

pub mod stake_allow_zero_undelegated_amount {
    trezoa_sdk::declare_id!("C36ZK7TUcNXZV2x8e9jA6B4pGFWKhNXg1AxPQXpwRcNb");
}

pub mod require_static_program_ids_in_transaction {
    trezoa_sdk::declare_id!("8jUaFoHWcv8QLh54cHfXW2NwjP5xUZjLnE12wUq27ngc");
}

pub mod stake_raise_minimum_delegation_to_1_trz {
    // This is a feature-proposal *feature id*.  The feature keypair address is `GQXzC7YiSNkje6FFUk6sc2p53XRvKoaZ9VMktYzUMnpL`.
    trezoa_sdk::declare_id!("8JVQSEukeV3yqZY7n99W3Qbb5ZmdUWoveNwVSsrX4JK9");
}

pub mod stake_minimum_delegation_for_rewards {
    trezoa_sdk::declare_id!("2oRVv923a2A2wAjmcEYaV4SpBwteFS69gcq8BfaTU1Ws");
}

pub mod add_set_compute_unit_price_ix {
    trezoa_sdk::declare_id!("5wnfMrgrfFTQsnRodikCgzN1LkFRNCs2gGjh4RB1VvWf");
}

pub mod disable_deploy_of_alloc_free_syscall {
    trezoa_sdk::declare_id!("4HTdNasKjuQjbSUW8TXY8W9o8m7vazwKJ7KEF5ZknezX");
}

pub mod include_account_index_in_rent_error {
    trezoa_sdk::declare_id!("GVc9dKVJFqsjQ5HLJJLsjepLVKUCFSMtiM7reztcV2y4");
}

pub mod add_shred_type_to_shred_seed {
    trezoa_sdk::declare_id!("6zma35a6gcVJXTuQCApsd5fNCke3KpsJdaGNpRzJezh5");
}

pub mod warp_timestamp_with_a_vengeance {
    trezoa_sdk::declare_id!("BGkcG8czHpusnRYyAM2qduG7meLew6qpnvErhxBsZ1hf");
}

pub mod separate_nonce_from_blockhash {
    trezoa_sdk::declare_id!("ChwbCVVojvywP5Srhpyua7GSoMmCNr6tNDX4V5mvcLpM");
}

pub mod enable_durable_nonce {
    trezoa_sdk::declare_id!("8nJGbppDnrB4RgNzBNx8Xrqs9NYh3bvdNWUsNxunCfZc");
}

pub mod vote_state_update_credit_per_dequeue {
    trezoa_sdk::declare_id!("2FzevWJGXnYkDQeuUZexGTtD371padyVq7T22Z4uDFSC");
}

pub mod quick_bail_on_panic {
    trezoa_sdk::declare_id!("6UnztR8p5x63YfpRM1GRJKnMcdmd3a2C5ePTx9HWkZMy");
}

pub mod nonce_must_be_authorized {
    trezoa_sdk::declare_id!("BGg91FW3GffXbZSKMjgoVqBBC1GNtHUknr3MMUfvbUSJ");
}

pub mod nonce_must_be_advanceable {
    trezoa_sdk::declare_id!("6frac6H96umBWfjY46QVWAqrdDjx3z9kCqHxx8jyQz9G");
}

pub mod vote_authorize_with_seed {
    trezoa_sdk::declare_id!("2mYZGBFBT8wA4t4ykPnNstoPx14sNvBB65NABHXEJGdw");
}

pub mod cap_accounts_data_size_per_block {
    trezoa_sdk::declare_id!("3gzaSHfwUUsRiiLPQNfjzkUQKBszpBQ44RAzjHRbZ1WZ");
}

pub mod preserve_rent_epoch_for_rent_exempt_accounts {
    trezoa_sdk::declare_id!("CmfcaqnMRrSgT1CdHdWXcCZ9FymRdtBnvLXbxFpqBXTm");
}

pub mod enable_bpf_loader_extend_program_ix {
    trezoa_sdk::declare_id!("ACX2xV8sQspAfmBKmT8wmdnttZsk2RhLoqShgt8uP9wc");
}

pub mod enable_early_verification_of_account_modifications {
    trezoa_sdk::declare_id!("EdKycumc9jox8FNnmExkG9Upkhyt4VS7dcuSRXwdUXyw");
}

pub mod skip_rent_rewrites {
    trezoa_sdk::declare_id!("ATDmWTyHM7tC5jtq8566vY8antvHryyX9attKHgPNEnS");
}

pub mod prevent_crediting_accounts_that_end_rent_paying {
    trezoa_sdk::declare_id!("5djFKBYXUqhmcSDYip4x1oKzZb9ZkHuU1t3UNiyP8MuF");
}

pub mod cap_bpf_program_instruction_accounts {
    trezoa_sdk::declare_id!("4QgLxE6Xa9EndJdEfNHFJz9pbLHKbr7Td9kVFSEnePH4");
}

pub mod loosen_cpi_size_restriction {
    trezoa_sdk::declare_id!("8XKEp5V127TL8DELXCPm4n7pVoT7CKAd5gbaeibY4jzu");
}

pub mod use_default_units_in_fee_calculation {
    trezoa_sdk::declare_id!("EnuZhNiuTcKNkGkbJYtjzEgJkJdRwgrdbznRqLGRdvgi");
}

pub mod compact_vote_state_updates {
    trezoa_sdk::declare_id!("AJqHtMKAnqdw7tTkSMZ33jaV3gx19ytQrnQoBTmU727Z");
}

pub mod incremental_snapshot_only_incremental_hash_calculation {
    trezoa_sdk::declare_id!("EZNmrkijE9dY4Lx5ApcaJBS7Xubpgm9C3pz7CnzSky2P");
}

pub mod disable_cpi_setting_executable_and_rent_epoch {
    trezoa_sdk::declare_id!("Cejs4bWQyk8Dqog6My8fQs2hi9pJj8AKif4TksaBVuF");
}

pub mod on_load_preserve_rent_epoch_for_rent_exempt_accounts {
    trezoa_sdk::declare_id!("43P22Z1b8DzaHknGtYWMPHiQWoVd6zLQJpiBF3PiKnfF");
}

pub mod account_hash_ignore_slot {
    trezoa_sdk::declare_id!("PTE3puhmFRUJ1j21KwdjEgwzqaTPqz57fgkustsB6Zq");
}

pub mod set_exempt_rent_epoch_max {
    trezoa_sdk::declare_id!("HWe7nBajEcML4CAY6zP4bJoiZXaPQwhDfShr6jybX1Gv");
}

pub mod relax_authority_signer_check_for_lookup_table_creation {
    trezoa_sdk::declare_id!("DAdvmKSYPJt1YfVBvPW9mU1jyneGnzVK2QphaHtebRk2");
}

pub mod stop_sibling_instruction_search_at_parent {
    trezoa_sdk::declare_id!("28fAFYhvm2V8Wk99W897ysMHBMLCErMjVmSskM5TbAx4");
}

pub mod vote_state_update_root_fix {
    trezoa_sdk::declare_id!("8B3S8NRkgczG1Tgcxfc1ZTDowJxG4C2NfVdVmYuJm7if");
}

pub mod cap_accounts_data_allocations_per_transaction {
    trezoa_sdk::declare_id!("9sFpMpUV1TjakdXdgvdEqtG2M6PaiREkhQ5hwhfZRRmB");
}

pub mod epoch_accounts_hash {
    trezoa_sdk::declare_id!("99pR8UrnVaG7YpfiXwT3QNtsmMRrbWtAFCBjw2aLix22");
}

pub mod remove_deprecated_request_unit_ix {
    trezoa_sdk::declare_id!("9W4LeYW8QxS6YiMG3PKUSm3FQko5UaCzYKjiCdYDNa5j");
}

pub mod disable_rehash_for_rent_epoch {
    trezoa_sdk::declare_id!("GYPEqrNsnx7Xf9JYnVQmTof4eZ1Dp3RRtwbZhaAw3SXh");
}

pub mod increase_tx_account_lock_limit {
    trezoa_sdk::declare_id!("9rvEpbW71Wakv1NbX53ejRdkR8P15SWwUrqYQNgNcr8q");
}

pub mod limit_max_instruction_trace_length {
    trezoa_sdk::declare_id!("3nLWNrSoFS86nBnpPwQ86FM7ABUBr1WA71YeyzyukM5M");
}

pub mod check_syscall_outputs_do_not_overlap {
    trezoa_sdk::declare_id!("JCPe5ewNeTDPSFTwRKH1c4z4nGjtK59WLpLiJPrwDms5");
}

pub mod enable_bpf_loader_set_authority_checked_ix {
    trezoa_sdk::declare_id!("dsgxhh2dH6fuHk3qpznSK9ix8TLzBCxXWgqEcJJwvbL");
}

pub mod enable_alt_bn128_syscall {
    trezoa_sdk::declare_id!("HdXdrpLfF864SNV6KXerassipLEahBTWhiTPoCJWE9RW");
}

pub mod simplify_alt_bn128_syscall_error_codes {
    trezoa_sdk::declare_id!("7pU36NCueJKchJrcu9jJjGWcrH6jdyjXUpcjxHfkGUJS");
}

pub mod enable_alt_bn128_compression_syscall {
    trezoa_sdk::declare_id!("9HDXowiutoUSvPYJUYkHQTAqwA5kxhY4Yqw9WG7Ak8v8");
}

pub mod enable_program_redeployment_cooldown {
    trezoa_sdk::declare_id!("6o4fwRBywk8HQvdDstSXWM91MsksZYJi4Bxy7Lw1GAHy");
}

pub mod commission_updates_only_allowed_in_first_half_of_epoch {
    trezoa_sdk::declare_id!("9FTVRiDXyK919i2waxMNJHpRLnAN5tdUSKBuHRpmkjq");
}

pub mod enable_turbine_fanout_experiments {
    trezoa_sdk::declare_id!("GN1Uri9oet7cfTL4Vb8ciD5pH9pNsWksCqQg2sgaWgef");
}

pub mod disable_turbine_fanout_experiments {
    trezoa_sdk::declare_id!("CxNKYaW497ribFFgrBCQfzr6TQ2ehdeu4hnNnxhcoKKn");
}

pub mod move_serialized_len_ptr_in_cpi {
    trezoa_sdk::declare_id!("D1rFoPAFZnJ5SirVjvEYGLwPcHbYBxWbqn3z27UnFyxj");
}

pub mod update_hashes_per_tick {
    trezoa_sdk::declare_id!("AnzxWshfkRaPjv6z3YnudkxrHbSthhusHMh5zG5jAxYe");
}

pub mod enable_big_mod_exp_syscall {
    trezoa_sdk::declare_id!("78SyzyjXuNKo6qgH8PuFpGUJMLp76iSJUKG8rndnjPXk");
}

pub mod disable_builtin_loader_ownership_chains {
    trezoa_sdk::declare_id!("UbFxqXMbvqqD89AS2mMEEqo3DQ6JEzxQuEt3K6tQ2TB");
}

pub mod cap_transaction_accounts_data_size {
    trezoa_sdk::declare_id!("8dBfhyB6Ptt4Yaf7ddqdjUmgpjJMgYgt8YcZCURcybXD");
}

pub mod remove_congestion_multiplier_from_fee_calculation {
    trezoa_sdk::declare_id!("NTbCTEBZMr5BcQdGyDjq6eQJUp68k7LnnTA4cdF2BkU");
}

pub mod enable_request_heap_frame_ix {
    trezoa_sdk::declare_id!("2Ssm1e8kFqwG1CNgcp8whpoZYcYVqe8h5YCvphsFdasU");
}

pub mod prevent_rent_paying_rent_recipients {
    trezoa_sdk::declare_id!("7E7v19kF8b9fQV8uHwKgzAoZwqxK5os7fuhPvxsQ8RwJ");
}

pub mod delay_visibility_of_program_deployment {
    trezoa_sdk::declare_id!("HsMGnYr35772xnm1VNotGkxw1jg2UZ3UWWXpzRowQvr7");
}

pub mod apply_cost_tracker_during_replay {
    trezoa_sdk::declare_id!("6eD1pzj9co2KCpLxfBDKCE1n3oR27KVdjf4fMZ6SdTaw");
}
pub mod bpf_account_data_direct_mapping {
    trezoa_sdk::declare_id!("DAcE5eqQZn9cSVND34NGTPXrEqiKK94gnJyDP9VYujr2");
}

pub mod add_set_tx_loaded_accounts_data_size_instruction {
    trezoa_sdk::declare_id!("EoPPK398wodr9hLRg97g5ZGEDJVNGnM8DgWWc2gWjvB1");
}

pub mod switch_to_new_elf_parser {
    trezoa_sdk::declare_id!("F1UBSm7PQ2TkebJ1f627CuZi8JoXYpXUmgbqhAnDZB7L");
}

pub mod round_up_heap_size {
    trezoa_sdk::declare_id!("F5aNz6FqKAuXos8cevasC7RZ5ndPwsTrABxSjZVNAfDH");
}

pub mod remove_bpf_loader_incorrect_program_id {
    trezoa_sdk::declare_id!("9w5wh2pjWEtxvUzmfU99vFMqdakTMxZhas9gQa5jJE1V");
}

pub mod include_loaded_accounts_data_size_in_fee_calculation {
    trezoa_sdk::declare_id!("VB8AAkejW59R9PksLAjqz9kMLkT7iifQq5qci3KxGxd");
}

pub mod native_programs_consume_cu {
    trezoa_sdk::declare_id!("7M3REVxB8TENx2gUBkNGYY8pcvcd86g5gnB6HE3FBxHP");
}

pub mod simplify_writable_program_account_check {
    trezoa_sdk::declare_id!("FLAYui6hbDCUvMLtHaTFBAG99Ev6LQ6W31fsCA9uavjE");
}

pub mod stop_truncating_strings_in_syscalls {
    trezoa_sdk::declare_id!("HCzqUQM24fqcHv3QqS9vC5n4BBGsmf3CFBUdo9M2FjSy");
}

pub mod clean_up_delegation_errors {
    trezoa_sdk::declare_id!("4UUR6jQDdS3pFwuoe7t42o4yjztyELU1zdFx23tTz8Ut");
}

pub mod vote_state_add_vote_latency {
    trezoa_sdk::declare_id!("7Wwhj6E52u5tN5Je85icouZM6CqkCTf3mLTNuPgUnWYW");
}

pub mod checked_arithmetic_in_fee_validation {
    trezoa_sdk::declare_id!("CLt53NEWw9F8s1AVWBaoBs7v9zgrYrLFsC5QX1JUjc2G");
}

pub mod last_restart_slot_sysvar {
    trezoa_sdk::declare_id!("EAoQX1ifcMir24Pnqbhup9EXNKunqgMVkYoZcWgaEWXr");
}

pub mod reduce_stake_warmup_cooldown {
    trezoa_sdk::declare_id!("AqEmXGogQvsRNEJkstFErG71PQq24AtBj5rxDZ9CXm14");
}

pub mod revise_turbine_epoch_stakes {
    trezoa_sdk::declare_id!("DZk6AvW2hB8CSVbQvicA8t2awscGc5sHroNcd6dvEpvi");
}

pub mod enable_poseidon_syscall {
    trezoa_sdk::declare_id!("FSaEN4r4UXy9iLr7NSwjXS14XjU9Nvj78Rq1r3NJ79g5");
}

pub mod timely_vote_credits {
    trezoa_sdk::declare_id!("7iyL5goBXeJWTguHZB1cvQdmGhDWu2jjaNhNtcCYaeh6");
}

pub mod remaining_compute_units_syscall_enabled {
    trezoa_sdk::declare_id!("EX7A3ufLgAzxmtJPpZ1kw5YB62XtM7JLywsTuFzGYjd7");
}

pub mod enable_program_runtime_v2_and_loader_v4 {
    trezoa_sdk::declare_id!("9Vp7SevqxgX5xskZp8aBJP8YHdqwN3ybALtV3D2ZDKPJ");
}

pub mod require_rent_exempt_split_destination {
    trezoa_sdk::declare_id!("29dyJWaH7a4Nm1obMUtLcuhiW7W6YRxzWMB2NgQ5kBbZ");
}

pub mod better_error_codes_for_tx_lamport_check {
    trezoa_sdk::declare_id!("25sM6bKY7WH8kKvFQxGbBExRGCTs465qFGyLHPpGnwFh");
}

pub mod update_hashes_per_tick2 {
    trezoa_sdk::declare_id!("AYL1Po5iV7JmYz46UM9VgBgMdyFVyJjq3LzrCav7R56n");
}

pub mod update_hashes_per_tick3 {
    trezoa_sdk::declare_id!("5HGa8jZR4LsvPvEVYBtEu9B5Ea2KSMSpDHXZe9r6VvWo");
}

pub mod update_hashes_per_tick4 {
    trezoa_sdk::declare_id!("BMmgcFubsJYMcGMVYBsVCPbPzo2V8ymsue45AbSDGjUW");
}

pub mod update_hashes_per_tick5 {
    trezoa_sdk::declare_id!("76MCq3iEShhrLdprWELUwJDbREXQb1PyYqLA25DSwkB");
}

pub mod update_hashes_per_tick6 {
    trezoa_sdk::declare_id!("2Jg3KigvwFoSZYhncDLzsPdTarm2TG9BLS4owSHLdHfz");
}

pub mod validate_fee_collector_account {
    trezoa_sdk::declare_id!("CAx4QskERvT6B5WRgpPDgcpTf6WSyESMAqkCVt6EUBxY");
}

pub mod disable_rent_fees_collection {
    trezoa_sdk::declare_id!("Br2mtyqxpHKs5SxVQYCTTCn1iaZnhWx9fpHY8nNWPciN");
}

pub mod enable_zk_transfer_with_fee {
    trezoa_sdk::declare_id!("33iiHJqyJHW7LrDGyfwat9uSfu2xxVB3UzTTPKEUyyVJ");
}

pub mod drop_legacy_shreds {
    trezoa_sdk::declare_id!("EE4TJcHzakdc2L5X9PYtVwWza3JxKX4dC1aFQ3L83Ept");
}

pub mod allow_commission_decrease_at_any_time {
    trezoa_sdk::declare_id!("2gY99PcthFzDNHRQQdRvXMsVddkvPEuUFh6uAJnoGDNs");
}

pub mod consume_blockstore_duplicate_proofs {
    trezoa_sdk::declare_id!("UhpKXHMMff4VHXMRUeiGmUicHHBVWAydM92mWVyt45R");
}

pub mod index_erasure_conflict_duplicate_proofs {
    trezoa_sdk::declare_id!("kqLFdaitoPGkYMFPtM9o6yxzeDPzA6SGxhymeXynDKJ");
}

pub mod merkle_conflict_duplicate_proofs {
    trezoa_sdk::declare_id!("FK2oHrkf2Ws9dfmMNZH83W3Qw3WKyLa4MBYwggntizpE");
}

pub mod disable_bpf_loader_instructions {
    trezoa_sdk::declare_id!("CP3PZ1VdXuTY8gq7JajdND3qMQnuhgU3pXoDsa1KXud3");
}

pub mod enable_zk_proof_from_account {
    trezoa_sdk::declare_id!("HYXUfCGadrGTeme7pse8W7KSK9YF73RGGE8Wt6aZ2eSE");
}

pub mod cost_model_requested_write_lock_cost {
    trezoa_sdk::declare_id!("4PCmCTDfeuqCsFLqyze9gvqARrkEatZRy5KwXP68aw5p");
}

pub mod enable_gossip_duplicate_proof_ingestion {
    trezoa_sdk::declare_id!("Gf754yv8M1y4Eq48LZr2Hexz6qYznvBsmjGXprML4BFV");
}

pub mod chained_merkle_conflict_duplicate_proofs {
    trezoa_sdk::declare_id!("6R73gPo8uxyh1wAKjbkWxhZqK3ct2cndoPzd21566gJc");
}

pub mod enable_chained_merkle_shreds {
    trezoa_sdk::declare_id!("6riZVruBz1BSUCXySpdTZYzZB8Tc7gdkGSiyVQA1isHr");
}

pub mod deprecate_unused_legacy_vote_plumbing {
    trezoa_sdk::declare_id!("E8GTCwtwjVNEpnBd7qX85R6BDJ22gmgEtWbYv6oFfWz3");
}

pub mod enable_turbine_extended_fanout_experiments {
    trezoa_sdk::declare_id!("AZoQ7yFVZCNgEWWPN9dkDV29SdUaUr241k31ppBxCYHE");
}

lazy_static! {
    /// Map of feature identifiers to user-visible description
    pub static ref FEATURE_NAMES: HashMap<Pubkey, &'static str> = [
        (secp256k1_program_enabled::id(), "secp256k1 program"),
        (deprecate_rewards_sysvar::id(), "deprecate unused rewards sysvar"),
        (pico_inflation::id(), "pico inflation"),
        (full_inflation::devnet_and_testnet::id(), "full inflation on devnet and testnet"),
        (tpl_token_v2_multisig_fix::id(), "tpl-token multisig fix"),
        (no_overflow_rent_distribution::id(), "no overflow rent distribution"),
        (filter_stake_delegation_accounts::id(), "filter stake_delegation_accounts #14062"),
        (require_custodian_for_locked_stake_authorize::id(), "require custodian to authorize withdrawer change for locked stake"),
        (tpl_token_v2_self_transfer_fix::id(), "tpl-token self-transfer fix"),
        (full_inflation::mainnet::certusone::enable::id(), "full inflation enabled by Certus One"),
        (full_inflation::mainnet::certusone::vote::id(), "community vote allowing Certus One to enable full inflation"),
        (warp_timestamp_again::id(), "warp timestamp again, adjust bounding to 25% fast 80% slow #15204"),
        (check_init_vote_data::id(), "check initialized Vote data"),
        (secp256k1_recover_syscall_enabled::id(), "secp256k1_recover syscall"),
        (system_transfer_zero_check::id(), "perform all checks for transfers of 0 lamports"),
        (blake3_syscall_enabled::id(), "blake3 syscall"),
        (dedupe_config_program_signers::id(), "dedupe config program signers"),
        (verify_tx_signatures_len::id(), "prohibit extra transaction signatures"),
        (vote_stake_checked_instructions::id(), "vote/state program checked instructions #18345"),
        (rent_for_sysvars::id(), "collect rent from accounts owned by sysvars"),
        (libsecp256k1_0_5_upgrade_enabled::id(), "upgrade libsecp256k1 to v0.5.0"),
        (tx_wide_compute_cap::id(), "transaction wide compute cap"),
        (tpl_token_v2_set_authority_fix::id(), "tpl-token set_authority fix"),
        (merge_nonce_error_into_system_error::id(), "merge NonceError into SystemError"),
        (disable_fees_sysvar::id(), "disable fees sysvar"),
        (stake_merge_with_unmatched_credits_observed::id(), "allow merging active stakes with unmatched credits_observed #18985"),
        (zk_token_sdk_enabled::id(), "enable Zk Token proof program and syscalls"),
        (curve25519_syscall_enabled::id(), "enable curve25519 syscalls"),
        (versioned_tx_message_enabled::id(), "enable versioned transaction message processing"),
        (libsecp256k1_fail_on_bad_count::id(), "fail libsecp256k1_verify if count appears wrong"),
        (libsecp256k1_fail_on_bad_count2::id(), "fail libsecp256k1_verify if count appears wrong"),
        (instructions_sysvar_owned_by_sysvar::id(), "fix owner for instructions sysvar"),
        (stake_program_advance_activating_credits_observed::id(), "Enable advancing credits observed for activation epoch #19309"),
        (credits_auto_rewind::id(), "Auto rewind stake's credits_observed if (accidental) vote recreation is detected #22546"),
        (demote_program_write_locks::id(), "demote program write locks to readonly, except when upgradeable loader present #19593 #20265"),
        (ed25519_program_enabled::id(), "enable builtin ed25519 signature verify program"),
        (return_data_syscall_enabled::id(), "enable trz_{set,get}_return_data syscall"),
        (reduce_required_deploy_balance::id(), "reduce required payer balance for program deploys"),
        (trz_log_data_syscall_enabled::id(), "enable trz_log_data syscall"),
        (stakes_remove_delegation_if_inactive::id(), "remove delegations from stakes cache when inactive"),
        (do_support_realloc::id(), "support account data reallocation"),
        (prevent_calling_precompiles_as_programs::id(), "prevent calling precompiles as programs"),
        (optimize_epoch_boundary_updates::id(), "optimize epoch boundary updates"),
        (remove_native_loader::id(), "remove support for the native loader"),
        (send_to_tpu_vote_port::id(), "send votes to the tpu vote port"),
        (requestable_heap_size::id(), "Requestable heap frame size"),
        (disable_fee_calculator::id(), "deprecate fee calculator"),
        (add_compute_budget_program::id(), "Add compute_budget_program"),
        (nonce_must_be_writable::id(), "nonce must be writable"),
        (tpl_token_v3_3_0_release::id(), "tpl-token v3.3.0 release"),
        (leave_nonce_on_success::id(), "leave nonce as is on success"),
        (reject_empty_instruction_without_program::id(), "fail instructions which have native_loader as program_id directly"),
        (fixed_memcpy_nonoverlapping_check::id(), "use correct check for nonoverlapping regions in memcpy syscall"),
        (reject_non_rent_exempt_vote_withdraws::id(), "fail vote withdraw instructions which leave the account non-rent-exempt"),
        (evict_invalid_stakes_cache_entries::id(), "evict invalid stakes cache entries on epoch boundaries"),
        (allow_votes_to_directly_update_vote_state::id(), "enable direct vote state update"),
        (cap_accounts_data_len::id(), "cap the accounts data len"),
        (max_tx_account_locks::id(), "enforce max number of locked accounts per transaction"),
        (require_rent_exempt_accounts::id(), "require all new transaction accounts with data to be rent-exempt"),
        (filter_votes_outside_slot_hashes::id(), "filter vote slots older than the slot hashes history"),
        (update_syscall_base_costs::id(), "update syscall base costs"),
        (stake_deactivate_delinquent_instruction::id(), "enable the deactivate delinquent stake instruction #23932"),
        (vote_withdraw_authority_may_change_authorized_voter::id(), "vote account withdraw authority may change the authorized voter #22521"),
        (tpl_associated_token_account_v1_0_4::id(), "TPL Associated Token Account Program release version 1.0.4, tied to token 3.3.0 #22648"),
        (reject_vote_account_close_unless_zero_credit_epoch::id(), "fail vote account withdraw to 0 unless account earned 0 credits in last completed epoch"),
        (add_get_processed_sibling_instruction_syscall::id(), "add add_get_processed_sibling_instruction_syscall"),
        (bank_transaction_count_fix::id(), "fixes Bank::transaction_count to include all committed transactions, not just successful ones"),
        (disable_bpf_deprecated_load_instructions::id(), "disable ldabs* and ldind* SBF instructions"),
        (disable_bpf_unresolved_symbols_at_runtime::id(), "disable reporting of unresolved SBF symbols at runtime"),
        (record_instruction_in_transaction_context_push::id(), "move the CPI stack overflow check to the end of push"),
        (syscall_saturated_math::id(), "syscalls use saturated math"),
        (check_physical_overlapping::id(), "check physical overlapping regions"),
        (limit_secp256k1_recovery_id::id(), "limit secp256k1 recovery id"),
        (disable_deprecated_loader::id(), "disable the deprecated BPF loader"),
        (check_slice_translation_size::id(), "check size when translating slices"),
        (stake_split_uses_rent_sysvar::id(), "stake split instruction uses rent sysvar"),
        (add_get_minimum_delegation_instruction_to_stake_program::id(), "add GetMinimumDelegation instruction to stake program"),
        (error_on_syscall_bpf_function_hash_collisions::id(), "error on bpf function hash collisions"),
        (reject_callx_r10::id(), "Reject bpf callx r10 instructions"),
        (drop_redundant_turbine_path::id(), "drop redundant turbine path"),
        (executables_incur_cpi_data_cost::id(), "Executables incur CPI data costs"),
        (fix_recent_blockhashes::id(), "stop adding hashes for skipped slots to recent blockhashes"),
        (update_rewards_from_cached_accounts::id(), "update rewards from cached accounts"),
        (enable_partitioned_epoch_reward::id(), "enable partitioned rewards at epoch boundary #32166"),
        (tpl_token_v3_4_0::id(), "TPL Token Program version 3.4.0 release #24740"),
        (tpl_associated_token_account_v1_1_0::id(), "TPL Associated Token Account Program version 1.1.0 release #24741"),
        (default_units_per_instruction::id(), "Default max tx-wide compute units calculated per instruction"),
        (stake_allow_zero_undelegated_amount::id(), "Allow zero-lamport undelegated amount for initialized stakes #24670"),
        (require_static_program_ids_in_transaction::id(), "require static program ids in versioned transactions"),
        (stake_raise_minimum_delegation_to_1_trz::id(), "Raise minimum stake delegation to 1.0 TRZ #24357"),
        (stake_minimum_delegation_for_rewards::id(), "stakes must be at least the minimum delegation to earn rewards"),
        (add_set_compute_unit_price_ix::id(), "add compute budget ix for setting a compute unit price"),
        (disable_deploy_of_alloc_free_syscall::id(), "disable new deployments of deprecated trz_alloc_free_ syscall"),
        (include_account_index_in_rent_error::id(), "include account index in rent tx error #25190"),
        (add_shred_type_to_shred_seed::id(), "add shred-type to shred seed #25556"),
        (warp_timestamp_with_a_vengeance::id(), "warp timestamp again, adjust bounding to 150% slow #25666"),
        (separate_nonce_from_blockhash::id(), "separate durable nonce and blockhash domains #25744"),
        (enable_durable_nonce::id(), "enable durable nonce #25744"),
        (vote_state_update_credit_per_dequeue::id(), "Calculate vote credits for VoteStateUpdate per vote dequeue to match credit awards for Vote instruction"),
        (quick_bail_on_panic::id(), "quick bail on panic"),
        (nonce_must_be_authorized::id(), "nonce must be authorized"),
        (nonce_must_be_advanceable::id(), "durable nonces must be advanceable"),
        (vote_authorize_with_seed::id(), "An instruction you can use to change a vote accounts authority when the current authority is a derived key #25860"),
        (cap_accounts_data_size_per_block::id(), "cap the accounts data size per block #25517"),
        (stake_redelegate_instruction::id(), "enable the redelegate stake instruction #26294"),
        (preserve_rent_epoch_for_rent_exempt_accounts::id(), "preserve rent epoch for rent exempt accounts #26479"),
        (enable_bpf_loader_extend_program_ix::id(), "enable bpf upgradeable loader ExtendProgram instruction #25234"),
        (skip_rent_rewrites::id(), "skip rewriting rent exempt accounts during rent collection #26491"),
        (enable_early_verification_of_account_modifications::id(), "enable early verification of account modifications #25899"),
        (disable_rehash_for_rent_epoch::id(), "on accounts hash calculation, do not try to rehash accounts #28934"),
        (account_hash_ignore_slot::id(), "ignore slot when calculating an account hash #28420"),
        (set_exempt_rent_epoch_max::id(), "set rent epoch to Epoch::MAX for rent-exempt accounts #28683"),
        (on_load_preserve_rent_epoch_for_rent_exempt_accounts::id(), "on bank load account, do not try to fix up rent_epoch #28541"),
        (prevent_crediting_accounts_that_end_rent_paying::id(), "prevent crediting rent paying accounts #26606"),
        (cap_bpf_program_instruction_accounts::id(), "enforce max number of accounts per bpf program instruction #26628"),
        (loosen_cpi_size_restriction::id(), "loosen cpi size restrictions #26641"),
        (use_default_units_in_fee_calculation::id(), "use default units per instruction in fee calculation #26785"),
        (compact_vote_state_updates::id(), "Compact vote state updates to lower block size"),
        (incremental_snapshot_only_incremental_hash_calculation::id(), "only hash accounts in incremental snapshot during incremental snapshot creation #26799"),
        (disable_cpi_setting_executable_and_rent_epoch::id(), "disable setting is_executable and_rent_epoch in CPI #26987"),
        (relax_authority_signer_check_for_lookup_table_creation::id(), "relax authority signer check for lookup table creation #27205"),
        (stop_sibling_instruction_search_at_parent::id(), "stop the search in get_processed_sibling_instruction when the parent instruction is reached #27289"),
        (vote_state_update_root_fix::id(), "fix root in vote state updates #27361"),
        (cap_accounts_data_allocations_per_transaction::id(), "cap accounts data allocations per transaction #27375"),
        (epoch_accounts_hash::id(), "enable epoch accounts hash calculation #27539"),
        (remove_deprecated_request_unit_ix::id(), "remove support for RequestUnitsDeprecated instruction #27500"),
        (increase_tx_account_lock_limit::id(), "increase tx account lock limit to 128 #27241"),
        (limit_max_instruction_trace_length::id(), "limit max instruction trace length #27939"),
        (check_syscall_outputs_do_not_overlap::id(), "check syscall outputs do_not overlap #28600"),
        (enable_bpf_loader_set_authority_checked_ix::id(), "enable bpf upgradeable loader SetAuthorityChecked instruction #28424"),
        (enable_alt_bn128_syscall::id(), "add alt_bn128 syscalls #27961"),
        (simplify_alt_bn128_syscall_error_codes::id(), "simplify alt_bn128 syscall error codes SIMD-0129"),
        (enable_program_redeployment_cooldown::id(), "enable program redeployment cooldown #29135"),
        (commission_updates_only_allowed_in_first_half_of_epoch::id(), "validator commission updates are only allowed in the first half of an epoch #29362"),
        (enable_turbine_fanout_experiments::id(), "enable turbine fanout experiments #29393"),
        (disable_turbine_fanout_experiments::id(), "disable turbine fanout experiments #29393"),
        (move_serialized_len_ptr_in_cpi::id(), "cpi ignore serialized_len_ptr #29592"),
        (update_hashes_per_tick::id(), "Update desired hashes per tick on epoch boundary"),
        (enable_big_mod_exp_syscall::id(), "add big_mod_exp syscall #28503"),
        (disable_builtin_loader_ownership_chains::id(), "disable builtin loader ownership chains #29956"),
        (cap_transaction_accounts_data_size::id(), "cap transaction accounts data size up to a limit #27839"),
        (remove_congestion_multiplier_from_fee_calculation::id(), "Remove congestion multiplier from transaction fee calculation #29881"),
        (enable_request_heap_frame_ix::id(), "Enable transaction to request heap frame using compute budget instruction #30076"),
        (prevent_rent_paying_rent_recipients::id(), "prevent recipients of rent rewards from ending in rent-paying state #30151"),
        (delay_visibility_of_program_deployment::id(), "delay visibility of program upgrades #30085"),
        (apply_cost_tracker_during_replay::id(), "apply cost tracker to blocks during replay #29595"),
        (add_set_tx_loaded_accounts_data_size_instruction::id(), "add compute budget instruction for setting account data size per transaction #30366"),
        (switch_to_new_elf_parser::id(), "switch to new ELF parser #30497"),
        (round_up_heap_size::id(), "round up heap size when calculating heap cost #30679"),
        (remove_bpf_loader_incorrect_program_id::id(), "stop incorrectly throwing IncorrectProgramId in bpf_loader #30747"),
        (include_loaded_accounts_data_size_in_fee_calculation::id(), "include transaction loaded accounts data size in base fee calculation #30657"),
        (native_programs_consume_cu::id(), "Native program should consume compute units #30620"),
        (simplify_writable_program_account_check::id(), "Simplify checks performed for writable upgradeable program accounts #30559"),
        (stop_truncating_strings_in_syscalls::id(), "Stop truncating strings in syscalls #31029"),
        (clean_up_delegation_errors::id(), "Return InsufficientDelegation instead of InsufficientFunds or InsufficientStake where applicable #31206"),
        (vote_state_add_vote_latency::id(), "replace Lockout with LandedVote (including vote latency) in vote state #31264"),
        (checked_arithmetic_in_fee_validation::id(), "checked arithmetic in fee validation #31273"),
        (bpf_account_data_direct_mapping::id(), "use memory regions to map account data into the rbpf vm instead of copying the data"),
        (last_restart_slot_sysvar::id(), "enable new sysvar last_restart_slot"),
        (reduce_stake_warmup_cooldown::id(), "reduce stake warmup cooldown from 25% to 9%"),
        (revise_turbine_epoch_stakes::id(), "revise turbine epoch stakes"),
        (enable_poseidon_syscall::id(), "Enable Poseidon syscall"),
        (timely_vote_credits::id(), "use timeliness of votes in determining credits to award"),
        (remaining_compute_units_syscall_enabled::id(), "enable the remaining_compute_units syscall"),
        (enable_program_runtime_v2_and_loader_v4::id(), "Enable Program-Runtime-v2 and Loader-v4 #33293"),
        (require_rent_exempt_split_destination::id(), "Require stake split destination account to be rent exempt"),
        (better_error_codes_for_tx_lamport_check::id(), "better error codes for tx lamport check #33353"),
        (enable_alt_bn128_compression_syscall::id(), "add alt_bn128 compression syscalls"),
        (update_hashes_per_tick2::id(), "Update desired hashes per tick to 2.8M"),
        (update_hashes_per_tick3::id(), "Update desired hashes per tick to 4.4M"),
        (update_hashes_per_tick4::id(), "Update desired hashes per tick to 7.6M"),
        (update_hashes_per_tick5::id(), "Update desired hashes per tick to 9.2M"),
        (update_hashes_per_tick6::id(), "Update desired hashes per tick to 10M"),
        (validate_fee_collector_account::id(), "validate fee collector account #33888"),
        (disable_rent_fees_collection::id(), "Disable rent fees collection #33945"),
        (enable_zk_transfer_with_fee::id(), "enable Zk Token proof program transfer with fee"),
        (drop_legacy_shreds::id(), "drops legacy shreds #34328"),
        (allow_commission_decrease_at_any_time::id(), "Allow commission decrease at any time in epoch #33843"),
        (consume_blockstore_duplicate_proofs::id(), "consume duplicate proofs from blockstore in consensus #34372"),
        (index_erasure_conflict_duplicate_proofs::id(), "generate duplicate proofs for index and erasure conflicts #34360"),
        (merkle_conflict_duplicate_proofs::id(), "generate duplicate proofs for merkle root conflicts #34270"),
        (disable_bpf_loader_instructions::id(), "disable bpf loader management instructions #34194"),
        (enable_zk_proof_from_account::id(), "Enable zk token proof program to read proof from accounts instead of instruction data #34750"),
        (curve25519_restrict_msm_length::id(), "restrict curve25519 multiscalar multiplication vector lengths #34763"),
        (cost_model_requested_write_lock_cost::id(), "cost model uses number of requested write locks #34819"),
        (enable_gossip_duplicate_proof_ingestion::id(), "enable gossip duplicate proof ingestion #32963"),
        (enable_chained_merkle_shreds::id(), "Enable chained Merkle shreds #34916"),
        (deprecate_unused_legacy_vote_plumbing::id(), "Deprecate unused legacy vote tx plumbing"),
        (chained_merkle_conflict_duplicate_proofs::id(), "generate duplicate proofs for chained merkle root conflicts"),
        (enable_turbine_extended_fanout_experiments::id(), "enable turbine extended fanout experiments #2373"),
        /*************** ADD NEW FEATURES HERE ***************/
    ]
    .iter()
    .cloned()
    .collect();

    /// Unique identifier of the current software's feature set
    pub static ref ID: Hash = {
        let mut hasher = Hasher::default();
        let mut feature_ids = FEATURE_NAMES.keys().collect::<Vec<_>>();
        feature_ids.sort();
        for feature in feature_ids {
            hasher.hash(feature.as_ref());
        }
        hasher.result()
    };
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct FullInflationFeaturePair {
    pub vote_id: Pubkey, // Feature that grants the candidate the ability to enable full inflation
    pub enable_id: Pubkey, // Feature to enable full inflation by the candidate
}

lazy_static! {
    /// Set of feature pairs that once enabled will trigger full inflation
    pub static ref FULL_INFLATION_FEATURE_PAIRS: HashSet<FullInflationFeaturePair> = [
        FullInflationFeaturePair {
            vote_id: full_inflation::mainnet::certusone::vote::id(),
            enable_id: full_inflation::mainnet::certusone::enable::id(),
        },
    ]
    .iter()
    .cloned()
    .collect();
}

/// `FeatureSet` holds the set of currently active/inactive runtime features
#[derive(AbiExample, Debug, Clone, Eq, PartialEq)]
pub struct FeatureSet {
    pub active: HashMap<Pubkey, Slot>,
    pub inactive: HashSet<Pubkey>,
}
impl Default for FeatureSet {
    fn default() -> Self {
        // All features disabled
        Self {
            active: HashMap::new(),
            inactive: FEATURE_NAMES.keys().cloned().collect(),
        }
    }
}
impl FeatureSet {
    pub fn is_active(&self, feature_id: &Pubkey) -> bool {
        self.active.contains_key(feature_id)
    }

    pub fn activated_slot(&self, feature_id: &Pubkey) -> Option<Slot> {
        self.active.get(feature_id).copied()
    }

    /// List of enabled features that trigger full inflation
    pub fn full_inflation_features_enabled(&self) -> HashSet<Pubkey> {
        let mut hash_set = FULL_INFLATION_FEATURE_PAIRS
            .iter()
            .filter_map(|pair| {
                if self.is_active(&pair.vote_id) && self.is_active(&pair.enable_id) {
                    Some(pair.enable_id)
                } else {
                    None
                }
            })
            .collect::<HashSet<_>>();

        if self.is_active(&full_inflation::devnet_and_testnet::id()) {
            hash_set.insert(full_inflation::devnet_and_testnet::id());
        }
        hash_set
    }

    /// All features enabled, useful for testing
    pub fn all_enabled() -> Self {
        Self {
            active: FEATURE_NAMES.keys().cloned().map(|key| (key, 0)).collect(),
            inactive: HashSet::new(),
        }
    }

    /// Activate a feature
    pub fn activate(&mut self, feature_id: &Pubkey, slot: u64) {
        self.inactive.remove(feature_id);
        self.active.insert(*feature_id, slot);
    }

    /// Deactivate a feature
    pub fn deactivate(&mut self, feature_id: &Pubkey) {
        self.active.remove(feature_id);
        self.inactive.insert(*feature_id);
    }

    pub fn new_warmup_cooldown_rate_epoch(&self, epoch_schedule: &EpochSchedule) -> Option<Epoch> {
        self.activated_slot(&reduce_stake_warmup_cooldown::id())
            .map(|slot| epoch_schedule.get_epoch(slot))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_inflation_features_enabled_devnet_and_testnet() {
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::devnet_and_testnet::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::devnet_and_testnet::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_full_inflation_features_enabled() {
        // Normal sequence: vote_id then enable_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );

        // Backwards sequence: enable_id and then vote_id
        let mut feature_set = FeatureSet::default();
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::enable::id(), 42);
        assert!(feature_set.full_inflation_features_enabled().is_empty());
        feature_set
            .active
            .insert(full_inflation::mainnet::certusone::vote::id(), 42);
        assert_eq!(
            feature_set.full_inflation_features_enabled(),
            [full_inflation::mainnet::certusone::enable::id()]
                .iter()
                .cloned()
                .collect()
        );
    }

    #[test]
    fn test_feature_set_activate_deactivate() {
        let mut feature_set = FeatureSet::default();

        let feature = Pubkey::new_unique();
        assert!(!feature_set.is_active(&feature));
        feature_set.activate(&feature, 0);
        assert!(feature_set.is_active(&feature));
        feature_set.deactivate(&feature);
        assert!(!feature_set.is_active(&feature));
    }
}
