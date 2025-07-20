use solana_program:: {
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint:: ProgramResult,
    pubkey::Pubkey,
    msg,
};
use borsh::{BorshDeserialize, BorshSerialize};


#[derive(BorshDeserialize, BorshSerialize)]
enum InstructionData {
    Increment(u32),
    Decrement(u32), 
}

#[derive(BorshDeserialize, BorshSerialize)]
struct ProgramData {
    counter: u32,
}

entrypoint!(counter_contract);

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    intruction_data: &[u8],
) -> ProgramResult {
    let acc = next_account_info(&mut accounts.iter())?;
    let data = InstructionData::try_from_slice(intruction_data)?;
    let mut program_data = ProgramData::try_from_slice(& acc.data.borrow())?; 
    
    match data {
        InstructionData::Increment(amount) => {
            program_data.counter += amount;
        },
        InstructionData::Decrement(amount) => {
            let mut program_data = ProgramData::try_from_slice(& acc.data.borrow())?; 
            program_data.counter -= amount;
        },
    };

    program_data.serialize(&mut *acc.data.borrow_mut());
    msg!("Counter updated to {}", program_data.counter);

    Ok(())
}