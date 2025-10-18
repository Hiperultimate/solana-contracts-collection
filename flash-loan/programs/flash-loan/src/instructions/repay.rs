use anchor_lang::{prelude::*, solana_program::sysvar::instructions::load_instruction_at_checked};
use anchor_spl::token::{transfer, Transfer};

use crate::{error::{ErrorCode}, Loan, ID, instruction};

impl<'a> Loan<'a>{
    pub fn repay(&mut self) -> Result<()>{
        let instructions = self.instruction.to_account_info();

        // in order to check that we need to check if borrow instruction is being run from our program or not
        let borrow_instruction = load_instruction_at_checked(0, &instructions)?;
        require_keys_eq!(borrow_instruction.program_id, ID, ErrorCode::InvalidProgramInstruction);

        // so we need to check if the very first instruciton is borrow or not
        require!(borrow_instruction.data[0..8].eq(instruction::Borrow::DISCRIMINATOR), ErrorCode::InvalidInstructions); 

        let borrowed_amount = u64::from_le_bytes(borrow_instruction.data[9..17].try_into().unwrap());

        // We can add fee to borrowed_amount later

        // get the borrowed amount back
        let accounts = Transfer {
            from: self.user_ata.to_account_info(),
            to: self.treasury_ata.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), accounts);
        transfer(cpi_context, borrowed_amount)
    }
}

pub fn handler(ctx : Context<Loan>) -> Result<()>{
    ctx.accounts.repay()?;

    Ok(())
}