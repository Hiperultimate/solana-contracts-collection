use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenInterface};

use crate::StakeDetails;

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        mint::token_program=token_program
    )]
    pub mint : InterfaceAccount<'info, Mint>,

    // store user data
    #[account(
        init, 
        payer=user,
        seeds=[b"stake_details", user.key().as_ref(), mint.key().as_ref()],
        space=8+StakeDetails::INIT_SPACE,
        bump
    )]
    pub stake_details : Account<'info, StakeDetails>,

    pub system_program : Program<'info,System>,
    pub token_program : Interface<'info, TokenInterface>
}

pub fn handler(ctx:Context<CreateUser>) -> Result<()>{
    ctx.accounts.stake_details.staked_amount = 0;
    ctx.accounts.stake_details.holding_rewards = 0;
    ctx.accounts.stake_details.last_updated = Clock::get()?.unix_timestamp;
    ctx.accounts.stake_details.last_reward_per_token = 0;
    ctx.accounts.stake_details.bump = ctx.bumps.stake_details;
    Ok(())
}