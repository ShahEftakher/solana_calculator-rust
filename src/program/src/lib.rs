use borsh::{ BorshDeserialize, BorshSerialize };
use solana_program::{
    account_info::{ next_account_info, AccountInfo },
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

pub mod instruction;
use crate::instruction::CalculatorInstruction;

/// Declaraing PDA structure
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CalculatorAccount {
    pub result: u32,
}

// the program's entrypoint
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;
    let instruction = CalculatorInstruction::unpack(instruction_data)?;

    if account.owner != program_id {
        msg!("Account is not the owner of PDA");
        return Err(ProgramError::IncorrectProgramId);
    }

    //get the instance of the Calculator account PDA
    let mut calculator_account = CalculatorAccount::try_from_slice(&account.data.borrow())?;
    //match the instruction to find out what to do
    match instruction {
        CalculatorInstruction::Increment => {
            calculator_account.result += 1;
        }
        CalculatorInstruction::Decrement => {
            calculator_account.result -= 1;
        }
        CalculatorInstruction::Set(val) => {
            calculator_account.result = val;
        }
    }

    calculator_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    msg!("Result: {}", calculator_account.result);
    Ok(())
}