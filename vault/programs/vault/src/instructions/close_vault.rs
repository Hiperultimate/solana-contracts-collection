use anchor_lang::prelude::*;

use crate::Vault;

#[derive(Accounts)]
pub struct CloseVault<'info> {
    
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        mut,
        close = signer,
        seeds=[b"vault", signer.key().as_ref()],
        bump = vault.vault_bump
    )]
    pub vault : Account<'info, Vault>
}

// return all the remaining lamports in vault 
pub fn handler(ctx: Context<CloseVault>) -> Result<()> {

    let remaining_lamports = ctx.accounts.vault.get_lamports();
    ctx.accounts.vault.sub_lamports(remaining_lamports)?;
    ctx.accounts.signer.add_lamports(remaining_lamports)?;

    Ok(())
}
