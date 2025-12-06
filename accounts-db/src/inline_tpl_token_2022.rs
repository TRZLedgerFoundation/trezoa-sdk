/// Partial TPL Token declarations inlined to avoid an external dependency on the tpl-token-2022 crate
use crate::inline_tpl_token::{self, GenericTokenAccount};

trezoa_sdk::declare_id!("TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb");

// `tpl_token_program_2022::extension::AccountType::Account` ordinal value
pub const ACCOUNTTYPE_ACCOUNT: u8 = 2;

pub struct Account;
impl GenericTokenAccount for Account {
    fn valid_account_data(account_data: &[u8]) -> bool {
        inline_tpl_token::Account::valid_account_data(account_data)
            || ACCOUNTTYPE_ACCOUNT
                == *account_data
                    .get(inline_tpl_token::Account::get_packed_len())
                    .unwrap_or(&0)
    }
}
