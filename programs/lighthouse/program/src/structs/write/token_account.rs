use anchor_lang::prelude::{
    borsh,
    borsh::{BorshDeserialize, BorshSerialize},
};
use anchor_spl::token::spl_token::state::AccountState;
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum LegacyTokenAccountDataField {
    Mint(Pubkey),
    Owner(Pubkey),
    Amount(u64),
    Delegate(Option<Pubkey>),
    State(u8),
    IsNative(Option<u64>),
    DelegatedAmount(u64),
    CloseAuthority(Option<Pubkey>),
}

pub fn account_state_from_u8(value: u8) -> AccountState {
    match value {
        0 => AccountState::Uninitialized,
        1 => AccountState::Initialized,
        2 => AccountState::Frozen,
        _ => panic!("Invalid account state"),
    }
}

pub fn u8_from_account_state(state: AccountState) -> u8 {
    match state {
        AccountState::Uninitialized => 0,
        AccountState::Initialized => 1,
        AccountState::Frozen => 2,
    }
}

// pub struct Account {
//     /// The mint associated with this account
//     pub mint: Pubkey,
//     /// The owner of this account.
//     pub owner: Pubkey,
//     /// The amount of tokens this account holds.
//     pub amount: u64,
//     /// If `delegate` is `Some` then `delegated_amount` represents
//     /// the amount authorized by the delegate
//     pub delegate: COption<Pubkey>,
//     /// The account's state
//     pub state: AccountState,
//     /// If is_native.is_some, this is a native token, and the value logs the rent-exempt reserve. An
//     /// Account is required to be rent-exempt, so the value is used by the Processor to ensure that
//     /// wrapped SOL accounts do not drop below this threshold.
//     pub is_native: COption<u64>,
//     /// The amount delegated
//     pub delegated_amount: u64,
//     /// Optional authority to close the account.
//     pub close_authority: COption<Pubkey>,
// }
