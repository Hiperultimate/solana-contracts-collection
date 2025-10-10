use anchor_lang::{prelude::*};
use crate::{error::ErrorCode, Vault};

#[derive(Accounts)]
pub struct WithdrawLamports<'info> {
    #[account(mut)]
    pub signer : Signer<'info>,

    /// CHECK: Vault that will store lamports
    #[account(
        mut,
        seeds=[b"vault", signer.key().as_ref()],
        bump=vault.vault_bump
    )]
    pub vault : Account<'info, Vault>,

    pub system_program : Program<'info, System>
}

pub fn handler(ctx : Context<WithdrawLamports>, amount_to_withdraw: u64) -> Result<()>{
    let vault_balance = ctx.accounts.vault.get_lamports();
    require!(amount_to_withdraw < vault_balance, ErrorCode::InvalidLamportsRequested);

    // Method 1 - Not safe
    // **ctx.accounts.vault.lamports.borrow_mut() -= amount_to_withdraw;
    // **ctx.accounts.signer.lamports.borrow_mut() += amount_to_withdraw;

    // Method 2 - Better but still unsafe
    // **ctx.accounts.vault.try_borrow_mut_lamports()? -= amount_to_withdraw;
    // **ctx.accounts.signer.try_borrow_mut_lamports()? += amount_to_withdraw;

    // Method 3 - Best of them all
    ctx.accounts.vault.sub_lamports(amount_to_withdraw)?;
    ctx.accounts.signer.add_lamports(amount_to_withdraw)?;

    Ok(())
}