pub use crate::processor::init;
pub use crate::processor::register;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::sysvar;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
};

use spl_name_service;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum RegistryInstruction {
    // 0
    // Accounts expected by this insctruction
    //
    // | Index | Writable | Signer | Description    |
    // |-------|----------|--------|----------------|
    // | 0     | ✅        | ❌      | State account  |
    // | 1     | ❌        | ❌      | System program |
    // | 2     | ✅        | ✅      | Fee payer      |
    // | 3     | ❌        | ❌      | Rent sysvar    |
    Init(init::Params),

    // 1
    // Accounts expected by this instructions
    //
    // | Index | Writable | Signer | Description                   |
    // |-------|----------|--------|-------------------------------|
    // | 0     | ❌        | ❌      | Name service program          |
    // | 1     | ❌        | ❌      | System program                |
    // | 2     | ✅        | ✅      | Fee payer                     |
    // | 3     | ❌        | ❌      | Rent sysvar                   |
    // | 4     | ✅        | ❌      | Ticker name account           |
    // | 5     | ✅        | ❌      | Mint name account             |
    // | 6     | ❌        | ❌      | Root name account (Token TLD) |
    // | 7     | ❌        | ❌      | Central state account         |
    Register(register::Params),
}

pub fn init(
    token_registry_program_id: Pubkey,
    state_account: Pubkey,
    fee_payer: Pubkey,
    params: init::Params,
) -> Instruction {
    let instruction_data = RegistryInstruction::Init(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new(state_account, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(fee_payer, true),
        AccountMeta::new_readonly(sysvar::rent::ID, false),
    ];

    Instruction {
        program_id: token_registry_program_id,
        accounts,
        data,
    }
}

pub fn register(
    token_registry_program_id: Pubkey,
    fee_payer: Pubkey,
    ticker_name_account: Pubkey,
    mint_name_account: Pubkey,
    root_name_account: Pubkey,
    central_state: Pubkey,
    params: register::Params,
) -> Instruction {
    let instruction_data = RegistryInstruction::Register(params);
    let data = instruction_data.try_to_vec().unwrap();
    let accounts = vec![
        AccountMeta::new_readonly(spl_name_service::ID, false),
        AccountMeta::new_readonly(system_program::ID, false),
        AccountMeta::new(fee_payer, true),
        AccountMeta::new_readonly(sysvar::rent::ID, false),
        AccountMeta::new(ticker_name_account, false),
        AccountMeta::new(mint_name_account, false),
        AccountMeta::new(root_name_account, false),
        AccountMeta::new(central_state, false),
    ];

    Instruction {
        program_id: token_registry_program_id,
        accounts,
        data,
    }
}
