use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
    program_error::ProgramError,
    program_pack::{Pack, IsInitialized},
    instruction::{AccountMeta, Instruction},
    system_instruction, system_program,
    transaction::Transaction,
};

use borsh::{BorshDeserialize, BorshSerialize};

entrypoint!(process_instruction);

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TipContract {
    is_initialized: bool,
    owner: Pubkey,
    deposited_amount: u64,
}

// ... (imports and struct definition)

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut tip_contract = TipContract::try_from_slice(&account.data.borrow())?;

    if tip_contract.is_initialized {
        if tip_contract.owner == *account.key {
            send_tip(&mut tip_contract, accounts_iter, instruction_data, account)?;
        } else {
            deposit_into_contract(&mut tip_contract, accounts_iter, account)?;
        }
    } else {
        initialize_contract(&mut tip_contract, accounts_iter, account)?;
    }

    tip_contract.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

fn send_tip(
    tip_contract: &mut TipContract,
    accounts_iter: &mut std::slice::Iter<AccountInfo>,
    instruction_data: &[u8],
    account: &AccountInfo,
) -> ProgramResult {
    let recipient = next_account_info(accounts_iter)?;
    let amount = u64::from_le_bytes(*array_ref![instruction_data, 0, 8]);
    if **account.lamports.borrow() < amount {
        return Err(ProgramError::InsufficientFunds);
    }
    **account.lamports.borrow_mut() = account.lamports.borrow().checked_sub(amount).ok_or(ProgramError::InsufficientFunds)?;
    **recipient.lamports.borrow_mut() = recipient.lamports.borrow().checked_add(amount).ok_or(ProgramError::InsufficientFunds)?;
    msg!("Sent {} lamports from {} to {}", amount, tip_contract.owner, recipient.key);
    Ok(())
}

fn deposit_into_contract(
    tip_contract: &mut TipContract,
    accounts_iter: &mut std::slice::Iter<AccountInfo>,
    account: &AccountInfo,
) -> ProgramResult {
    let depositor = next_account_info(accounts_iter)?;
    let amount = **depositor.lamports.borrow();
    if amount == 0 {
        return Err(ProgramError::InsufficientFunds);
    }
    **depositor.lamports.borrow_mut() = 0;
    **account.lamports.borrow_mut() = account.lamports.borrow().checked_add(amount).ok_or(ProgramError::InsufficientFunds)?;
    tip_contract.deposited_amount = tip_contract.deposited_amount.checked_add(amount).ok_or(ProgramError::InsufficientFunds)?;
    msg!("Deposited {} lamports from {} to the contract", amount, depositor.key);
    Ok(())
}

fn initialize_contract(
    tip_contract: &mut TipContract,
    accounts_iter: &mut std::slice::Iter<AccountInfo>,
    account: &AccountInfo,
) -> ProgramResult {
    let owner = next_account_info(accounts_iter)?;
    tip_contract.is_initialized = true;
    tip_contract.owner = *owner.key;
    msg!("Initialized the contract with owner {}", owner.key);
    Ok(())
}
