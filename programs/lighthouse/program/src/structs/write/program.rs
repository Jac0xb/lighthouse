use anchor_lang::{
    prelude::{
        borsh,
        borsh::{BorshDeserialize, BorshSerialize},
    },
    Id,
};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct ProgramInfo {
    pub id: Pubkey,
    pub executable_data: Pubkey,
}

// impl AccountInfoData {
//     // length constant
//     pub fn size() -> u64 {
//         32 + 8 + 8 + 32 + 8 + 1 + 1 + 1
//     }
// }

// #[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
// pub enum AccountInfoDataField {
//     Key(Pubkey),
//     Lamports(u64),
//     DataLength(u64),
//     Owner(Pubkey),
//     RentEpoch(u64),
//     IsSigner(bool),
//     IsWritable(bool),
//     Executable(bool),
// }
