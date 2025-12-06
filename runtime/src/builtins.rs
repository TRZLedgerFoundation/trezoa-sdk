use {
    trezoa_program_runtime::invoke_context::BuiltinFunctionWithContext,
    trezoa_sdk::{
        bpf_loader, bpf_loader_deprecated, bpf_loader_upgradeable, feature_set, pubkey::Pubkey,
    },
};

/// Transitions of built-in programs at epoch bondaries when features are activated.
pub struct BuiltinPrototype {
    pub feature_id: Option<Pubkey>,
    pub program_id: Pubkey,
    pub name: &'static str,
    pub entrypoint: BuiltinFunctionWithContext,
}

impl std::fmt::Debug for BuiltinPrototype {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut builder = f.debug_struct("BuiltinPrototype");
        builder.field("program_id", &self.program_id);
        builder.field("name", &self.name);
        builder.field("feature_id", &self.feature_id);
        builder.finish()
    }
}

#[cfg(RUSTC_WITH_SPECIALIZATION)]
impl trezoa_frozen_abi::abi_example::AbiExample for BuiltinPrototype {
    fn example() -> Self {
        // BuiltinPrototype isn't serializable by definition.
        trezoa_program_runtime::declare_process_instruction!(MockBuiltin, 0, |_invoke_context| {
            // Do nothing
            Ok(())
        });
        Self {
            feature_id: None,
            program_id: Pubkey::default(),
            name: "",
            entrypoint: MockBuiltin::vm,
        }
    }
}

pub static BUILTINS: &[BuiltinPrototype] = &[
    BuiltinPrototype {
        feature_id: None,
        program_id: trezoa_system_program::id(),
        name: "system_program",
        entrypoint: trezoa_system_program::system_processor::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: trezoa_vote_program::id(),
        name: "vote_program",
        entrypoint: trezoa_vote_program::vote_processor::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: trezoa_stake_program::id(),
        name: "stake_program",
        entrypoint: trezoa_stake_program::stake_instruction::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: trezoa_config_program::id(),
        name: "config_program",
        entrypoint: trezoa_config_program::config_processor::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: bpf_loader_deprecated::id(),
        name: "trezoa_bpf_loader_deprecated_program",
        entrypoint: trezoa_bpf_loader_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: bpf_loader::id(),
        name: "trezoa_bpf_loader_program",
        entrypoint: trezoa_bpf_loader_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: bpf_loader_upgradeable::id(),
        name: "trezoa_bpf_loader_upgradeable_program",
        entrypoint: trezoa_bpf_loader_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: trezoa_sdk::compute_budget::id(),
        name: "compute_budget_program",
        entrypoint: trezoa_compute_budget_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: None,
        program_id: trezoa_sdk::address_lookup_table::program::id(),
        name: "address_lookup_table_program",
        entrypoint: trezoa_address_lookup_table_program::processor::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: Some(feature_set::zk_token_sdk_enabled::id()),
        program_id: trezoa_zk_token_sdk::zk_token_proof_program::id(),
        name: "zk_token_proof_program",
        entrypoint: trezoa_zk_token_proof_program::Entrypoint::vm,
    },
    BuiltinPrototype {
        feature_id: Some(feature_set::enable_program_runtime_v2_and_loader_v4::id()),
        program_id: trezoa_sdk::loader_v4::id(),
        name: "loader_v4",
        entrypoint: trezoa_loader_v4_program::Entrypoint::vm,
    },
];
