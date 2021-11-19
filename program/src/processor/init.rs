use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program, sysvar,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    cpi::Cpi,
    error::TokenRegistryError,
    state::CentralState,
    utils::{check_account_key, check_account_owner},
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Params {
    pub signer_nonce: u8,
}

struct Accounts<'a, 'b: 'a> {
    state_account: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    fee_payer: &'a AccountInfo<'b>,
    rent_sysvar_account: &'a AccountInfo<'b>,
}

impl<'a, 'b: 'a> Accounts<'a, 'b> {
    fn parse_accounts(
        _program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Accounts<'a, 'b>, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Accounts {
            state_account: next_account_info(accounts_iter)?,
            system_program: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
            rent_sysvar_account: next_account_info(accounts_iter)?,
        };

        check_account_owner(
            accounts.state_account,
            &system_program::id(),
            TokenRegistryError::InvalidKey,
        )?;
        check_account_key(
            accounts.system_program,
            &system_program::id(),
            TokenRegistryError::InvalidKey,
        )?;
        check_account_key(
            accounts.rent_sysvar_account,
            &sysvar::rent::id(),
            TokenRegistryError::InvalidKey,
        )?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse_accounts(program_id, accounts)?;
    let Params { signer_nonce } = params;

    let signer_seeds: &[&[u8]] = &[&program_id.to_bytes(), &[signer_nonce]];
    let derived_state_key = Pubkey::create_program_address(signer_seeds, program_id)?;

    if &derived_state_key != accounts.state_account.key {
        msg!("Incorrect state account or signer nonce provided");
        return Err(ProgramError::InvalidArgument);
    }

    Cpi::create_account(
        program_id,
        accounts.system_program,
        accounts.fee_payer,
        accounts.state_account,
        accounts.rent_sysvar_account,
        signer_seeds,
        CentralState::LEN,
    )?;

    let state = CentralState { signer_nonce };
    state.pack_into_slice(&mut accounts.state_account.data.borrow_mut());

    Ok(())
}
