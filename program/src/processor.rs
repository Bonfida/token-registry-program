use borsh::BorshDeserialize;
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::RegistryInstruction;

pub mod init;
pub mod register;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        msg!("Beginning processing");
        let instruction = RegistryInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        msg!("Instruction unpacked");

        match instruction {
            RegistryInstruction::Init(params) => {
                msg!("Instruction: Init central state");
                init::process(program_id, accounts, params)?;
            }
            RegistryInstruction::Register(params) => {
                msg!("Instruction: Register");
                register::process(program_id, accounts, params)?;
            }
        }
        Ok(())
    }
}
