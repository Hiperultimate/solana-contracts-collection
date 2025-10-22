use anchor_lang::prelude::*;
use anchor_spl::{token_interface::{Mint, TokenAccount, TokenInterface}};
use crate::{StakeDetails, StakePool};

#[derive(Accounts)]
pub struct BaseStakeAccounts<'info>{
    #[account(mut)]
    pub user : Signer<'info>,

    #[account(
        associated_token::mint=mint,
        associated_token::authority=user,
        associated_token::token_program=token_program,
    )]
    pub user_ata : InterfaceAccount<'info, TokenAccount>,

    #[account(
        associated_token::mint=mint,
        associated_token::authority=stake_pool,
        associated_token::token_program=token_program
    )]
    pub treasury_ata : InterfaceAccount<'info, TokenAccount>,

    #[account(
        seeds=[b"stake_pool", mint.key().as_ref()],
        bump=stake_pool.bump
    )]
    pub stake_pool : Account<'info, StakePool>,

    #[account(
        seeds=[b"stake_details", user.key().as_ref()],
        bump
    )]
    pub stake_details : Account<'info, StakeDetails>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub token_program: Interface<'info, TokenInterface>
}

impl<'info> BaseStakeAccounts<'info>{
    pub fn update_stake_pool(&mut self, current_time : i64) -> Result<()>{
        let elapsed_time = u64::try_from(current_time.checked_sub(self.stake_pool.last_updated).unwrap()).unwrap();

        // Securing division by 0
        if self.stake_pool.total_pool > 0 {
            self.stake_pool.reward_per_token += (self.stake_pool.reward_rate * elapsed_time).checked_div(self.stake_pool.total_pool).unwrap();
        }
        self.stake_pool.last_updated = current_time;
        Ok(())
    }

    pub fn update_user_stake(&mut self, current_time : i64) -> Result<()>{
        self.stake_details.holding_rewards += (self.stake_pool.reward_per_token - self.stake_details.last_reward_per_token) * self.stake_details.staked_amount; 
        self.stake_details.last_reward_per_token = self.stake_pool.reward_per_token;
        self.stake_details.last_updated = current_time;
        Ok(())
    }
}