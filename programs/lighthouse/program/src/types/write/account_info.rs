use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub enum AccountInfoField {
    Key,
    Lamports,
    DataLength,
    Owner,
    RentEpoch,
    Executable,
}
