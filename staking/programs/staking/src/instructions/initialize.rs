use anchor_lang::prelude::*;

use crate::AdminDetails;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin : Signer<'info>,

    #[account(
        init,
        space=8+AdminDetails::INIT_SPACE,
        seeds=[b"admin", admin.key().as_ref()],
        payer=admin,
        bump
    )]
    pub admin_details : Account<'info, AdminDetails>,

    pub system_program : Program<'info, System>
}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    ctx.accounts.admin_details.admin = ctx.accounts.admin.key();
    ctx.accounts.admin_details.bump = ctx.bumps.admin_details;
    Ok(())
}
