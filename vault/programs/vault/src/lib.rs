use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("DW9t3fDXQbGfja56Y8p9uPmcA7Kj57XkejHJGShFvxr6");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn create_vault(ctx: Context<CreateVault>) -> Result<()> {
        create_vault::handler(ctx)
    }

    pub fn submit_lamports(ctx: Context<SubmitLamports>, transfer_amount: u64) -> Result<()> {
        submit_lamports::handler(ctx, transfer_amount)
    }

    pub fn withdraw_lamports(ctx : Context<WithdrawLamports>, withdraw_amount : u64) -> Result<()> {
        withdraw_lamports::handler(ctx, withdraw_amount)
    }

    pub fn close_vault(ctx : Context<CloseVault>) -> Result<()> {
        close_vault::handler(ctx)
    }
}
