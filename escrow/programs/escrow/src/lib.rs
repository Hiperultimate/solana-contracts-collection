pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("27ay29uhZBUhrbc5HwnELYqB3tgkw7BnYb8dpD9RYzxE");

#[program]
pub mod escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn make_escrow(ctx : Context<MakeEscrow>, token_a_amount : u64, token_b_amount : u64, seed: u64) -> Result<()> {
        make_escrow::handler(ctx, token_a_amount, token_b_amount , seed)
    }

    pub fn take_escrow(ctx : Context<TakeEscrow>, seed: u64) -> Result<()> {
        take_escrow::handler(ctx, seed)
    }

    pub fn close_escrow(ctx : Context<CloseEscrow>, seed : u64) -> Result<()>{
        close_escrow::handler(ctx, seed)
    }


}
