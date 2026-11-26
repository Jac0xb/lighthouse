#[allow(unused)]
#[allow(clippy::identity_op)]
mod generated;
pub mod hooked;
pub mod registry;
#[cfg(feature = "sdk")]
pub mod transaction;
#[cfg(feature = "sdk")]
pub mod utils;

pub use generated::programs::LIGHTHOUSE_ID;
pub use generated::programs::LIGHTHOUSE_ID as ID;
pub use generated::types;
pub use lighthouse_common::{CompactU64, LEB128Vec};
pub use solana_address::{error::AddressError, Address};

pub mod instructions {
    pub use crate::generated::instructions::{
        AssertAccountDataBuilder, AssertAccountDataMultiBuilder, AssertAccountDeltaBuilder,
        AssertAccountInfoBuilder, AssertAccountInfoMultiBuilder,
        AssertBubblegumTreeConfigAccountBuilder, AssertMerkleTreeAccountBuilder,
        AssertMintAccountBuilder, AssertMintAccountMultiBuilder, AssertStakeAccountBuilder,
        AssertStakeAccountMultiBuilder, AssertSysvarClockBuilder, AssertTokenAccountBuilder,
        AssertTokenAccountMultiBuilder, AssertUpgradeableLoaderAccountBuilder,
        AssertUpgradeableLoaderAccountMultiBuilder, MemoryCloseBuilder, MemoryWriteBuilder,
    };
}

#[cfg(feature = "cpi")]
pub mod cpi {
    pub use crate::generated::instructions::{
        AssertAccountDataCpiBuilder, AssertAccountDataMultiCpiBuilder, AssertAccountDeltaCpiBuilder,
        AssertAccountInfoCpiBuilder, AssertAccountInfoMultiCpiBuilder,
        AssertBubblegumTreeConfigAccountCpiBuilder, AssertMerkleTreeAccountCpiBuilder,
        AssertMintAccountCpiBuilder, AssertMintAccountMultiCpiBuilder, AssertStakeAccountCpiBuilder,
        AssertStakeAccountMultiCpiBuilder, AssertSysvarClockCpiBuilder,
        AssertTokenAccountCpiBuilder, AssertTokenAccountMultiCpiBuilder,
        AssertUpgradeableLoaderAccountCpiBuilder, AssertUpgradeableLoaderAccountMultiCpiBuilder,
        MemoryCloseCpiBuilder, MemoryWriteCpiBuilder,
    };
}

pub mod errors {
    pub use crate::generated::errors::*;
}

pub fn find_memory_pda(payer: Address, memory_id: u8) -> (Address, u8) {
    Address::find_program_address(&[b"memory", payer.as_ref(), &[memory_id]], &crate::ID)
}

