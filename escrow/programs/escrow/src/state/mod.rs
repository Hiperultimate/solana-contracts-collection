
use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct EscrowDetails{
    pub maker_address : Pubkey,
    pub taker_address : Pubkey,
    pub token_a_amount : u64,
    pub token_b_amount : u64,
    pub token_mint_a: Pubkey,
    pub token_mint_b: Pubkey,
    pub escrow_bump : u8
}