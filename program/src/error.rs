use num_derive::FromPrimitive;
use solana_program::{decode_error::DecodeError, program_error::ProgramError};
use thiserror::Error;

#[derive(Error, Debug, Clone, FromPrimitive)]
pub enum TokenRegistryError {
    #[error("Account not rent exempt")]
    AccountNotRentExempt = 0,
    #[error("Invalid key")]
    InvalidKey = 1,
    #[error("Invalid token TLD")]
    InvalidTld = 2,
    #[error("Non white listed signer")]
    NonWhiteListedSigner = 3,
    #[error("Invalid name provided")]
    InvalidNameProvided = 4,
}
impl From<TokenRegistryError> for ProgramError {
    fn from(e: TokenRegistryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for TokenRegistryError {
    fn type_of() -> &'static str {
        "Token Registry Error"
    }
}
