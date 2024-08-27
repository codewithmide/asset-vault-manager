use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("rDvvHvSgLxJMSrU2r22uDQkZnyN3PHPhy8t2hxvvhuC");

#[program]
pub mod asset_vault_manager {
    use super::*;

    pub fn initialize_vault(ctx: Context<InitializeVault>) -> Result<()> {
        let vault: &mut Account<Vault> = &mut ctx.accounts.vault;
        vault.authority = ctx.accounts.authority.key();
        vault.token_program = ctx.accounts.token_program.key();
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        let transfer_instruction: Transfer = Transfer {
            from: ctx.accounts.depositor_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.depositor.to_account_info(),
        };

        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
            ),
            amount,
        )?;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        let seeds = &[
            ctx.accounts.vault.to_account_info().key.as_ref(),
            &[ctx.accounts.vault.bump],
        ];
        let signer = &[&seeds[..]];

        let transfer_instruction = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.depositor_token_account.to_account_info(),
            authority: ctx.accounts.vault.to_account_info(),
        };

        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                transfer_instruction,
                signer,
            ),
            amount,
        )?;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeVault<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 32 +1)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    pub depositor: Signer<'info>,
    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut, has_one = authority)]
    pub vault: Account<'info, Vault>,
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub depositor_token_account: Account<'info, TokenAccount>,
    pub depositor: Signer<'info>,
    pub authority: AccountInfo<'info>,    
    pub token_program: Program<'info, Token>,
}


#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub token_program: Pubkey,
    pub bump: u8,
}