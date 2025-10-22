use anchor_lang::prelude::*;
use anchor_spl::{token_2022::{transfer_checked, TransferChecked}};

use crate::{error::ErrorCode, BaseStakeAccounts};

impl<'info> BaseStakeAccounts<'info> {
    pub fn add_stake(&mut self, added_amount : u64) -> Result<()>{
        let current_time = Clock::get()?.unix_timestamp;

        self.update_stake_pool(current_time)?;
        self.update_user_stake(current_time)?;

        // update stake_details
        self.stake_pool.total_pool += added_amount;
        self.stake_details.staked_amount += added_amount;
        Ok(())
    }

    pub fn stake_funds(&mut self, amount_added : u64) -> Result<()>{
        // transfer from user_ata to treasury_ata
        require!( amount_added > 0, ErrorCode::InvalidTokenBalance);
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

pub fn handler(ctx: Context<BaseStakeAccounts>, added_amount : u64, amount_added : u64) -> Result<()> {

    ctx.accounts.add_stake(added_amount)?;
    ctx.accounts.stake_funds(amount_added)?;

    Ok(())
}