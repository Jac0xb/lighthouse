use crate::{
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::{keys_equal, Result},
};
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    stake::state::StakeStateV2,
};
use std::{fmt::Debug, slice::Iter};

#[derive(Clone)]
pub(crate) struct AssertStakeAccountContext<'a, 'info> {
    pub(crate) stake_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> AssertStakeAccountContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        let stake_account = next_account_info(account_iter)?;

        if !keys_equal(stake_account.owner, &solana_program::stake::program::ID) {
            return Err(LighthouseError::AccountOwnerMismatch.into());
        }

        Ok(Self { stake_account })
    }
}

pub(crate) fn assert_stake_account<'a, 'info, T: for<'b> Assert<&'b StakeStateV2> + Debug>(
    ctx: AssertStakeAccountContext<'a, 'info>,
    assertion: T,
    log_level: LogLevel,
) -> Result<()> {
    let data = ctx
        .stake_account
        .try_borrow_data()
        .map_err(LighthouseError::failed_borrow_err)?;

    let stake_account =
        StakeStateV2::deserialize(&mut data.as_ref()).map_err(LighthouseError::stake_deser_err)?;

    assertion.evaluate(&stake_account, log_level)
}

pub(crate) fn assert_stake_account_multi<'a, 'info, T: for<'b> Assert<&'b StakeStateV2> + Debug>(
    ctx: AssertStakeAccountContext<'a, 'info>,
    assertions: &[T],
    log_level: LogLevel,
) -> Result<()> {
    let data = ctx
        .stake_account
        .try_borrow_data()
        .map_err(LighthouseError::failed_borrow_err)?;

    let stake_account =
        StakeStateV2::deserialize(&mut data.as_ref()).map_err(LighthouseError::stake_deser_err)?;

    for (i, assertion) in assertions.iter().enumerate() {
        assertion
            .evaluate(&stake_account, log_level)
            .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
    }

    Ok(())
}
