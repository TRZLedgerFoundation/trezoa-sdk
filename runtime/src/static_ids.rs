use {
    crate::inline_tpl_associated_token_account,
    trezoa_accounts_db::{inline_tpl_token, inline_tpl_token_2022},
    trezoa_sdk::pubkey::Pubkey,
};

lazy_static! {
    /// Vector of static token & mint IDs
    pub static ref STATIC_IDS: Vec<Pubkey> = vec![
        inline_tpl_associated_token_account::id(),
        inline_tpl_associated_token_account::program_v1_1_0::id(),
        inline_tpl_token::id(),
        inline_tpl_token::native_mint::id(),
        inline_tpl_token_2022::id(),
    ];
}
