use crate::{
    err, err_msg,
    error::LighthouseError,
    types::assert::{Assert, LogLevel},
    utils::{keys_equal, Result},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    bpf_loader_upgradeable::{self, UpgradeableLoaderState},
};
use std::{fmt::Debug, slice::Iter};

#[derive(Clone)]
pub(crate) struct AssertUpgradeableLoaderStateContext<'a, 'info> {
    pub(crate) upgradeable_loader_account: &'a AccountInfo<'info>,
}

impl<'a, 'info> AssertUpgradeableLoaderStateContext<'a, 'info> {
    pub(crate) fn load(account_iter: &mut Iter<'a, AccountInfo<'info>>) -> Result<Self> {
        let upgradeable_loader_account = next_account_info(account_iter)?;

        if !keys_equal(
            upgradeable_loader_account.owner,
            &bpf_loader_upgradeable::ID,
        ) {
            return Err(LighthouseError::AccountOwnerMismatch.into());
        }

        Ok(Self {
            upgradeable_loader_account,
        })
    }
}

pub(crate) fn assert_upgradeable_loader_state<
    'a,
    'info,
    T: for<'b> Assert<&'b UpgradeableLoaderState> + Debug,
>(
    ctx: AssertUpgradeableLoaderStateContext<'a, 'info>,
    assertion: &T,
    log_level: LogLevel,
) -> Result<()> {
    let data = ctx
        .upgradeable_loader_account
        .try_borrow_data()
        .map_err(LighthouseError::failed_borrow_err)?;

    let state: UpgradeableLoaderState = bincode::deserialize(&data).map_err(|e| {
        err_msg!("Failed to deserialize upgradeable loader state", e);
        err!(LighthouseError::AccountBorrowFailed)
    })?;

    assertion.evaluate(&state, log_level)
}

pub(crate) fn assert_upgradeable_loader_state_multi<
    'a,
    'info,
    T: for<'b> Assert<&'b UpgradeableLoaderState> + Debug,
>(
    ctx: AssertUpgradeableLoaderStateContext<'a, 'info>,
    assertions: &[T],
    log_level: LogLevel,
) -> Result<()> {
    let data = ctx
        .upgradeable_loader_account
        .try_borrow_data()
        .map_err(LighthouseError::failed_borrow_err)?;

    let state: UpgradeableLoaderState = bincode::deserialize(&data).map_err(|e| {
        err_msg!("Failed to deserialize upgradeable loader state", e);
        err!(LighthouseError::AccountBorrowFailed)
    })?;

    for (i, assertion) in assertions.iter().enumerate() {
        assertion
            .evaluate(&state, log_level)
            .map_err(|e| LighthouseError::map_multi_err(e, i as u32))?;
    }

    Ok(())
}
