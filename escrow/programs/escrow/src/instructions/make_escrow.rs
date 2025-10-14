use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token;
use anchor_spl::token_2022::TransferChecked;
use anchor_spl::token_interface::{self, Mint, TokenAccount, TokenInterface};
use anchor_lang::prelude::*;

use crate::program::Escrow;
use crate::{EscrowDetails};

#[derive(Accounts)]
#[instruction( seed : u64 )]
pub struct MakeEscrow<'info>{
    #[account(mut)]
    pub maker : Signer<'info>, // The maker

    // add the unique seed for this escrow_details
    #[account(
        init,
        // seeds=[b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],   // Causes of all the issues
        seeds=[b"escrow", maker.key().as_ref()],
        payer=maker,
        space=8+EscrowDetails::INIT_SPACE,
        bump
    )]
    pub escrow : Account<'info, EscrowDetails>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_a : InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub mint_b : InterfaceAccount<'info, Mint>,
    
    #[account(
        init_if_needed,
        associated_token::mint=mint_a,
        associated_token::authority=maker,
        associated_token::token_program=token_program,
        payer=maker,
    )]
    pub maker_ata_a : InterfaceAccount<'info, TokenAccount>, // users account

    // #[account(
    //     init_if_needed,
    //     associated_token::mint=mint_b,
    //     associated_token::authority=maker,
    //     associated_token::token_program=token_program,
    //     payer=maker,
    // )]
    // pub maker_ata_b : InterfaceAccount<'info, TokenAccount>,

    // This should be a deterministic account
    #[account(
        init,
        associated_token::mint=mint_a,
        associated_token::authority=escrow,
        associated_token::token_program=token_program,
        payer=maker,
    )]
    pub vault_ata_a : InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>, // we probably shouldnt need this
    pub system_program : Program<'info, System>
}

impl<'info> MakeEscrow<'info> {
    fn populate_escrow(&mut self, seed : u64, token_a_amount : u64, token_b_amount : u64, escrow_bump : u8) -> Result<()>{
        self.escrow.set_inner(EscrowDetails { 
            seed ,
            maker_address: self.maker.key(),
            mint_a: self.mint_a.key(),
            mint_b: self.mint_b.key(),
            token_a_amount,
            token_b_amount,
            escrow_bump 
        });
        Ok(())
    }

    fn deposit_tokens(&mut self, token_a_amount : u64 ) -> Result<()> {
        let cpi_accounts = TransferChecked{
            from : self.maker_ata_a.to_account_info(),
            to: self.vault_ata_a.to_account_info(),
            authority: self.maker.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };

        let cpi_context = CpiContext::new(
            self.token_program.to_account_info(),
            cpi_accounts
        );

        token_interface::transfer_checked(cpi_context, token_a_amount, self.mint_a.decimals)?;

        Ok(())
    }
}

pub fn handler(ctx : Context<MakeEscrow>, token_a_amount : u64, token_b_amount : u64, seed: u64) -> Result<()> {
    ctx.accounts.populate_escrow(seed, token_a_amount, token_b_amount, ctx.bumps.escrow)?;
    ctx.accounts.deposit_tokens(token_a_amount)?;

    Ok(())
}