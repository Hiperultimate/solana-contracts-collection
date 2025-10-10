use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

// Nothing to do here
pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Vault initialization: {:?}", ctx.program_id);
    Ok(())
}
