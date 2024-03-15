use anchor_lang::prelude::*;
use arrayref::array_ref;

#[derive(Debug)]
#[account]
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

impl TestAccountV1 {
    pub fn try_to_vec_override(&self) -> Vec<u8> {
        self.try_to_vec().unwrap()
    }
}

#[derive(Accounts)]
pub(crate) struct CreateTestAccountV1<'info> {
    #[account(mut)]
    pub(crate) signer: Signer<'info>,

    #[account(
        init,
        space=512,
        payer=signer,
    )]
    pub(crate) test_account: Account<'info, TestAccountV1>,

    /// CHECK: ...
    pub(crate) slot_hashes: AccountInfo<'info>,
    pub(crate) rent: Sysvar<'info, Rent>,
    pub(crate) system_program: Program<'info, System>,
}

pub(crate) fn create_test_account<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateTestAccountV1<'info>>,
    random: bool,
) -> Result<()> {
    let test_account = &mut ctx.accounts.test_account;

    if random {
        let pubkey_sum = test_account
            .key()
            .to_bytes()
            .iter()
            .fold(0u128, |acc, &x| acc + (((x as u32) ^ 2) as u128));

        let seed = rand(&ctx.accounts.slot_hashes, pubkey_sum)?;

        let test_account_data = TestAccountV1 {
            u8: (seed % 256) as u8,
            i8: ((seed.wrapping_mul(3)) % 256) as i8,
            u16: ((seed.wrapping_mul(5)) % 65536) as u16,
            i16: ((seed.wrapping_mul(7)) % 65536) as i16,
            u32: ((seed.wrapping_mul(11)) % 4294967296) as u32,
            i32: ((seed.wrapping_mul(13)) % 4294967296) as i32,
            u64: ((seed.wrapping_mul(17)) as u64),
            i64: -((seed.wrapping_mul(31)) as i64),
            u128: (seed.wrapping_mul(37)) as u128,
            i128: -((seed.wrapping_mul(41)) as i128),
            bytes: [(seed.wrapping_mul(43)) as u8; 32],
            true_field: true,
            false_field: false,
            option_u8: Some(((seed.wrapping_mul(43)) % 256) as u8),
            option_u8_none: None,
            option_u16: Some(((seed.wrapping_mul(47)) % 65536) as u16),
            option_u16_none: None,
            pubkey: *ctx.accounts.signer.key,
            vec: vec![(seed.wrapping_mul(53)) as u8; 32],
        };

        test_account.set_inner(test_account_data);
    } else {
        let test_account_data = TestAccountV1 {
            u8: 1,
            i8: -1,
            u16: (u8::MAX as u16) + 1,
            i16: (i8::MIN as i16) - 1,
            u32: (u16::MAX as u32) + 1,
            i32: (i16::MIN as i32) - 1,
            u64: (u32::MAX as u64) + 1,
            i64: (i32::MIN as i64) - 1,
            u128: (u64::MAX as u128) + 1,
            i128: (i64::MIN as i128) - 1,
            bytes: [u8::MAX; 32],
            true_field: true,
            false_field: false,
            option_u8: Some(u8::MAX),
            option_u8_none: None,
            option_u16: Some(u16::MAX),
            option_u16_none: None,
            pubkey: *ctx.accounts.signer.key,
            vec: vec![u8::MAX; 32],
        };

        test_account.set_inner(test_account_data);
    }

    Ok(())
}

pub fn rand(slot_hashes: &AccountInfo, seed: u128) -> Result<usize> {
    assert!(*slot_hashes.key == solana_program::sysvar::slot_hashes::ID);

    let data = slot_hashes.data.borrow();
    let most_recent = array_ref![data, 12, 8];

    let clock = Clock::get()?;

    // seed for the random number is a combination of the slot_hash - timestamp
    let seed = usize::from_le_bytes(*most_recent)
        .wrapping_sub(clock.unix_timestamp as usize)
        .wrapping_mul(seed as usize);

    Ok(seed)
}