pub fn find_memory_pda_bump_iterate(
    payer: Address,
    memory_id: u8,
    bump_skip: u8,
    start_bump: Option<u8>,
) -> Option<(Address, u8)> {
    let seeds = [b"memory".as_ref(), payer.as_ref(), &[memory_id]];

    let mut bump_seed = [start_bump.unwrap_or(u8::MAX)];
    let mut bump_skip = bump_skip as usize;

    loop {
        let mut seeds_with_bump = seeds.to_vec();
        seeds_with_bump.push(&bump_seed);
        match Address::create_program_address(&seeds_with_bump, &crate::ID) {
            Ok(address) => {
                if bump_skip == 0 {
                    return Some((address, bump_seed[0]));
                } else {
                    bump_skip -= 1;
                }
            }
            Err(AddressError::InvalidSeeds) => {}
            _ => break,
        }
        if bump_seed[0] == 0 {
            break;
        }
        bump_seed[0] -= 1;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::{
        AssertAccountDataBuilder, AssertAccountDataMultiBuilder, AssertAccountInfoBuilder,
        AssertSysvarClockBuilder, MemoryWriteBuilder,
    };
    use crate::registry::{AssertVoteAccountBuilder, VoteAccountAssertion};
    use crate::types::{
        AccountInfoAssertion, DataValueAssertion, EquatableOperator, IntegerOperator, LogLevel,
        SysvarClockAssertion, WriteType,
    };

    #[test]
    fn test_find_memory_pda() {
        let payer = Address::new_from_array([1u8; 32]);
        let (pda, bump) = find_memory_pda(payer, 0);
        let derived = Address::create_program_address(
            &[b"memory", payer.as_ref(), &[0u8], &[bump]],
            &LIGHTHOUSE_ID,
        )
        .unwrap();
        assert_eq!(pda, derived);

        let iterated = find_memory_pda_bump_iterate(payer, 0, 0, None).unwrap();
        assert_eq!(iterated, (pda, bump));
    }

    #[test]
    fn test_instruction_builders_and_wire_format() {
        let target = Address::new_from_array([2u8; 32]);

        let ix = AssertAccountInfoBuilder::new()
            .target_account(target)
            .assertion(AccountInfoAssertion::Lamports {
                value: 1_000_000,
                operator: IntegerOperator::GreaterThanOrEqual,
            })
            .instruction();
        assert_eq!(ix.program_id, LIGHTHOUSE_ID);
        assert_eq!(ix.accounts.len(), 1);
        assert_eq!(ix.accounts[0].pubkey, target);
        // discriminator 5 (AssertAccountInfo), log_level 0 (Silent), variant 0 (Lamports)
        assert_eq!(ix.data[0], 5);
        assert_eq!(ix.data[1], 0);

        let data_ix = AssertAccountDataBuilder::new()
            .target_account(target)
            .offset(CompactU64(64))
            .assertion(DataValueAssertion::U64 {
                value: 42,
                operator: IntegerOperator::Equal,
            })
            .instruction();
        assert_eq!(data_ix.data[0], 2);

        let multi_ix = AssertAccountDataMultiBuilder::new()
            .target_account(target)
            .assertions(
                vec![hooked::AccountDataAssertion {
                    offset: CompactU64(0),
                    assertion: DataValueAssertion::U8 {
                        value: 1,
                        operator: IntegerOperator::Equal,
                    },
                }]
                .into(),
            )
            .instruction();
        assert_eq!(multi_ix.data[0], 3);

        let clock_ix = AssertSysvarClockBuilder::new()
            .log_level(LogLevel::PlaintextMessage)
            .assertion(SysvarClockAssertion::Slot {
                value: 100,
                operator: IntegerOperator::GreaterThan,
            })
            .instruction();
        assert_eq!(clock_ix.accounts.len(), 0);
        assert_eq!(clock_ix.data[0], 15);

        let (memory, bump) = find_memory_pda(target, 0);
        let write_ix = MemoryWriteBuilder::new()
            .program_id(LIGHTHOUSE_ID)
            .payer(target)
            .memory(memory)
            .source_account(target)
            .memory_bump(bump)
            .write_offset(CompactU64(0))
            .write_type(WriteType::AccountData {
                offset: 0,
                data_length: 8,
            })
            .instruction();
        assert_eq!(write_ix.data[0], 0);
        assert_eq!(write_ix.accounts.len(), 5);

        let vote_ix = AssertVoteAccountBuilder::new()
            .target_account(target)
            .assertion(VoteAccountAssertion::AuthorizedWithdrawer {
                value: target,
                operator: EquatableOperator::Equal,
            })
            .instruction();
        assert_eq!(vote_ix.data[0], 3);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_serde_roundtrip() {
        let addr = Address::new_from_array([7u8; 32]);
        let assertion = DataValueAssertion::Pubkey {
            value: addr,
            operator: EquatableOperator::Equal,
        };
        let json = serde_json::to_string(&assertion).unwrap();
        let decoded: DataValueAssertion = serde_json::from_str(&json).unwrap();
        assert_eq!(assertion, decoded);
    }

    #[cfg(feature = "sdk")]
    #[test]
    fn test_parse_evaluation_payloads_from_logs() {
        use base64::{engine::general_purpose::STANDARD, Engine as _};
        use borsh::BorshDeserialize;
        let expected = types::AssertionResult::U64(Some(100), Some(100), 0, true);
        let bytes = borsh::to_vec(&expected).unwrap();
        let log_line = format!("Program data: {}", STANDARD.encode(&bytes));
        let other_line = "Program log: hello".to_string();
        let parsed =
            utils::parse_evaluation_payloads_from_logs(vec![&other_line, &log_line]).unwrap();
        assert_eq!(parsed, vec![expected]);
        assert_eq!(
            types::AssertionResult::try_from_slice(&bytes).unwrap(),
            parsed[0]
        );
    }
}
