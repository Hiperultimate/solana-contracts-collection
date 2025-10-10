use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};

use crate::Vault;


#[derive(Accounts)]
pub struct SubmitLamports<'info> {
    #[account(mut)]
    pub signer : Signer<'info>,

    #[account(
        mut,
        seeds=[b"vault", signer.key().as_ref()],
        bump=vault.vault_bump
    )]
    pub vault : Account<'info, Vault>,

    pub system_program : Program<'info, System>

}

pub fn handler(ctx: Context<SubmitLamports>, transfer_amount : u64) -> Result<()>{
    let transfer_cpi = Transfer {
        from : ctx.accounts.signer.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };

    let cpi = CpiContext::new(ctx.accounts.system_program.to_account_info(), transfer_cpi);
    transfer(cpi, transfer_amount)?;


    Ok(())
}