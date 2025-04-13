use pinocchio::{
    account_info::AccountInfo,
    instruction::{Seed, Signer},
    program_error::ProgramError,
    pubkey::find_program_address,
    ProgramResult,
};

use crate::{error::MyProgramError, state::EscrowState};

pub fn process_take_instruction(accounts: &[AccountInfo]) -> ProgramResult {
    // all accounts that we need
    let [taker_acc, maker_acc, _mint_a, _mint_b, taker_ata_a, taker_ata_b, maker_ata_b, escrow, vault, _token_prgram, _system_program, _remaining @ ..] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // load the data (borrow it)
    let escrow_data = escrow
        .try_borrow_data()
        .map_err(|_| ProgramError::AccountBorrowFailed)?;

    // deserialize the data using bytemuck
    let ser_escrow_data = bytemuck::try_from_bytes::<EscrowState>(&escrow_data)
        .map_err(|_| MyProgramError::DeserializationFailed)?;

    // can we can do it like this without bytemuck ?

    /*pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        if input.len() < EscrowState::LEN {
            return Err(ProgramError::InvalidAccountData);
        }

        let (maker_bytes, rest) = input.split_at(32);
        let (mint_a_bytes, rest) = rest.split_at(32);
        let (mint_b_bytes, rest) = rest.split_at(32);
        let (amount_a_bytes, rest) = rest.split_at(8);
        let (amount_b_bytes, rest) = rest.split_at(8);
        let bump_byte = rest.get(0).ok_or(ProgramError::InvalidAccountData)?;

        Ok(Self {
            maker: Pubkey::new(maker_bytes),
            mint_a: Pubkey::new(mint_a_bytes),
            mint_b: Pubkey::new(mint_b_bytes),
            amount_mint_a: u64::from_le_bytes(amount_a_bytes.try_into().unwrap()),
            amount_mint_b: u64::from_le_bytes(amount_b_bytes.try_into().unwrap()),
            bump: *bump_byte,
        })
    }
    let ser_escrow_data = EscrowState::unpack(&escrow_data);
    */

    //parsing the vault account as a TokenAccount

    let vault_acc = pinocchio_token::state::TokenAccount::from_account_info(vault);

    //creating seed
    let seed = [
        b"escrow",
        maker_acc.key().as_slice(),
        &[ser_escrow_data.bump], //accessing here afetr deserializing it
    ];

    //Take the whole array seed and turn it into a slice.
    //Converts the array into a slice of &[u8], needed by find_program_address.
    let seeds = &seed[..];

    let _escrow_pda = find_program_address(seeds, &crate::ID);

    // transfer happening from taker to the maker
    pinocchio_token::instructions::Transfer {
        from: taker_ata_b,
        to: maker_ata_b,
        amount: u64::from_le_bytes(ser_escrow_data.receive_amout),
        authority: taker_acc,
    }
    .invoke()?;

    let bump = [ser_escrow_data.bump];
    let seed = [
        Seed::from(b"escrow"),
        Seed::from(maker_acc.key()),
        Seed::from(&bump),
    ];
    let seeds = Signer::from(&seed);

    // transfer happening from vault (the token was sent by the maker to the vault) to the taker
    pinocchio_token::instructions::Transfer {
        from: vault,
        to: taker_ata_a,
        authority: escrow,
        amount: vault_acc?.amount(),
    }
    .invoke_signed(&[seeds.clone()])?;

    pinocchio_token::instructions::CloseAccount {
        account: vault,
        destination: maker_acc,
        authority: escrow,
    }
    .invoke_signed(&[seeds])?;

    Ok(())
}
