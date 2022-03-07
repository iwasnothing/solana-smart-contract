use std::{mem};
//use std::convert::TryInto;
use std::io::Write;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    log::{sol_log_params, sol_log_slice},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
const SIZE: usize = 512;
/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub _field1: [u8; SIZE],
}
fn fill_from_str(mut bytes: &mut [u8], s: &[u8]) {
    bytes.write(s).unwrap();
}
// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    _instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer than indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    msg!("before get  greeting_account");
    // Increment and store the number of times the account has been greeted
    program_id.log();
    msg!("Initial Account of size {}",mem::size_of::<GreetingAccount>());
    sol_log_params(accounts, _instruction_data);
    sol_log_slice(&account.data.borrow());
    msg!("formatted {}: {:?}", "debug messages", &account.data.borrow() );
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    //msg!("got greeting_account {:?}",greeting_account.counter);
    //greeting_account.counter += 1;
    //msg!("incremented greeting_account");
    msg!("got bytes {:?} end-string", greeting_account._field1);
    //let _x = "abcdefghi".to_string();
    //let _a = _x.as_bytes().try_into().expect("_field1 with incorrect length");
    //greeting_account._field1 = _a;
    let new_b = _instruction_data;
    let n = new_b.len();
    let mut bytes: [u8; SIZE] = [0; SIZE];
    if n >= SIZE {
        greeting_account._field1.clone_from_slice(&new_b[0..SIZE]);
    } else {
        fill_from_str(&mut bytes,new_b );
        greeting_account._field1.clone_from_slice(&bytes[0..SIZE]);
    } 
        
    msg!("set bytes {:?} end-string", greeting_account._field1);
    msg!("set string greeting_account");
    msg!("to vec {:?} end-string",greeting_account.try_to_vec().unwrap() );
    let encoded_str = greeting_account.try_to_vec().unwrap();
    let new_account = GreetingAccount::try_from_slice(&encoded_str).unwrap();
    msg!("deserialize vec {:?} end-string of size {}",new_account,mem::size_of::<GreetingAccount>());

    new_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    //msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );
        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        assert_eq!(
            GreetingAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
