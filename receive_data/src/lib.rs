use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let data_account = next_account_info(accounts_iter)?;
    let payer_account = next_account_info(accounts_iter)?;
    let pda_account = next_account_info(accounts_iter)?;

    if data_account.owner != program_id {
        msg!("Error: Account does not belong to this program");
        return Err(ProgramError::IncorrectProgramId);
    }

    let seed = b"seed";
    let (pda, _bump_seed) = Pubkey::find_program_address(
        &[seed, &payer_account.key.to_bytes(), &program_id.to_bytes()],
        program_id,
    );
    if *pda_account.key != pda {
        msg!("Error: Invalid PDA, the account provided is not the expected PDA");
        return Err(ProgramError::InvalidAccountData);
    }

    if !data_account.is_writable {
        msg!("Error: Data account is not writable");
        return Err(ProgramError::InvalidAccountData);
    }

    if instruction_data.len() != 4 {
        msg!("Error: Invalid instruction data length");
        return Err(ProgramError::InvalidInstructionData);
    }

    let number = u32::from_le_bytes(
        instruction_data
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );

    if number < 1 || number > 5 {
        msg!("Error: Number must be between 1 and 5");
        return Err(ProgramError::InvalidInstructionData);
    }

    let data = &mut data_account.data.borrow_mut();
    data[0..4].copy_from_slice(&instruction_data);
    msg!("Successfully wrote data: {}", number);

    Ok(())
}
