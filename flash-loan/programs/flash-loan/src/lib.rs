pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("9tTCXoatHRtam6hus1cWUsYsyp4CnyehQ1uAmv8NTYk");

#[program]
pub mod flash_loan {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn borrow(ctx: Context<Loan>, amount : u64) -> Result<()> {
        borrow::handler(ctx, amount)
    }

    pub fn repay(ctx: Context<Loan>) -> Result<()> {
        repay::handler(ctx)
    }
}
