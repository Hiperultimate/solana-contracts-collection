use anchor_lang::prelude::*;
use anchor_spl::token_2022::{transfer_checked, TransferChecked};

use crate::BaseStakeAccounts;
use crate::error::ErrorCode;

impl<'info> BaseStakeAccounts<'info>{
    pub fn unstake(&mut self, taken_amount :u64) -> Result<()>{
        require!(self.stake_details.staked_amount >= taken_amount, ErrorCode::InvalidUnstake);
        let current_time = Clock::get()?.unix_timestamp;

        self.update_stake_pool(current_time)?;
        self.update_user_stake(current_time)?;

        // update stake_details
        self.stake_pool.total_pool -= taken_amount;
        self.stake_details.staked_amount -= taken_amount;
        Ok(())
    }

    pub fn transfer_unstaked_token(&mut self, taken_amount : u64) -> Result<()>{
        // transfer from treasury_ata to user_ata
        let mint_key = self.mint.key();
        let signer_seeds: &[&[&[u8]]] = &[&[b"stake_pool", mint_key.as_ref() ,&[self.stake_pool.bump]]];
        let accounts = TransferChecked {
            mint: self.mint.to_account_info(),
            authority : self.stake_pool.to_account_info(),
            from: self.treasury_ata.to_account_info(),
            to: self.user_ata.to_account_info(),
        };
        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);
        transfer_checked(cpi_context, taken_amount, self.mint.decimals)?;
        Ok(())
    }
}


pub fn handler(ctx : Context<BaseStakeAccounts>, taken_amount : u64) -> Result<()> {
    ctx.accounts.unstake(taken_amount)?;
    ctx.accounts.transfer_unstaked_token(taken_amount)?;
    Ok(())
}
