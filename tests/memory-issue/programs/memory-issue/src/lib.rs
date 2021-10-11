use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod memory_issue {
    use super::*;
    pub fn initialize(ctx: Context<Initialize>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init,
        mint::decimals = 6,
        mint::authority = authority,
        seeds = [b"this_mint".as_ref()],
        bump,
        payer = authority)]
    pub this_mint: Box<Account<'info, Mint>>,
    #[account(init,
        token::mint = this_mint,
        token::authority = authority,
        seeds = [b"token_1".as_ref()],
        bump,
        payer = authority)]
    pub token_1: Box<Account<'info, TokenAccount>>,
    #[account(init,
        token::mint = this_mint,
        token::authority = authority,
        seeds = [b"token_2".as_ref()],
        bump,
        payer = authority)]
    pub token_2: Box<Account<'info, TokenAccount>>,
    #[account(init,
        token::mint = this_mint,
        token::authority = authority,
        seeds = [b"token_3".as_ref()],
        bump,
        payer = authority)]
    pub token_3: Box<Account<'info, TokenAccount>>,
    #[account(init,
        token::mint = this_mint,
        token::authority = authority,
        seeds = [b"token_4".as_ref()],
        bump,
        payer = authority)]
    pub token_4: Box<Account<'info, TokenAccount>>,
    #[account(init,
        token::mint = this_mint,
        token::authority = authority,
        seeds = [b"token_5".as_ref()],
        bump,
        payer = authority)]
    pub token_5: Box<Account<'info, TokenAccount>>,
    #[account(init,
        token::mint = this_mint,
        token::authority = authority,
        seeds = [b"token_6".as_ref()],
        bump,
        payer = authority)]
    pub token_6: Box<Account<'info, TokenAccount>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
