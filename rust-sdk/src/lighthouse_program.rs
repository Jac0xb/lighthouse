use crate::TxBuilder;
use anchor_lang::*;
use kaigan::types::RemainderVec;
use lighthouse::{
    processor::{create_memory_account::CreateMemoryAccountParameters, write::WriteParameters},
    types::{
        AccountDataAssertion, AccountInfoAssertion, AssertionConfigV1, MintAccountAssertion,
        SysvarClockAssertion, TokenAccountAssertion, WriteTypeParameter,
    },
};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
};

pub struct LighthouseProgram {}

impl LighthouseProgram {
    fn tx_builder(&mut self, ixs: Vec<Instruction>, payer: Pubkey) -> TxBuilder {
        TxBuilder {
            payer,
            ixs,
            look_up_tables: None,
        }
    }

    pub fn entrypoint(&mut self, payer: Pubkey) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![],
                data: vec![],
            }],
            payer,
        )
    }

    pub fn assert_account_data(
        &mut self,
        payer: Pubkey,
        target_account: Pubkey,
        assertion: AccountDataAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![AccountMeta::new_readonly(target_account, false)],
                data: lighthouse::LighthouseInstruction::AssertAccountData(assertion)
                    .try_to_vec()
                    .unwrap(),
            }],
            payer,
        )
    }

    pub fn assert_account_info(
        &mut self,
        payer: Pubkey,
        target_account: Pubkey,
        assertion: AccountInfoAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![AccountMeta::new_readonly(target_account, false)],
                data: lighthouse::LighthouseInstruction::AssertAccountInfo(assertion)
                    .try_to_vec()
                    .unwrap(),
            }],
            payer,
        )
    }

    pub fn assert_mint_account(
        &mut self,
        payer: Pubkey,
        target_account: Pubkey,
        assertion: MintAccountAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![AccountMeta::new_readonly(target_account, false)],
                data: lighthouse::LighthouseInstruction::AssertMintAccount(assertion)
                    .try_to_vec()
                    .unwrap(),
            }],
            payer,
        )
    }

    pub fn assert_mint_account_multi(
        &mut self,
        payer: Pubkey,
        target_account: Pubkey,
        assertions: Vec<MintAccountAssertion>,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        let mut remainder_assertions =
            RemainderVec::<MintAccountAssertion>::try_from_slice(&[]).unwrap();

        for assertion in assertions {
            remainder_assertions.push(assertion);
        }

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![AccountMeta::new_readonly(target_account, false)],
                data: lighthouse::LighthouseInstruction::AssertMintAccountMulti(
                    remainder_assertions,
                )
                .try_to_vec()
                .unwrap(),
            }],
            payer,
        )
    }

    // pub fn assert_data_hash(
    //     &mut self,
    //     payer: Pubkey,
    //     target_account: Pubkey,
    //     assertion: AccountDataAssertion,
    //     _config: Option<AssertionConfigV1>,
    // ) -> TxBuilder {
    //     self.tx_builder(
    //         vec![Instruction {
    //             program_id: lighthouse::ID,
    //             accounts: vec![
    //                 AccountMeta::new_readonly(lighthouse::ID, false),
    //                 AccountMeta::new_readonly(target_account, false),
    //             ],
    //             data: lighthouse::LighthouseInstruction::AssertDataHash(assertion)
    //                 .try_to_vec()
    //                 .unwrap(),
    //         }],
    //         payer,
    //     )
    // }

    pub fn assert_token_account(
        &mut self,
        payer: Pubkey,
        target_account: Pubkey,
        assertion: TokenAccountAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![AccountMeta::new_readonly(target_account, false)],
                data: lighthouse::LighthouseInstruction::AssertTokenAccount(assertion)
                    .try_to_vec()
                    .unwrap(),
            }],
            payer,
        )
    }

    pub fn assert_token_account_multi(
        &mut self,
        payer: Pubkey,
        target_account: Pubkey,
        assertions: Vec<TokenAccountAssertion>,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        let mut remainder_assertions =
            RemainderVec::<TokenAccountAssertion>::try_from_slice(&[]).unwrap();

        for assertion in assertions {
            remainder_assertions.push(assertion);
        }

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![
                    AccountMeta::new_readonly(target_account, false),
                    AccountMeta::new_readonly(lighthouse::ID, false),
                ],
                data: lighthouse::LighthouseInstruction::AssertTokenAccountMulti(
                    remainder_assertions,
                )
                .try_to_vec()
                .unwrap(),
            }],
            payer,
        )
    }

    pub fn assert_sysvar_clock(
        &mut self,
        payer: Pubkey,
        assertion: SysvarClockAssertion,
        _config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::ID,
                accounts: vec![],
                data: lighthouse::LighthouseInstruction::AssertSysvarClock(assertion)
                    .try_to_vec()
                    .unwrap(),
            }],
            payer,
        )
    }

    pub fn create_memory_account(
        &mut self,
        payer: Pubkey,
        memory_index: u8,
        memory_account_size: u64,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: vec![
                    AccountMeta::new_readonly(lighthouse::id(), false),
                    AccountMeta::new(payer, true),
                    AccountMeta::new(find_memory_account(payer, memory_index).0, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: lighthouse::LighthouseInstruction::CreateMemoryAccount(
                    CreateMemoryAccountParameters {
                        memory_index,
                        memory_account_size,
                    },
                )
                .try_to_vec()
                .unwrap(),
            }],
            payer,
        )
    }

    // pub fn close_memory_account(&mut self, payer: Pubkey, memory_index: u8) -> TxBuilder {
    //     let (memory_account, memory_account_bump) =
    //         find_memory_account(payer, memory_index);

    //     self.tx_builder(
    //         vec![Instruction {
    //             program_id: lighthouse::id(),
    //             accounts: (lighthouse::accounts::CloseMemoryAccountV1 {
    //                 signer: payer,
    //                 system_program: system_program::id(),
    //                 memory_account,
    //                 rent: sysvar::rent::id(),
    //             })
    //             .to_account_metas(None),
    //             data: lighthouse::instruction::CloseMemoryAccountV1 {
    //                 memory_index,
    //                 memory_bump: memory_account_bump,
    //             }
    //             .data(),
    //         }],
    //         payer,
    //     )
    // }

    pub fn write_v1(
        &mut self,
        payer: Pubkey,
        source_account: Pubkey,
        memory_index: u8,
        write_type_parameter: WriteTypeParameter,
    ) -> TxBuilder {
        let (memory_account, memory_account_bump) = find_memory_account(payer, memory_index);

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: vec![
                    AccountMeta::new_readonly(lighthouse::id(), false),
                    AccountMeta::new(payer, true),
                    AccountMeta::new(memory_account, false),
                    AccountMeta::new_readonly(source_account, false),
                    AccountMeta::new_readonly(system_program::ID, false),
                ],
                data: lighthouse::LighthouseInstruction::Write(WriteParameters {
                    memory_index,
                    memory_account_bump,
                    write_type: write_type_parameter,
                })
                .try_to_vec()
                .unwrap(),
            }],
            payer,
        )
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
