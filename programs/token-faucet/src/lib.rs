use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint, Transfer};


declare_id!("DyTqqVKtYEzJAdJhT2aDcVuTD7JFkRWjBDfeXTCZWPUe");

#[program]
pub mod token_faucet {
    use super::*;

    pub fn initialize_faucet(
        ctx: Context<InitializeFaucet>,
        drip_amount: u64,
        cooldown_seconds: u64,
    ) -> Result<()> {

        let faucet = &mut ctx.accounts.faucet;

        faucet.admin = ctx.accounts.user.key();
        faucet.mint = ctx.accounts.user_token_mint.key();
        faucet.drip_amount = drip_amount;
        faucet.cooldown_seconds = cooldown_seconds;
        faucet.bump = ctx.bumps.faucet;

        Ok(())
    }

    pub fn fund_faucet(ctx:Context<FundFaucet>,amount:u64) -> Result<()> {

        let faucet=&mut ctx.accounts.faucet;

        let cpi_account=Transfer{
            from:ctx.accounts.user_token_ata.to_account_info(),
            to:ctx.accounts.vault_faucet.to_account_info(),
            authority:ctx.accounts.user.to_account_info()
        };

        let cpi_program=CpiContext::new(
            ctx.accounts.token_program.to_account_info(), cpi_account);

        token::transfer(cpi_program, amount)?;

        Ok(())
    }

    pub fn drip_faucet(ctx:Context<DripFaucet>) -> Result<()> {

        let faucet=&ctx.accounts.faucet;

        let seeds: &[&[u8]] = &[
            b"faucet_vault",
            faucet.mint.as_ref(),
            &[faucet.bump],
        ];

        let signer_seeds: &[&[&[u8]]] = &[seeds];

        let cpi_account=Transfer{
            from:ctx.accounts.vault_faucet.to_account_info(),
            to:ctx.accounts.request_user_token_ata.to_account_info(),
            authority:ctx.accounts.faucet.to_account_info()
        };

        let cpi_ctx=CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            cpi_account,
            signer_seeds,
        );

        token::transfer(cpi_ctx, faucet.drip_amount)?;

        Ok(())
    }

}

#[derive(Accounts)]
pub struct DripFaucet<'info> {
    #[account(mut)]
    pub request_user:Signer<'info>,

    #[account(mut)]
    pub request_user_token_ata:Account<'info,TokenAccount>,

    #[account(mut)]
    pub vault_faucet:Account<'info,TokenAccount>,

    pub faucet:Account<'info,Faucet>,

    pub token_program:Program<'info,Token>,

}

#[derive(Accounts)]
pub struct FundFaucet<'info> {

    #[account(mut)]
    pub user:Signer<'info>,

    #[account(mut)]
    pub user_token_ata:Account<'info,TokenAccount>,

    #[account(mut)]
    pub vault_faucet:Account<'info,TokenAccount>,

    pub faucet: Account<'info, Faucet>,

    pub token_program: Program<'info, Token>,

}

#[derive(Accounts)]
pub struct InitializeFaucet<'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    pub user_token_mint: Account<'info, Mint>,

    #[account(mut)]
    pub user_token_ata: Account<'info, TokenAccount>,

    // PDA metadata
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 32 + 8 + 8 + 1,
        seeds = [b"faucet_metadata", user_token_mint.key().as_ref()],
        bump
    )]
    pub faucet: Account<'info, Faucet>,

    // Vault
    #[account(
        init,
        payer = user,
        seeds = [b"faucet_vault", user_token_mint.key().as_ref()],
        bump,
        token::mint = user_token_mint,
        token::authority = faucet,
    )]
    pub vault_faucet: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[account]
pub struct Faucet {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub drip_amount: u64,
    pub cooldown_seconds: u64,
    pub bump: u8,
}
