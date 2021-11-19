use std::str::FromStr;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed, pubkey::Pubkey,
    rent::Rent, system_instruction::create_account, sysvar::Sysvar,
};
use spl_name_service::instruction::NameRegistryInstruction;

use crate::state::TOKEN_TLD;

pub struct Cpi {}

impl Cpi {
    pub fn create_account<'a>(
        program_id: &Pubkey,
        system_program: &AccountInfo<'a>,
        fee_payer: &AccountInfo<'a>,
        account_to_create: &AccountInfo<'a>,
        rent_sysvar_account: &AccountInfo<'a>,
        signer_seeds: &[&[u8]],
        space: usize,
    ) -> ProgramResult {
        let rent = Rent::from_account_info(rent_sysvar_account)?;

        let create_state_instruction = create_account(
            fee_payer.key,
            account_to_create.key,
            rent.minimum_balance(space),
            space as u64,
            program_id,
        );

        invoke_signed(
            &create_state_instruction,
            &[
                system_program.clone(),
                fee_payer.clone(),
                account_to_create.clone(),
            ],
            &[signer_seeds],
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_name_account<'a>(
        name_service_program: &AccountInfo<'a>,
        system_program_account: &AccountInfo<'a>,
        name_account: &AccountInfo<'a>,
        fee_payer: &AccountInfo<'a>,
        new_owner_account: &AccountInfo<'a>,
        root_name_account: &AccountInfo<'a>,
        authority: &AccountInfo<'a>,
        hashed_name: Vec<u8>,
        lamports: u64,
        space: u32,
        signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        let create_name_instruction = spl_name_service::instruction::create(
            *name_service_program.key,
            NameRegistryInstruction::Create {
                hashed_name,
                lamports,
                space,
            },
            *name_account.key,
            *fee_payer.key,
            *new_owner_account.key,
            None,
            Some(*root_name_account.key),
            Some(*authority.key),
        )?;

        invoke_signed(
            &create_name_instruction,
            &[
                name_service_program.clone(),
                fee_payer.clone(),
                name_account.clone(),
                new_owner_account.clone(),
                system_program_account.clone(),
                root_name_account.clone(),
                authority.clone(),
            ],
            &[signer_seeds],
        )
    }

    pub fn update_name_account_data<'a>(
        name_service_program: &AccountInfo<'a>,
        name_account: &AccountInfo<'a>,
        name_update_signer: &AccountInfo<'a>,
        data: Vec<u8>,
        root_parent: &AccountInfo<'a>,
        signer_seeds: &[&[u8]],
    ) -> ProgramResult {
        let update_name_instruction = spl_name_service::instruction::update(
            *name_service_program.key,
            0,
            data,
            *name_account.key,
            *name_update_signer.key,
            Some(Pubkey::from_str(TOKEN_TLD).unwrap()),
        )?;

        invoke_signed(
            &update_name_instruction,
            &[
                name_service_program.clone(),
                name_account.clone(),
                name_update_signer.clone(),
                root_parent.clone(),
            ],
            &[signer_seeds],
        )
    }
}
