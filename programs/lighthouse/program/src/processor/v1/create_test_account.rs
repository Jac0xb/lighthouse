use anchor_lang::prelude::*;

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
    pub string: String,
}

#[derive(Accounts)]
pub struct CreateTestAccountV1<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(
        init,
        seeds=[
            "test_account".as_bytes(),
        ],
        space=256,
        payer=signer,
        bump
    )]
    pub test_account: Account<'info, TestAccountV1>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_test_account<'info>(
    ctx: Context<'_, '_, '_, 'info, CreateTestAccountV1<'info>>,
) -> Result<()> {
    let test_account = &mut ctx.accounts.test_account;

    test_account.u8 = 1;
    test_account.i8 = -1;
    test_account.u16 = (u8::MAX as u16) + 1;
    test_account.i16 = (i8::MIN as i16) - 1;
    test_account.u32 = (u16::MAX as u32) + 1;
    test_account.i32 = (i16::MIN as i32) - 1;
    test_account.u64 = (u32::MAX as u64) + 1;
    test_account.i64 = (i32::MIN as i64) - 1;
    test_account.u128 = (u64::MAX as u128) + 1;
    test_account.i128 = (i64::MIN as i128) - 1;
    test_account.bytes = [u8::MAX; 32];
    test_account.string = "Hello, World!".to_string();

    Ok(())
}
