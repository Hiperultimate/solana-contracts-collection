use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Instruction does not follow the required pattern to copy")]
    InvalidInstructions,

    #[msg("Instruction does not belong the required program")]
    InvalidProgramInstruction,

    #[msg("Invalid program account passed")]
    InvalidAccount,

    #[msg("Invalid borrower ATA")]
    InvalidBorrowerAta,

}
