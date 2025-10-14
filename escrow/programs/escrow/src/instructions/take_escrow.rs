use anchor_lang::{prelude::*};
use crate::{EscrowDetails};
use anchor_spl::{associated_token::AssociatedToken, token_2022::TransferChecked, token_interface::{ self, Mint, TokenAccount, TokenInterface}};


#[derive(Accounts)]
#[instruction(seed : u64)]
pub struct TakeEscrow<'info> {
    #[account(
        mut,
    )]
    pub signer : Signer<'info>,

    /// CHECK : as the name suggests, its escrow owner and we are just using it to check
    #[account(
        constraint= escrow_details.maker_address == escrow_owner.key()
    )]
    pub escrow_owner: AccountInfo<'info>,

    #[account(
        init_if_needed,
        associated_token::mint=token_mint_b,
        associated_token::authority=signer,
        associated_token::token_program=token_program,
        payer=signer,
    )]
    pub taker_ata_b : InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        associated_token::mint=token_mint_a,
        associated_token::authority=signer,
        associated_token::token_program=token_program,
        payer=signer,
    )]
    pub taker_ata_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_mint_a,
        associated_token::authority=escrow_details,
        associated_token::token_program=token_program,
    )]
    pub vault_ata_a : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint=token_mint_a,
        associated_token::authority=escrow_owner,
        associated_token::token_program=token_program,
    )]
    pub maker_ata_b : InterfaceAccount<'info, TokenAccount>,

    #[account(
        constraint=token_mint_a.key() == escrow_details.mint_a
    )]
    pub token_mint_a : InterfaceAccount<'info, Mint>,

    #[account(
        constraint=token_mint_b.key() == escrow_details.mint_b
    )]
    pub token_mint_b : InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        // seeds=[b"escrow", escrow_owner.key().as_ref(), seed.to_le_bytes().as_ref()],
        seeds=[b"escrow", escrow_owner.key().as_ref()],
        bump=escrow_details.escrow_bump
    )]
    pub escrow_details : Account<'info, EscrowDetails>,

    pub token_program : Interface<'info, TokenInterface>,
    pub associated_token_program : Program<'info, AssociatedToken>,
    pub system_program : Program<'info, System>
}

pub fn handler(ctx : Context<TakeEscrow>, seed : u64) -> Result<()> {
    // Transfer token from taker_ata_b to maker_ata_b
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.taker_ata_b.to_account_info(),
        to: ctx.accounts.maker_ata_b.to_account_info(),
        authority: ctx.accounts.signer.to_account_info(),
        mint: ctx.accounts.token_mint_b.to_account_info(),
    };

    let cpi_context = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token_interface::transfer_checked(cpi_context, ctx.accounts.escrow_details.token_b_amount, ctx.accounts.token_mint_b.decimals)?;


    // Transfer token from vault_ata_a to taker_ata_a 
    let cpi_accounts = TransferChecked {
        from: ctx.accounts.vault_ata_a.to_account_info(),
        to: ctx.accounts.taker_ata_a.to_account_info(),
        authority: ctx.accounts.escrow_details.to_account_info(),   // This is wrong
        mint: ctx.accounts.token_mint_a.to_account_info(),
    };

    let escrow_owner_key = ctx.accounts.escrow_owner.key();
    let seed_bytes = seed.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[&[b"escrow", escrow_owner_key.as_ref(), seed_bytes.as_ref(), &[ctx.accounts.escrow_details.escrow_bump]]];

    let cpi_context = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer_seeds);
    token_interface::transfer_checked(cpi_context, ctx.accounts.escrow_details.token_b_amount, ctx.accounts.token_mint_b.decimals)?;


    Ok(())
}