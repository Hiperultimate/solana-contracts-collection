use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{StakeDetails, StakePool, error::ErrorCode};

#[derive(Accounts)]
pub struct Stake<'info>{
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

impl<'info> Stake<'info> {
    pub fn update_states(&mut self, added_amount : u64) -> Result<()>{
        let current_time = Clock::get()?.unix_timestamp;
        let elapsed_time = u64::try_from(current_time.checked_sub(self.stake_pool.last_updated).unwrap()).unwrap();

        // Securing division by 0
        if self.stake_pool.total_pool > 0 {
            self.stake_pool.reward_per_token += (self.stake_pool.reward_rate * elapsed_time).checked_div(self.stake_pool.total_pool).unwrap();
        }
        self.stake_pool.last_updated = current_time;
        
        self.stake_details.holding_rewards += (self.stake_pool.reward_per_token - self.stake_details.last_reward_per_token) * self.stake_details.staked_amount; 
        self.stake_details.last_reward_per_token = self.stake_pool.reward_per_token;

        self.stake_pool.total_pool += added_amount;

        // update stake_details
        self.stake_details.staked_amount += added_amount;
        self.stake_details.last_updated = current_time;
        Ok(())
    }

    pub fn stake_funds(&mut self, amount_added : u64) -> Result<()>{
        // transfer from user_ata to treasury_ata
        require!( amount_added == 0, ErrorCode::InvalidTokenBalance);
        let accounts = TransferChecked {
            authority : self.user.to_account_info(),
            from: self.user_ata.to_account_info(),
            to: self.treasury_ata.to_account_info(),
            mint: self.mint.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(cpi_context, amount_added, self.mint.decimals)
    }
}

pub fn handler(ctx: Context<Stake>, added_amount : u64, amount_added : u64) -> Result<()> {

    ctx.accounts.update_states(added_amount)?;
    ctx.accounts.stake_funds(amount_added)?;

    Ok(())
}