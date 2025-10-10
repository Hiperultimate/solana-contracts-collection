use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Vault { 
    pub vault_bump : u8
}