use super::{AccountValidation, CheckedAccount, DerivedAddress};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[derive(Clone)]
pub struct MemoryAccount<'a, 'info> {
    info: &'a AccountInfo<'info>,
}

pub struct MemoryAccountSeeds<'a> {
    pub payer: &'a Pubkey,
    pub memory_index: u8,
    pub bump: Option<u8>,
}

impl<'a, 'info> DerivedAddress<MemoryAccountSeeds<'a>> for MemoryAccount<'a, 'info> {
    fn get_seeds(seeds: MemoryAccountSeeds<'a>) -> Vec<Vec<u8>> {
        let MemoryAccountSeeds {
            payer,
            memory_index,
            bump,
        } = seeds;

        vec![
            b"memory".to_vec(),
            payer.to_bytes().to_vec(),
            if let Some(bump) = bump {
                vec![memory_index, bump]
            } else {
                vec![memory_index]
            },
        ]
    }
}

impl<'a, 'info> CheckedAccount<'a, 'info> for MemoryAccount<'a, 'info> {
    fn get_validations() -> Vec<AccountValidation<'a>> {
        vec![AccountValidation::IsProgramOwned(crate::ID)]
    }

    fn info(&self) -> &'a AccountInfo<'info> {
        self.info
    }

    fn new(account: &'a AccountInfo<'info>) -> Self {
        Self { info: account }
    }
}
