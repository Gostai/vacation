use crate::instruction::VacationInstruction;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};


/// Define the vacation type stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VacationAccount {
    /// date of starting of Vacation
    pub date: Date,
    /// length of Vacation
    pub length: u8,
        
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VacationNumberAccount {
    /// number of Vacations
    pub number: u8,
    /// year of Vacation
    pub year: u16,
        
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Date {
    /// year
    pub y: u16,
    /// month
    pub m: u8,
    /// day
    pub d: u8,
        
}

pub struct Processor {}
impl Processor {
    
    /// Processes an [Instruction](enum.Instruction.html).
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = VacationInstruction::unpack(input)?;
        Ok(())
    }

}

pub fn process_instruction_for_change(
    program_id: &Pubkey, // Public key of the account the choose vacation program was loaded into
    accounts: &[AccountInfo], // The account to save vacation to
    instruction_data: &[u8], // Vacation data
    
) -> ProgramResult {
    msg!("Choose Vacation program entrypoint");
    
    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();
    
    // Get the account to save vacation 
    let account = next_account_info(accounts_iter)?;
    
    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Vacation account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }
    
    let mut vacation_account = VacationAccount::try_from_slice(&account.data.borrow())?;
    //vacation_account.date = vacation_date;
    //vacation_account.length = vacation_length;
    vacation_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    
    msg!("Date {:?} length {}",vacation_account.date, vacation_account.length);
    
    Ok(())
}
