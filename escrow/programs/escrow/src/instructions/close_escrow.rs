use anchor_lang::prelude::*;
use anchor_spl::{ token_2022::{CloseAccount, close_account, transfer_checked, TransferChecked}, token_interface::{ Mint, TokenAccount, TokenInterface}};

use crate::EscrowDetails;

#[derive(Accounts)]
#[instruction( seed : u64 )]
pub struct CloseEscrow<'info>{

    #[account(mut)]
    pub maker : Signer<'info>,

    #[account(
        constraint=mint_a.key() == escrow.mint_a
    )]
    pub mint_a : InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault_ata_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[b"escrow", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        close=maker,
        bump=escrow.escrow_bump
    )]
    pub escrow : Account<'info, EscrowDetails>,

    pub token_program : Interface<'info, TokenInterface>

}

impl<'info> CloseEscrow<'info>{
    pub fn return_existing_mint_amount(&mut self, seed : u64) -> Result<()>{
        // Write logic to return all the existing amount left in vault
        let existing_balance = self.vault_ata_a.amount;
        
        if existing_balance == 0 {
            return Ok(())
        }

        // Transfer leftover amount from vault_ata to maker_ata
        let maker_key = self.maker.key();
        let seed_bytes = seed.to_le_bytes();
        let escrow_seeds: &[&[&[u8]]] = &[&[ b"escrow" , maker_key.as_ref(), seed_bytes.as_ref(), &[self.escrow.escrow_bump]]];
        
        let accounts = TransferChecked {
            from : self.vault_ata_a.to_account_info(),
            to: self.maker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
            mint: self.mint_a.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, escrow_seeds);
        
        transfer_checked(cpi_context, existing_balance, self.mint_a.decimals)
    }

    pub fn close_vault_account(&mut self, seed : u64) -> Result<()> {
        // perform a CPI call to close the vault ata account and send the taken lamports to maker
        let maker_key = self.maker.key();
        let seed_bytes = seed.to_le_bytes();
        let escrow_seeds: &[&[&[u8]]] = &[&[ b"escrow" , maker_key.as_ref(), seed_bytes.as_ref(), &[self.escrow.escrow_bump]]];

        let accounts = CloseAccount {
            account : self.vault_ata_a.to_account_info(), 
            authority: self.escrow.to_account_info(),
            destination: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, escrow_seeds);
        
        close_account(cpi_context)
    }
}

pub fn handler(ctx : Context<CloseEscrow>, seed : u64) -> Result<()> {
    ctx.accounts.return_existing_mint_amount(seed)?;
    ctx.accounts.close_vault_account(seed)?;

    Ok(())
}