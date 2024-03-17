use super::{AccountValidation, CheckedAccount, DerivedAddress};
use solana_program::{account_info::AccountInfo, pubkey::Pubkey};

#[derive(Clone)]
pub(crate) struct Memory<'a, 'info> {
    pub(crate) info: &'a AccountInfo<'info>,
}

pub struct MemorySeeds<'a> {
    pub payer: &'a Pubkey,
    pub memory_id: u8,
    pub bump: Option<u8>,
}

impl<'a, 'info> DerivedAddress<MemorySeeds<'a>> for Memory<'a, 'info> {
    fn get_seeds(seeds: MemorySeeds<'a>) -> Vec<Vec<u8>> {
        let MemorySeeds {
            payer,
            memory_id,
            bump,
        } = seeds;

        vec![
            b"memory".to_vec(),
            payer.to_bytes().to_vec(),
            vec![memory_id],
            bump.map_or_else(std::vec::Vec::new, |b| vec![b]),
        ]
    }
}

impl<'a, 'info> CheckedAccount<'a, 'info> for Memory<'a, 'info> {
    fn info(&self) -> &'a AccountInfo<'info> {
        self.info
    }

    fn get_validations() -> Option<Vec<AccountValidation<'a>>> {
        None
    }

    fn new(account: &'a AccountInfo<'info>) -> Self {
        Self { info: account }
    }
}
