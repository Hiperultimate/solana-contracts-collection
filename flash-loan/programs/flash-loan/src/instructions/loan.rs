use anchor_lang::{prelude::*, solana_program::sysvar::ID as INSTRUCTIONS_SYSVAR_ID};
use anchor_spl::token::{Mint, Token, TokenAccount}; 

#[derive(Accounts)]
pub struct Loan<'info> {

    #[account(mut)]
    pub user : Signer<'info>,
    
    pub mint : Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=user,
        associated_token::token_program=token_program
    )]
    pub user_ata : Account<'info, TokenAccount>,

    #[account(
        seeds=[b"treasury"],
        bump
    )]
    pub treasury : SystemAccount<'info>,

    #[account(
        mut,
        associated_token::mint=mint,
        associated_token::authority=treasury,
        associated_token::token_program=token_program
    )]
    pub treasury_ata : Account<'info, TokenAccount>,

    #[account(address = INSTRUCTIONS_SYSVAR_ID)]
    /// CHECK : InstructionSysvar account
    pub instruction : UncheckedAccount<'info>,
    pub token_program : Program<'info, Token>
}