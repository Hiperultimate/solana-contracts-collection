use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You do not have the balance to make the transaction")]
    InvalidTokenBalance,
}
