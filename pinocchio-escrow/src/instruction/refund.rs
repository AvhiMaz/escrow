use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};

use crate::{error::MyProgramError, state::EscrowState};

pub fn process_refund_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    let [maker_acc, _mint_a, maker_ata_a, vault, escrow, _token_program, _remaining @ ..] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let escrow_data = escrow
        .try_borrow_data()
        .map_err(|_| ProgramError::AccountBorrowFailed)?;

    let ser_escrow_data = bytemuck::try_from_bytes::<EscrowState>(&escrow_data)
        .map_err(|_| MyProgramError::DeserializationFailed)?;

    let vault_acc = pinocchio_token::state::TokenAccount::from_account_info(vault);

    let seed = [
        b"escrow",
        maker_acc.key().as_slice(),
        &[ser_escrow_data.bump],
    ];

    let seeds = &seed[..];

    //let escrow_pda = find_program_address(seeds, &crate::ID); // [u8; 32, u8]
    // escrow is [u8; 32] so we can't compare with ([u8; 32], u8)

    let (escrow_pda, _bump) = find_program_address(seeds, &crate::ID);

    assert_eq!(*escrow.key(), escrow_pda);

    let bump = [ser_escrow_data.bump];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker_acc.key()),
        Seed::from(&bump),
    ];

    let seeds = Signer::from(&seed);

    pinocchio_token::instructions::Transfer {
        from: vault,
        to: maker_ata_a,
        amount: vault_acc?.amount(),
        authority: escrow,
    }
    .invoke_signed(&[seeds.clone()])?;

    pinocchio_token::instructions::CloseAccount {
        account: vault,
        authority: escrow,
        destination: maker_acc,
    }
    .invoke_signed(&[seeds])?;

    unsafe {
        *maker_acc.borrow_mut_lamports_unchecked() += *escrow.borrow_lamports_unchecked();
        *escrow.borrow_mut_lamports_unchecked() = 0
    };

    Ok(())
}
