use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct AdminDetails{
    pub admin : Pubkey,
    pub bump : u8
}

#[account]
#[derive(InitSpace)]
pub struct StakePool{
    pub total_pool: u64,
    pub reward_rate: u64,
    pub reward_per_token : u64,
    pub last_updated: i64,
    pub bump : u8
}


#[account]
#[derive(InitSpace)]
pub struct StakeDetails {
    pub staked_amount : u64,
    pub holding_rewards : u64,
    pub last_reward_per_token : u64,
    pub last_updated : i64,
    pub bump : u8
}