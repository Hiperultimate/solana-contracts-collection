
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowDetails{
    pub seed: u64,
    pub maker_address : Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub token_a_amount : u64,
    pub token_b_amount : u64,
    pub escrow_bump : u8
}