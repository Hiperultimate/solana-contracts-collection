use anchor_lang::{prelude::*};
use anchor_spl::{ token_2022::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{StakeDetails, StakePool};

#[derive(Accounts)]
pub struct ClaimRewards<'info>{
    #[account(mut)]
    pub user : Signer<'info>,
    
    #[account(
        mint::token_program=token_program
    )]
    pub mint : InterfaceAccount<'info, Mint>,

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
        mut,
        seeds=[b"stake_details", user.key().as_ref(), mint.key().as_ref()],
        bump=stake_details.bump
    )]
    pub stake_details : Account<'info, StakeDetails>,

    pub token_program : Interface<'info, TokenInterface>
}

impl<'info> ClaimRewards<'info> {
    pub fn calculate_reward_and_update_state(&mut self) -> Result<u64> {
        let current_time = Clock::get()?.unix_timestamp;
        let elapsed_time = u64::try_from(current_time.checked_sub(self.stake_pool.last_updated).unwrap()).unwrap();

        if self.stake_pool.total_pool > 0 {
            self.stake_pool.reward_per_token += (elapsed_time * self.stake_pool.reward_rate) / self.stake_pool.total_pool;
        }
        self.stake_pool.last_updated = current_time;

        let user_reward = (self.stake_pool.reward_per_token - self.stake_details.last_reward_per_token) * self.stake_details.staked_amount;
        self.stake_details.last_updated = current_time;
        self.stake_details.holding_rewards = 0;
        self.stake_details.last_reward_per_token = self.stake_pool.reward_per_token;
        Ok(user_reward)
    }

    pub fn transfer_reward(&mut self, reward : u64) -> Result<()>{
        let signer_seeds: &[&[&[u8]]] = &[&[b"stake_pool", &[self.stake_pool.bump]]];
        let accounts = TransferChecked {
            mint: self.mint.to_account_info(),
            authority : self.stake_pool.to_account_info(),
            from: self.treasury_ata.to_account_info(),
            to: self.user_ata.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);
        transfer_checked(cpi_context, reward, self.mint.decimals)?;
        Ok(())
    }
}

pub fn handler(ctx : Context<ClaimRewards>) -> Result<()>{
    let reward = ctx.accounts.calculate_reward_and_update_state()?;
    ctx.accounts.transfer_reward(reward)?;
    Ok(())
}