use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Vault does not have requested amount of lamports")]
    InvalidLamportsRequested
}