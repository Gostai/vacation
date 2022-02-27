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
    pub date: u32,
    /// length of Vacation
    pub length: u32,
        
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct VacationNumberAccount {
    /// number of Vacations
    pub number: u32,
    /// year of Vacation
    pub year: u32,
        
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Date {
    /// year
    pub y: u32,
    /// month
    pub m: u32,
    /// day
    pub d: u32,
        
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    Ok(())
}
