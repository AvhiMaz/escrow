#![allow(unused_imports)]

use pinocchio::{
    account_info::AccountInfo, default_panic_handler, no_allocator, program_entrypoint,
    program_error::ProgramError, pubkey::Pubkey, ProgramResult,
};

use crate::instruction::{self, EscrowInstructions};

// This is the entrypoint for the program.
program_entrypoint!(process_instruction);
// Do not allocate heap memory.
//no_allocator!();
// Use the default panic handler || ensures that panics are caught and turned into program errors..
default_panic_handler!();

#[inline(always)]
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let Some((ix_desc, instruction_data)) = instruction_data.split_first() else {
        //separates the first byte (called a discriminator) from the rest.
        //discriminator determines which instruction you're trying to run (like a function ID).
        return Err(ProgramError::InvalidInstructionData);
    };

    match EscrowInstructions::try_from(ix_desc)? {
        EscrowInstructions::Make => {
            instruction::process_make_offer_instruction(program_id, accounts, instruction_data)
        }

        EscrowInstructions::Take => instruction::process_take_instruction(accounts),
        EscrowInstructions::Refund => instruction::process_refund_instruction(accounts),
    }
}
