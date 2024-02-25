use crate::TxBuilder;
use borsh::{BorshDeserialize, BorshSerialize};
use kaigan::types::RemainderVec;
use lighthouse::{
    instruction::LighthouseInstruction,
    processor::{create_memory_account::CreateMemoryAccountParameters, write::WriteParameters},
    types::{
        AccountDataAssertion, AccountInfoAssertion, AssertionConfigV1, MintAccountAssertion,
        StakeAccountAssertion, SysvarClockAssertion, TokenAccountAssertion, WriteTypeParameter,
    },
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};
use solana_sdk::system_program;

pub struct LighthouseProgram {}

impl LighthouseProgram {
    fn tx_builder(&self, ixs: Vec<Instruction>) -> TxBuilder {
        TxBuilder {
            ixs,
            look_up_tables: None,
        }
    }

    pub fn entrypoint(&self) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![],
            data: vec![],
        }])
    }

    pub fn assert_account_data(
        &self,
        target_account: Pubkey,
        assertion: AccountDataAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![AccountMeta::new_readonly(target_account, false)],
            data: LighthouseInstruction::AssertAccountData(assertion)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn assert_account_info(
        &self,
        target_account: Pubkey,
        assertion: AccountInfoAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![AccountMeta::new_readonly(target_account, false)],
            data: LighthouseInstruction::AssertAccountInfo(assertion)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn assert_stake_account(
        &self,
        target_account: Pubkey,
        assertion: StakeAccountAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![AccountMeta::new_readonly(target_account, false)],
            data: LighthouseInstruction::AssertStakeAccount(assertion)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn assert_mint_account(
        &self,
        target_account: Pubkey,
        assertion: MintAccountAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![AccountMeta::new_readonly(target_account, false)],
            data: LighthouseInstruction::AssertMintAccount(assertion)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn assert_mint_account_multi(
        &self,
        target_account: Pubkey,
        assertions: Vec<MintAccountAssertion>,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        let mut remainder_assertions =
            RemainderVec::<MintAccountAssertion>::try_from_slice(&[]).unwrap();

        for assertion in assertions {
            remainder_assertions.push(assertion);
        }

        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![AccountMeta::new_readonly(target_account, false)],
            data: LighthouseInstruction::AssertMintAccountMulti(remainder_assertions)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn assert_token_account(
        &self,
        target_account: Pubkey,
        assertion: TokenAccountAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![AccountMeta::new_readonly(target_account, false)],
            data: LighthouseInstruction::AssertTokenAccount(assertion)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn assert_token_account_multi(
        &self,
        target_account: Pubkey,
        assertions: Vec<TokenAccountAssertion>,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        let mut remainder_assertions =
            RemainderVec::<TokenAccountAssertion>::try_from_slice(&[]).unwrap();

        for assertion in assertions {
            remainder_assertions.push(assertion);
        }

        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![
                AccountMeta::new_readonly(target_account, false),
                AccountMeta::new_readonly(lighthouse::ID, false),
            ],
            data: LighthouseInstruction::AssertTokenAccountMulti(remainder_assertions)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn assert_sysvar_clock(
        &self,
        assertion: SysvarClockAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::ID,
            accounts: vec![],
            data: LighthouseInstruction::AssertSysvarClock(assertion)
                .try_to_vec()
                .unwrap(),
        }])
    }

    pub fn create_memory_account(
        &self,
        payer: Pubkey,
        memory_index: u8,
        memory_account_size: u64,
    ) -> TxBuilder {
        self.tx_builder(vec![Instruction {
            program_id: lighthouse::id(),
            accounts: vec![
                AccountMeta::new_readonly(lighthouse::id(), false),
                AccountMeta::new(payer, true),
                AccountMeta::new(find_memory_account(payer, memory_index).0, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: LighthouseInstruction::CreateMemoryAccount(CreateMemoryAccountParameters {
                memory_index,
                memory_account_size,
            })
            .try_to_vec()
            .unwrap(),
        }])
    }

    pub fn write_v1(
        &self,
        payer: Pubkey,
        source_account: Pubkey,
        memory_index: u8,
        write_type_parameter: WriteTypeParameter,
    ) -> TxBuilder {
        let (memory_account, memory_account_bump) = find_memory_account(payer, memory_index);

        self.tx_builder(vec![Instruction {
            program_id: lighthouse::id(),
            accounts: vec![
                AccountMeta::new_readonly(lighthouse::id(), false),
                AccountMeta::new(payer, true),
                AccountMeta::new(memory_account, false),
                AccountMeta::new_readonly(source_account, false),
                AccountMeta::new_readonly(system_program::ID, false),
            ],
            data: LighthouseInstruction::Write(WriteParameters {
                memory_index,
                memory_account_bump,
                write_type: write_type_parameter,
            })
            .try_to_vec()
            .unwrap(),
        }])
    }
}

pub fn find_memory_account(user: Pubkey, memory_index: u8) -> (solana_program::pubkey::Pubkey, u8) {
    solana_program::pubkey::Pubkey::find_program_address(
        &[
            "memory".to_string().as_ref(),
            user.as_ref(),
            &[memory_index],
        ],
        &lighthouse::ID,
    )
}
