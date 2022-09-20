use solana_program::{ program_error::ProgramError, msg };
use std::convert::TryInto;

#[derive(Debug)]
pub enum CalculatorInstruction {
    Increament,
    Decreament,
    Set(u32),
}

impl CalculatorInstruction {
    //a function that takes a u8 array of instruction process the array
    //and returns Self (here enum type CalculatorInstruction) and ProgramError in case of any error
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        //split_first() returns an Option either a value or nothing
        //ok_or() converts an Option to Result type
        //in our case it will return Self if successful or ProgramError type if not
        //
        //now this destructuring will just separate the first index as we are using first index
        //to determine what operation to perform
        //the rest is stored in the rest slice which in case of Set() will contain the value to set
        let (&tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => {
                return Ok(CalculatorInstruction::Increament);
            }
            1 => {
                return Ok(CalculatorInstruction::Decreament);
            }
            2 => {
                if rest.len() != 4 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                //now the rest of the array is i the rest slice
                //slice does not have size at compiletime
                //now we need to convert the slice in to actual array
                //now the compiler does not know about the type of the value in this slice
                //so we need to specify the size and type of the array as well
                //try_into() fn will try to convert the data in the rest to an actual array
                let val: Result<[u8; 4], _> = rest[..4].try_into();

                //
                match val {
                    Ok(i) => {
                        return Ok(HelloInstruction::Set(u32::from_ke_bytes(i)));
                    }
                    _ => {
                        return Err(ProgramError::InvalidInstructionData);
                    }
                }
            }
        }
    }
}