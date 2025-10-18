use anchor_lang::{prelude::*, solana_program::sysvar::{instructions::{load_current_index_checked, load_instruction_at_checked}}};
use anchor_spl::token::{transfer, Transfer};
use crate::{instruction, ID};
use crate::{error::ErrorCode, Loan};



impl<'a> Loan<'a> {
    pub fn borrow(&mut self, treasury_bump:u8, amount : u64) -> Result<()> {
        // add the checks
        // need to add checks for the accounts being used as well
        let instruction_account = self.instruction.to_account_info();
        let instruction_sysvar = instruction_account.try_borrow_data()?;
        let instruction_len = u16::from_le_bytes(instruction_sysvar[0..2].try_into().unwrap());
        let current_index = load_current_index_checked(&self.instruction.to_account_info())?;
        
        // First instruction should be borrow
        require_eq!(current_index, 0 , ErrorCode::InvalidInstructions);

        // check if repay instruction is at the end
        let last_instruction = load_instruction_at_checked(instruction_len as usize - 1, &instruction_account)?;
        
        // Checking the currently checked instruction belongs to the last_instruction
        require_keys_eq!(last_instruction.program_id, ID, ErrorCode::InvalidProgramInstruction );

        // Checking if the instruction belongs to the correct program
        require!(last_instruction.data[0..8].eq(instruction::Repay::DISCRIMINATOR), ErrorCode::InvalidProgramInstruction);

        // user_ata
        require_keys_eq!(last_instruction.accounts.get(3).ok_or(ErrorCode::InvalidAccount)?.pubkey, self.user_ata.key(), ErrorCode::InvalidBorrowerAta);

        // treasury_ata
        require_keys_eq!(last_instruction.accounts.get(4).ok_or(ErrorCode::InvalidAccount)?.pubkey, self.treasury_ata.key(), ErrorCode::InvalidBorrowerAta);

        // add borrow functionality
        // transfer from trasury_ata to user_ata
        let accounts = Transfer {
            from: self.treasury_ata.to_account_info(),
            to: self.user_ata.to_account_info(),
            authority: self.treasury.to_account_info(),
        };
        let signer_seeds: &[&[&[u8]]] = &[&[ b"treasury", &[treasury_bump] ]];
        let cpi_context = CpiContext::new_with_signer(self.token_program.to_account_info(), accounts, signer_seeds);
        transfer(cpi_context, amount)
    }
}

pub fn handler(ctx : Context<Loan>, amount : u64) -> Result<()>{
    ctx.accounts.borrow(ctx.bumps.treasury, amount)?;
    Ok(())
}