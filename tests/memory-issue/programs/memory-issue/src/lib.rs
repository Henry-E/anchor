use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod memory_issue {
    use super::*;
    pub fn initialize_struct(ctx: Context<InitStructs>) -> ProgramResult {
        Ok(())
    }
    pub fn initialize_tokens(ctx: Context<InitializeTokens>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitStructs<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(init,
        seeds = [b"pubkey_account".as_ref()],
        bump,
        payer = authority)]
    pub pubkey_account: Box<Account<'info, PubkeyAccount>>,
    #[account(init,
        seeds = [b"another_pubkey_account".as_ref()],
        bump,
        payer = authority)]
    pub another_pubkey_account: Box<Account<'info, PubkeyAccount>>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitializeTokens<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(seeds = [b"pubkey_account".as_ref()],
        bump)]
    pub pubkey_account: ProgramAccount<'info, PubkeyAccount>,
    pub another_pubkey_account: AccountInfo<'info>,
    #[account(init,
        seeds = [b"open_orders".as_ref()],
        bump,
        space = 3228,
        payer = authority,
        owner = token_program.key())]
    pub open_orders: AccountInfo<'info>,
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

#[account]
#[derive(Default)]
pub struct PubkeyAccount {
    pub pubkey_1: [Pubkey; 20],
    pub pubkey_2: [Pubkey; 20],
    // pub pubkey_3: [Pubkey; 20],
}
