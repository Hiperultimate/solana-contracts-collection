use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_2022::{transfer_checked, TransferChecked}, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{AdminDetails, StakePool};

#[derive(Accounts)]
pub struct InitializePool<'info>{
    #[account(
        mut,
        constraint=admin_details.admin == admin.key()
    )]
    pub admin : Signer<'info>,

    #[account(
        associated_token::mint=token_mint,
        associated_token::authority=admin,
        associated_token::token_program=token_program,
    )]
    pub admin_ata : InterfaceAccount<'info, TokenAccount>,

    #[account(
        init,
        payer=admin,
        associated_token::mint=token_mint,
        associated_token::authority=stake_pool,
        associated_token::token_program=token_program,
    )]
    pub treasury_ata : InterfaceAccount<'info, TokenAccount>,

    #[account(
        mint::token_program=token_program
    )]
    pub token_mint : InterfaceAccount<'info, Mint>,

    #[account(
        init,
        seeds=[b"stake_pool", admin.key().as_ref(), token_mint.key().as_ref()],
        space=8+StakePool::INIT_SPACE,
        payer=admin,
        bump
    )]
    pub stake_pool : Account<'info, StakePool>,

    #[account(
        seeds=[b"admin", admin.key().as_ref()],
        bump=admin_details.bump
    )]
    pub admin_details : Account<'info, AdminDetails>,

    pub token_program : Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program : Program<'info, AssociatedToken>
}

impl<'info> InitializePool<'info>{
    pub fn initialize(&mut self, reward_rate : u64) -> Result<()>{
        self.stake_pool.total_pool = 0;
        self.stake_pool.reward_rate = reward_rate;
        self.stake_pool.reward_per_token = 0;
        self.stake_pool.last_updated = Clock::get()?.unix_timestamp;
        Ok(())
    }

    pub fn transfer_to_treasury(&mut self, transfer_amount : u64) -> Result<()>{
        // transfer mint from admin_ata to treasury_ata ( no seed required )
        let accounts = TransferChecked {
            authority : self.admin.to_account_info(),
            from: self.admin_ata.to_account_info(),
            to: self.treasury_ata.to_account_info(),
            mint: self.token_mint.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), accounts);

        transfer_checked(cpi_context, transfer_amount, self.token_mint.decimals)
    }
}

pub fn handler(ctx: Context<InitializePool>, reward_rate : u64, transfer_amount : u64 ) -> Result<()> {
    ctx.accounts.initialize(reward_rate)?;
    ctx.accounts.transfer_to_treasury(transfer_amount)?;
    
    Ok(())
}