pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5FxoriHCMZQkTV6UsJh9kKHEuvipeK6pe1DTAGzp436F");

#[program]
pub mod staking {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn initialize_pool(ctx: Context<InitializePool>, reward_rate: u64, transfer_amount: u64 ) -> Result<()> {
        initialize_pool::handler(ctx, reward_rate, transfer_amount)
    }
}
