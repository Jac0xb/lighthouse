use anchor_lang::*;
use lighthouse::types::{Assertion, AssertionConfigV1, CompactAssertionArray, WriteTypeParameter};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program, sysvar,
};
use solana_sdk::signature::{Keypair, Signer};

use crate::TxBuilder;

pub struct LighthouseProgram {}

impl<'a> LighthouseProgram {
    fn tx_builder(&mut self, ixs: Vec<Instruction>, payer: Pubkey) -> TxBuilder {
        TxBuilder {
            payer,
            ixs,
            look_up_tables: vec![],
        }
    }

    pub fn create_assert(
        &'a mut self,
        payer: &'a Keypair,
        target_account: Pubkey,
        assertion: Assertion,
        config: Option<AssertionConfigV1>,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: (lighthouse::accounts::AssertV1 { target_account })
                    .to_account_metas(None),
                data: lighthouse::instruction::AssertV1 { assertion, config }.data(),
            }],
            payer.pubkey(),
        )
    }

    pub fn create_assert_compact(
        &mut self,
        payer: &Keypair,
        target_account: Pubkey,
        assertion: Assertion,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: (lighthouse::accounts::AssertCompactV1 { target_account })
                    .to_account_metas(None),
                data: lighthouse::instruction::AssertCompactV1 { assertion }.data(),
            }],
            payer.pubkey(),
        )
    }

    pub fn create_assert_multi(
        &mut self,
        payer: &Keypair,
        assertions: Vec<Assertion>,
        additional_accounts: Vec<Pubkey>,
    ) -> TxBuilder {
        let mut accounts = (lighthouse::accounts::AssertMultiV1 {
            system_program: system_program::id(),
        })
        .to_account_metas(None);

        // append additional_accounts to accounts
        accounts.append(
            &mut additional_accounts
                .into_iter()
                .map(|pubkey| AccountMeta::new_readonly(pubkey, false))
                .collect(),
        );

        let length = (lighthouse::instruction::AssertMultiV1 {
            assertions: assertions.clone(),
            config: Some(AssertionConfigV1 { verbose: true }),
        })
        .data()
        .len();

        println!("length: {}", length);

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts,
                data: (lighthouse::instruction::AssertMultiV1 {
                    assertions,
                    config: Some(AssertionConfigV1 { verbose: true }),
                })
                .data(),
            }],
            payer.pubkey(),
        )
    }

    pub fn create_assert_multi_compact(
        &mut self,
        payer: &Keypair,
        assertions: Vec<Assertion>,
        additional_accounts: Vec<Pubkey>,
    ) -> TxBuilder {
        let mut accounts = (lighthouse::accounts::AssertMultiCompactV1 {
            system_program: system_program::id(),
        })
        .to_account_metas(None);

        // append additional_accounts to accounts
        accounts.append(
            &mut additional_accounts
                .into_iter()
                .map(|pubkey| AccountMeta::new_readonly(pubkey, false))
                .collect(),
        );

        let assertion_array: CompactAssertionArray = match assertions.len() {
            1 => CompactAssertionArray::Size1([assertions[0].clone()]),
            2 => CompactAssertionArray::Size2([assertions[0].clone(), assertions[1].clone()]),
            3 => CompactAssertionArray::Size3([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
            ]),
            4 => CompactAssertionArray::Size4([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
            ]),
            5 => CompactAssertionArray::Size5([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
            ]),
            6 => CompactAssertionArray::Size6([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
            ]),
            7 => CompactAssertionArray::Size7([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
            ]),
            8 => CompactAssertionArray::Size8([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
            ]),
            9 => CompactAssertionArray::Size9([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
            ]),
            10 => CompactAssertionArray::Size10([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
                assertions[9].clone(),
            ]),
            11 => CompactAssertionArray::Size11([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
                assertions[9].clone(),
                assertions[10].clone(),
            ]),
            12 => CompactAssertionArray::Size12([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
                assertions[9].clone(),
                assertions[10].clone(),
                assertions[11].clone(),
            ]),
            13 => CompactAssertionArray::Size13([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
                assertions[9].clone(),
                assertions[10].clone(),
                assertions[11].clone(),
                assertions[12].clone(),
            ]),
            14 => CompactAssertionArray::Size14([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
                assertions[9].clone(),
                assertions[10].clone(),
                assertions[11].clone(),
                assertions[12].clone(),
                assertions[13].clone(),
            ]),
            15 => CompactAssertionArray::Size15([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
                assertions[9].clone(),
                assertions[10].clone(),
                assertions[11].clone(),
                assertions[12].clone(),
                assertions[13].clone(),
                assertions[14].clone(),
            ]),
            16 => CompactAssertionArray::Size16([
                assertions[0].clone(),
                assertions[1].clone(),
                assertions[2].clone(),
                assertions[3].clone(),
                assertions[4].clone(),
                assertions[5].clone(),
                assertions[6].clone(),
                assertions[7].clone(),
                assertions[8].clone(),
                assertions[9].clone(),
                assertions[10].clone(),
                assertions[11].clone(),
                assertions[12].clone(),
                assertions[13].clone(),
                assertions[14].clone(),
                assertions[15].clone(),
            ]),
            _ => panic!("Too many assertions for compact array instruction!"),
        };

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts,
                data: (lighthouse::instruction::AssertMultiCompactV1 {
                    assertions: assertion_array,
                })
                .data(),
            }],
            payer.pubkey(),
        )
    }

    pub fn create_memory_account(
        &mut self,
        payer: &Keypair,
        memory_index: u8,
        memory_account_size: u64,
    ) -> TxBuilder {
        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: (lighthouse::accounts::CreateMemoryAccountV1 {
                    system_program: system_program::id(),
                    signer: payer.pubkey(),
                    memory_account: find_memory_account(payer.pubkey(), memory_index).0,
                    rent: sysvar::rent::id(),
                })
                .to_account_metas(None),
                data: (lighthouse::instruction::CreateMemoryAccountV1 {
                    memory_index,
                    memory_account_size,
                })
                .data(),
            }],
            payer.pubkey(),
        )
    }

    pub fn close_memory_account(&mut self, payer: &Keypair, memory_index: u8) -> TxBuilder {
        let (memory_account, memory_account_bump) =
            find_memory_account(payer.pubkey(), memory_index);

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: (lighthouse::accounts::CloseMemoryAccountV1 {
                    signer: payer.pubkey(),
                    system_program: system_program::id(),
                    memory_account,
                    rent: sysvar::rent::id(),
                })
                .to_account_metas(None),
                data: lighthouse::instruction::CloseMemoryAccountV1 {
                    memory_index,
                    memory_bump: memory_account_bump,
                }
                .data(),
            }],
            payer.pubkey(),
        )
    }

    pub fn write_v1(
        &mut self,
        payer: &Keypair,
        source_account: Pubkey,
        memory_index: u8,
        write_type_parameter: WriteTypeParameter,
    ) -> TxBuilder {
        let (memory_account, memory_account_bump) =
            find_memory_account(payer.pubkey(), memory_index);

        let write_type_clone = write_type_parameter.clone();
        let mut ix_accounts = lighthouse::accounts::WriteV1 {
            system_program: system_program::id(),
            signer: payer.pubkey(),
            memory_account,
        }
        .to_account_metas(None);
        ix_accounts.append(&mut vec![AccountMeta::new(source_account, false)]);

        self.tx_builder(
            vec![Instruction {
                program_id: lighthouse::id(),
                accounts: ix_accounts,
                data: (lighthouse::instruction::WriteV1 {
                    write_type: write_type_clone,
                    memory_index,
                    memory_account_bump,
                })
                .data(),
            }],
            payer.pubkey(),
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
