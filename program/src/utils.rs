use std::str::FromStr;

use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, hash::hashv, msg,
    program_error::ProgramError, pubkey::Pubkey,
};

use crate::{
    error::TokenRegistryError,
    state::{ADMINS, TOKEN_TLD},
};
use spl_name_service::state::{get_seeds_and_key, HASH_PREFIX};

// Safety verification functions
pub fn check_account_key(
    account: &AccountInfo,
    key: &Pubkey,
    error: TokenRegistryError,
) -> Result<(), TokenRegistryError> {
    if account.key != key {
        return Err(error);
    }
    Ok(())
}

pub fn check_account_owner(
    account: &AccountInfo,
    owner: &Pubkey,
    error: TokenRegistryError,
) -> Result<(), TokenRegistryError> {
    if account.owner != owner {
        return Err(error);
    }
    Ok(())
}

pub fn check_signer(account: &AccountInfo) -> ProgramResult {
    if !(account.is_signer) {
        return Err(ProgramError::MissingRequiredSignature);
    }
    Ok(())
}

pub fn check_registrar_signer(key: &Pubkey, index: usize) -> ProgramResult {
    if ADMINS.get(index).unwrap() != &key.to_string() {
        return Err(TokenRegistryError::NonWhiteListedSigner.into());
    }
    Ok(())
}

pub fn check_name_account(name: &str, unsafe_name_key: &Pubkey) -> Result<Vec<u8>, ProgramError> {
    let hashed_name = hashv(&[(HASH_PREFIX.to_owned() + name).as_bytes()])
        .0
        .to_vec();

    if hashed_name.len() != 32 {
        msg!("Invalid seed length");
        return Err(ProgramError::InvalidArgument);
    }

    let (name_account_key, _) = get_seeds_and_key(
        &spl_name_service::ID,
        hashed_name.clone(),
        None,
        Some(&Pubkey::from_str(TOKEN_TLD).unwrap()),
    );

    if name_account_key != *unsafe_name_key {
        msg!("Provided wrong name account");
        return Err(TokenRegistryError::InvalidNameProvided.into());
    }

    Ok(hashed_name)
}
