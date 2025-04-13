#![allow(dead_code)]

use bytemuck::{Pod, Zeroable};
use pinocchio::{
    account_info::AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvars::{rent::Rent, Sysvar},
    ProgramResult,
};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
//? Zeroable is setting the bits of the data field to 0
//? Pod was the sole reason for safely byte-casting from raw-bytes to struct type
//? Pod has what's called as padding - padding adds 7 extra bytes and something called as offset
pub struct MakeOfferIxData {
    pub bump: u8,               //? 1 byte offset - 0
    pub amount_mint_a: [u8; 8], //? 8 bytes + offset - 1-9
    pub amount_mint_b: [u8; 8], //? 8 bytes + offset - 9-17
}

use crate::state::{DataLen, EscrowState};

pub fn process_make_offer_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Steps to process the instruction:
    // extract the required accounts from the `accounts: &[AccountInfo]`` array
    // mention fail safe checks for the accounts and data length
    // check if the maker account is a signer
    // extract the bump seeds from the instruction data and bind seeds for the PDA validation
    // create the PDA for the escrow account and validate the PDA
    // for the vault account, we can either create a new one or use it from the instruction data
    // verify if vault is owned by the escrow program
    // check if the escrow account is initialized
    // check if the vault account is initialized
    // Create the account for escrow with the required space
    // initialize the escrow account with the required data
    // transfer the tokens from the maker to the vaultq
    //? extract the required accounts from the `accounts: &[AccountInfo]`` array
    //? `_remaining @ .. is used to catch/bind the unnecessary accounts it receives from the client`
    
    let [maker_acc, mint_a, mint_b, maker_ata, vault, escrow_acc, _system_program, _token_program, _remaining @ ..] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    //? check if the maker account is a signer
    if !maker_acc.is_signer() {
        return Err(ProgramError::MissingRequiredSignature);
    }

    //? mention fail safe checks for the accounts and data length
    if  instruction_data.len() < 17 {
        return Err(ProgramError::InvalidInstructionData);
    }

    //? extract the bump seeds from the instruction data and bind seeds for the PDA validation
    /*
       let bump = unsafe {
           *(instruction_data.as_ptr() as *const u8)
       }
       .to_le_bytes();
       let seed = [(b"escrow"), maker_acc.key().as_slice(), bump.as_ref()];
       let seeds = &seed[..];

       //? create the PDA for the escrow account and validate the PDA
       let pda = pubkey::checked_create_program_address (
           seeds,
           &crate::ID,
       ).unwrap(); //! can use unwrap here

       assert!(mint_x.owner(), &pinocchio_token::ID);
       assert!(mint_y.owner(), &pinocchio_token::ID);
    */
    let _bump = instruction_data[0]; //? offset is 0
    let amount_mint_a = u64::from_le_bytes(
        //? offset is 1-9
        instruction_data[1..9]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?,
    );
    let amount_mint_b = 
        //? offset is 9-17
        instruction_data[9..17]
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;
    

    //? extract the required seeds from the data
    let seed = &["escrow".as_bytes(), maker_acc.key().as_ref()];

    let (_pda, bump) = pinocchio::pubkey::try_find_program_address(seed, &crate::ID)
        .ok_or(ProgramError::InvalidSeeds)?;

    // create the escrow account
    pinocchio_system::instructions::CreateAccount {
        from: maker_acc,
        to: escrow_acc,
        lamports: Rent::get()?.minimum_balance(EscrowState::LEN),
        space: EscrowState::LEN as u64,
        owner: &crate::ID,
    }
    .invoke()?;

    //TODO: need to make some unsafe checks

    //? actually initializing the account
    EscrowState::initialize(
        escrow_acc,
        *maker_acc.key(),
        *mint_a.key(),
        *mint_b.key(),
        amount_mint_b,
        bump,
    );

    pinocchio_token::instructions::Transfer {
        from: maker_ata,
        to: vault,
        authority: maker_acc,
        amount: amount_mint_a,
    }
    .invoke()?;

    //     if escrow.data_is_empty() {
    //         log!("Creating the escrow account");

    //         //? verify if the escrow account is owner of the vault
    //         assert!(TokenAccount::load_mut_raw_unchecked(vault).unwrap().owner() == escrow_acc.key());

    //         //? now check if this is the first-time initialization
    //         if escrow_acc.owner() != &crate::ID {
    //             log!("Initializing the escrow account")

    //             let seed = [
    //                 Seed::from(b"escrow"),
    //                 Seed::from(maker_acc.key()),
    //                 Seed::from(&bump)
    //             ];

    //             //? now create the escrow account with space -> Rent
    //             pinocchio_system::instruction::CreateAccount {
    //                 from; maker_acc,
    //                 to: escrow,
    //                 lamports: Rent::get()?.minimum_balance(EscrowState::LEN);
    //                 space: Escrow:LEN as u64,
    //                 owner: &crate::ID,
    //             };

    //             //? fill in the escrow details and initialize
    //             let init_escrow_acc = EscrowState::load_mut_raw_unchecked(&escrow_acc);
    //             init_escrow_acc.maker = *maker.key();
    //             init_escrow_acc.mint_x = *mint_x.key();
    //             init_escrow_acc.mint_y = *mint_y.key();
    //             init_escrow_acc.amount = *(instruction_data.as_ptr().add(1) as *const as u64); // TokenY amount requested
    //             init_escrow_acc.bump = *(data.as_ptr());

    //             let amount = *(instruction_data.as_ptr().add(1 + 8) as *const u64); // amount needed to deposit in vault

    //             pinocchio_system::instruction::Transfer {
    //                 from: maker_ata,
    //                 to: vault,
    //                 authority: maker,
    //                 amount,
    //             }
    //             .invoke()?;
    //         } else {
    //             return Err(ProgramError::AccountAlreadyInitialized);
    //         }
    //     }

    Ok(())
}
