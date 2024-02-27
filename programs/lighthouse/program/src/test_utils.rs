use solana_sdk::{signature::Keypair, signer::EncodableKeypair};
use test_program::processor::TestAccountV1;

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
