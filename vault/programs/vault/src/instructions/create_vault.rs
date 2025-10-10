use anchor_lang::prelude::*;

use crate::Vault;

#[derive(Accounts)]
pub struct CreateVault<'info>{
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        init,
        space=8 + Vault::INIT_SPACE,
        payer=signer,
        seeds=[b"vault", signer.key().as_ref()],
        bump
    )]
    pub vault : Account<'info, Vault>,

    pub system_program : Program<'info, System>,

}

pub fn handler(ctx : Context<CreateVault>) -> Result<()> {
    ctx.accounts.vault.vault_bump = ctx.bumps.vault;
    Ok(())
}