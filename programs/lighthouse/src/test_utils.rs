use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::{
    program_error::ProgramError, pubkey::Pubkey, signature::Keypair, signer::EncodableKeypair,
};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct TestAccountV1 {
    pub u8: u8,
    pub i8: i8,
    pub u16: u16,
    pub i16: i16,
    pub u32: u32,
    pub i32: i32,
    pub u64: u64,
    pub i64: i64,
    pub u128: u128,
    pub i128: i128,
    pub bytes: [u8; 32],
    pub true_field: bool,
    pub false_field: bool,
    pub option_u8: Option<u8>,
    pub option_u8_none: Option<u8>,
    pub option_u16: Option<u16>,
    pub option_u16_none: Option<u16>,
    pub pubkey: Pubkey,
    pub vec: Vec<u8>,
}

pub fn create_test_account() -> TestAccountV1 {
    TestAccountV1 {
        u8: 1,                                     // 0
        i8: -1,                                    // 1
        u16: (u8::MAX as u16) + 1,                 // 2
        i16: (i8::MIN as i16) - 1,                 // 4
        u32: (u16::MAX as u32) + 1,                // 6
        i32: (i16::MIN as i32) - 1,                // 10
        u64: (u32::MAX as u64) + 1,                // 14
        i64: (i32::MIN as i64) - 1,                // 22
        u128: (u64::MAX as u128) + 1,              // 30 bytes
        i128: (i64::MIN as i128) - 1,              // 46 bytes
        bytes: [u8::MAX; 32],                      // 62
        true_field: true,                          // 94
        false_field: false,                        // 95
        option_u8: Some(u8::MAX),                  // 96
        option_u8_none: None,                      // 99
        option_u16: Some(u16::MAX),                // 100
        option_u16_none: None,                     // 103
        pubkey: Keypair::new().encodable_pubkey(), // 103
        vec: vec![u8::MAX; 32],
    }
}

pub fn assert_is_program_error(err: ProgramError, expected_error: ProgramError) {
    assert_eq!(err, expected_error);
}

pub fn assert_passed(result: Result<(), ProgramError>) {
    assert!(result.is_ok(), "{:?}", result)
}

pub fn assert_failed(result: Result<(), ProgramError>) {
    assert!(result.is_err(), "{:?}", result)
}
