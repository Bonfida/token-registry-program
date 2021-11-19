use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};

pub const ADMINS: [&str; 1] = ["9f9K1Jwoys9r7hQFwKB1aqrk7AT47D8UogM4s6npEKLa"];
pub const TOKEN_TLD: &str = "6NSu2tci4apRKQtt257bAVcvqYjB3zV2H1dWo56vgpa6";

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct CentralState {
    pub signer_nonce: u8,
}

impl Sealed for CentralState {}

impl Pack for CentralState {
    const LEN: usize = 1;

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let mut pt = dst;
        self.serialize(&mut pt).unwrap();
    }

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let mut pt = src;
        let res = Self::deserialize(&mut pt)?;
        Ok(res)
    }
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TokenData {
    pub name: String,
    pub ticker: String,
    pub mint: [u8; 32],
    pub decimals: u8,
    pub website: String,
    pub logo_uri: String,
}

impl TokenData {
    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Mint {
    pub mint: [u8; 32],
}

impl Mint {
    pub fn save(&self, mut dst: &mut [u8]) {
        self.serialize(&mut dst).unwrap()
    }
}
