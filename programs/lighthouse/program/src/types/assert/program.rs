use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct ProgramInfo {
    pub id: Pubkey,
    pub executable_data: Pubkey,
}
