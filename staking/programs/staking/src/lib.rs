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

    pub fn create_user(ctx: Context<CreateUser>) -> Result<()> {
        create_user::handler(ctx)
    }

    pub fn stake(ctx: Context<BaseStakeAccounts>, added_amount : u64) -> Result<()> {
        stake::handler(ctx, added_amount)
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        claim_rewards::handler(ctx)
    }

    pub fn unstake(ctx: Context<BaseStakeAccounts>, taken_amount : u64) -> Result<()> {
        unstake::handler(ctx, taken_amount)
    }
}
