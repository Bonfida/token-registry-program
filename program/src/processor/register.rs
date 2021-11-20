use std::str::FromStr;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_program,
    sysvar::Sysvar,
};

use borsh::{BorshDeserialize, BorshSerialize};

use crate::{
    cpi::Cpi,
    error::TokenRegistryError,
    state::{Mint, TokenData, TOKEN_TLD},
    utils::{check_account_key, check_account_owner, check_name_account, check_signer},
};

#[cfg(not(feature = "test-bpf"))]
use crate::utils::check_registrar_signer;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Params {
    pub signer_index: usize,
    pub ticker: String,
    pub name: String,
    pub mint: String,
    pub decimals: u8,
    pub website: String,
    pub logo_uri: String,
}

impl<'a, 'b: 'a> Params {
    fn parse_params(
        params: &Params,
        accounts: &Accounts,
    ) -> Result<(Vec<u8>, Vec<u8>), ProgramError> {
        #[cfg(not(feature = "test-bpf"))]
        check_registrar_signer(accounts.fee_payer.key, params.signer_index)?;

        let hashed_ticker_name =
            check_name_account(&params.ticker, accounts.ticker_name_account.key)?;
        let hashed_mint_name = check_name_account(&params.mint, accounts.mint_name_account.key)?;

        Ok((hashed_ticker_name, hashed_mint_name))
    }
}

struct Accounts<'a, 'b: 'a> {
    name_service_program: &'a AccountInfo<'b>,
    system_program: &'a AccountInfo<'b>,
    fee_payer: &'a AccountInfo<'b>,
    rent_sysvar_account: &'a AccountInfo<'b>,
    ticker_name_account: &'a AccountInfo<'b>,
    mint_name_account: &'a AccountInfo<'b>,
    root_name_account: &'a AccountInfo<'b>,
    central_state: &'a AccountInfo<'b>,
}

impl<'a, 'b: 'a> Accounts<'a, 'b> {
    fn parse_accounts(
        program_id: &Pubkey,
        accounts: &'a [AccountInfo<'b>],
    ) -> Result<Accounts<'a, 'b>, ProgramError> {
        let accounts_iter = &mut accounts.iter();
        let accounts = Accounts {
            name_service_program: next_account_info(accounts_iter)?,
            system_program: next_account_info(accounts_iter)?,
            fee_payer: next_account_info(accounts_iter)?,
            rent_sysvar_account: next_account_info(accounts_iter)?,
            ticker_name_account: next_account_info(accounts_iter)?,
            mint_name_account: next_account_info(accounts_iter)?,
            root_name_account: next_account_info(accounts_iter)?,
            central_state: next_account_info(accounts_iter)?,
        };

        if accounts.ticker_name_account.data_len() != 0 {
            msg!("Name account (ticker) is already initialized.");
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if accounts.mint_name_account.data_len() != 0 {
            msg!("Name account (mint) is already initialized.");
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        // Key checks
        check_account_key(
            accounts.name_service_program,
            &spl_name_service::ID,
            TokenRegistryError::InvalidKey,
        )?;
        check_account_key(
            accounts.system_program,
            &system_program::ID,
            TokenRegistryError::InvalidKey,
        )?;
        check_account_key(
            accounts.rent_sysvar_account,
            &solana_program::sysvar::rent::ID,
            TokenRegistryError::InvalidKey,
        )?;
        check_account_key(
            accounts.root_name_account,
            &Pubkey::from_str(TOKEN_TLD).unwrap(),
            TokenRegistryError::InvalidTld,
        )?;

        // Ownership checks
        check_account_owner(
            accounts.root_name_account,
            &spl_name_service::ID,
            TokenRegistryError::InvalidKey,
        )?;
        check_account_owner(
            accounts.central_state,
            program_id,
            TokenRegistryError::InvalidKey,
        )?;

        // Signer checks
        check_signer(accounts.fee_payer)?;

        Ok(accounts)
    }
}

pub(crate) fn process(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    params: Params,
) -> ProgramResult {
    let accounts = Accounts::parse_accounts(program_id, accounts)?;
    let (hashed_ticker_name, hashed_mint_name) = Params::parse_params(&params, &accounts)?;

    let mint_as_bytes = Pubkey::from_str(params.mint.as_str()).unwrap().to_bytes();

    // Token data
    let token_data = TokenData::new(
        params.name,
        params.ticker,
        mint_as_bytes,
        params.decimals,
        params.website,
        params.logo_uri,
    )
    .try_to_vec()
    .unwrap();

    // Mint data
    let mint_data = Mint::new(mint_as_bytes).try_to_vec().unwrap();

    let lamports_token_data = Rent::get()?.minimum_balance(token_data.len());
    let lamports_mint_data = Rent::get()?.minimum_balance(mint_data.len());

    let central_state_nonce = accounts.central_state.data.borrow()[0];
    let central_state_signer_seeds: &[&[u8]] = &[&program_id.to_bytes(), &[central_state_nonce]];

    //// Create ticker registry
    Cpi::create_name_account(
        accounts.name_service_program,
        accounts.system_program,
        accounts.ticker_name_account,
        accounts.fee_payer,
        accounts.central_state,
        accounts.root_name_account,
        accounts.central_state,
        hashed_ticker_name,
        lamports_mint_data,
        mint_data.len() as u32,
        central_state_signer_seeds,
    )?;

    //// Create mint registry
    Cpi::create_name_account(
        accounts.name_service_program,
        accounts.system_program,
        accounts.mint_name_account,
        accounts.fee_payer,
        accounts.central_state,
        accounts.root_name_account,
        accounts.central_state,
        hashed_mint_name,
        lamports_token_data,
        token_data.len() as u32,
        central_state_signer_seeds,
    )?;

    // Serialization
    Cpi::update_name_account_data(
        accounts.name_service_program,
        accounts.ticker_name_account,
        accounts.central_state,
        mint_data,
        accounts.root_name_account,
        central_state_signer_seeds,
    )?;
    Cpi::update_name_account_data(
        accounts.name_service_program,
        accounts.mint_name_account,
        accounts.central_state,
        token_data,
        accounts.root_name_account,
        central_state_signer_seeds,
    )?;

    Ok(())
}
