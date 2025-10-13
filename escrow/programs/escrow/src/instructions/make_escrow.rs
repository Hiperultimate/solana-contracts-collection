use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};
use anchor_lang::prelude::*;

use crate::{EscrowDetails};

#[derive(Accounts)]
#[instruction( seed : u64 )]
pub struct MakeEscrow<'info>{
    #[account(mut)]
    pub signer : Signer<'info>, // The maker
    
    #[account(
        init_if_needed,
        associated_token::mint=token_mint_a,
        associated_token::authority=signer,
        associated_token::token_program=token_program,
        payer=signer,
    )]
    pub maker_ata_a : InterfaceAccount<'info, TokenAccount>, // users account

    #[account(
        init_if_needed,
        associated_token::mint=token_mint_b,
        associated_token::authority=signer,
        associated_token::token_program=token_program,
        payer=signer,
    )]
    pub maker_ata_b : InterfaceAccount<'info, TokenAccount>,

    // This should be a deterministic account
    #[account(
        init,
        associated_token::mint=token_mint_a,
        associated_token::authority=escrow_details,
        associated_token::token_program=token_program,
        payer=signer,
    )]
    pub vault_ata_a : InterfaceAccount<'info, TokenAccount>,

    pub token_mint_a : InterfaceAccount<'info, Mint>,
    pub token_mint_b : InterfaceAccount<'info, Mint>,

    // add the unique seed for this escrow_details
    #[account(
        init,
        // seeds=[b"escrow", signer.key().as_ref(), seed.to_le_bytes().as_ref()],   // Causes of all the issues
        seeds=[b"escrow", signer.key().as_ref()],
        payer=signer,
        space=8+EscrowDetails::INIT_SPACE,
        bump
    )]
    pub escrow_details : Account<'info, EscrowDetails>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>, // we probably shouldnt need this
    pub system_program : Program<'info, System>
}

pub fn handler(ctx : Context<MakeEscrow>, token_a_amount : u64, token_b_amount : u64, seed: u64) -> Result<()> {
    let escrow_details = &mut ctx.accounts.escrow_details;
    escrow_details.escrow_bump=ctx.bumps.escrow_details;
    escrow_details.token_a_amount=token_a_amount;
    escrow_details.token_b_amount=token_b_amount;
    escrow_details.token_mint_a=ctx.accounts.token_mint_a.key();
    escrow_details.token_mint_b=ctx.accounts.token_mint_b.key();
    escrow_details.maker_address=ctx.accounts.signer.key();
    escrow_details.taker_address=Pubkey::default(); // Giving it a default holder value

    // Write transfer token logic from user ata to vault ata
    let cpi_accounts = TransferChecked{
        from : ctx.accounts.maker_ata_a.to_account_info(),
        to: ctx.accounts.vault_ata_a.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
        mint: ctx.accounts.token_mint_a.to_account_info(),
    };

    let cpi_context = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        cpi_accounts
    );

    token_interface::transfer_checked(cpi_context, token_a_amount, ctx.accounts.token_mint_a.decimals)?;

    Ok(())
}