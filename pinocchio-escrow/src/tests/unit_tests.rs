#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(clippy::module_inception)]

#[cfg(test)]
mod tests {

    #![no_std]
    extern crate alloc;

    use mollusk_svm::{program, result::Check, Mollusk};
    use pinocchio_log::log;
    use solana_sdk::{
        account::{Account, WritableAccount},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        program_option::COption,
        program_pack::Pack,
        pubkey,
        pubkey::Pubkey,
    };

    use crate::state::{DataLen, EscrowState};

    const ID: Pubkey = pubkey!("E6UcK3dSFc2yaFtEb35pc1WsBVcrPhEbnB87YoNDXhqy");

    #[test]
    fn test_make() {
        let mut mollusk = Mollusk::new(&ID, "target/deploy/pinocchio_escrow");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        mollusk.add_program(
            &spl_token::ID,
            "src/tests/elfs/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        let maker = Pubkey::new_from_array([0x02; 32]);
        let maker_account = Account::new(LAMPORTS_PER_SOL, 0, &system_program);

        let (escrow, escrow_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[(b"escrow"), &maker.to_bytes()],
            &ID,
        );
        let escrow_account = Account::new(0, 0, &system_program);

        let mint_a = Pubkey::new_from_array([0x03; 32]);
        let mut mint_a_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_a_account.data_as_mut_slice(),
        )
        .unwrap();
        let mint_b = Pubkey::new_from_array([0x03; 32]);
        let mut mint_b_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_b_account.data_as_mut_slice(),
        )
        .unwrap();
        let maker_ata = Pubkey::new_from_array([0x05; 32]);
        let mut maker_ata_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: maker,
                amount: 100_000_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ata_account.data_as_mut_slice(),
        )
        .unwrap();
        let vault = Pubkey::new_from_array([0x06; 32]);
        let mut vault_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: escrow,
                amount: 1_000_000u64,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        )
        .unwrap();
        let data = [
            vec![0],                             // instruction discriminator (0 => Make)
            vec![escrow_bump],                   // bump
            1_000_000u64.to_le_bytes().to_vec(), // send amount
            1_000_000u64.to_le_bytes().to_vec(), // receive amount
        ]
        .concat();
        let instruction = Instruction::new_with_bytes(
            ID,
            &data,
            vec![
                AccountMeta::new(maker, true),
                AccountMeta::new_readonly(mint_a, false),
                AccountMeta::new_readonly(mint_b, false),
                AccountMeta::new(maker_ata, false),
                AccountMeta::new(vault, false),
                AccountMeta::new(escrow, true),
                AccountMeta::new_readonly(system_program, false),
                AccountMeta::new_readonly(token_program, false),
            ],
        );
        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (maker, maker_account),
                (mint_a, mint_a_account),
                (mint_b, mint_b_account),
                (maker_ata, maker_ata_account),
                (vault, vault_account),
                (escrow, escrow_account),
                (system_program, system_account),
                (token_program, token_account),
            ],
            &[Check::success()],
        );
    }

    #[test]
    fn test_take() {
        let mut mollusk = Mollusk::new(&ID, "target/deploy/pinocchio_escrow");

        let (system_program, system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        mollusk.add_program(
            &spl_token::ID,
            "src/tests/elfs/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );
        let maker = Pubkey::new_from_array([0x02; 32]);
        let maker_account = Account::new(LAMPORTS_PER_SOL, 0, &system_program);

        let taker = Pubkey::new_from_array([0x02; 32]);
        let taker_account = Account::new(LAMPORTS_PER_SOL, 0, &system_program);

        let (escrow, escrow_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[(b"escrow"), &maker.to_bytes()],
            &ID,
        );

        let escrow_account = Account::new(
            mollusk.sysvars.rent.minimum_balance(EscrowState::LEN),
            EscrowState::LEN,
            &ID,
        );

        let mint_a = Pubkey::new_from_array([0x03; 32]);
        let mut mint_a_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_a_account.data_as_mut_slice(),
        )
        .unwrap();
        let mint_b = Pubkey::new_from_array([0x03; 32]);
        let mut mint_b_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Mint::LEN),
            spl_token::state::Mint::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: COption::None,
                supply: 100_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: COption::None,
            },
            mint_b_account.data_as_mut_slice(),
        )
        .unwrap();
        let maker_ata_b = Pubkey::new_from_array([0x05; 32]);
        let mut maker_ata_b_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_b,
                owner: maker,
                amount: 100_000_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            maker_ata_b_account.data_as_mut_slice(),
        )
        .unwrap();
        let taker_ata_a = Pubkey::new_from_array([0x05; 32]);
        let mut taker_ata_a_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: taker,
                amount: 100_000_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            taker_ata_a_account.data_as_mut_slice(),
        )
        .unwrap();
        let taker_ata_b = Pubkey::new_from_array([0x05; 32]);
        let mut taker_ata_b_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_b,
                owner: taker,
                amount: 100_000_000,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            taker_ata_b_account.data_as_mut_slice(),
        )
        .unwrap();

        let vault = Pubkey::new_from_array([0x06; 32]);
        let mut vault_account = Account::new(
            mollusk
                .sysvars
                .rent
                .minimum_balance(spl_token::state::Account::LEN),
            spl_token::state::Account::LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Account {
                mint: mint_a,
                owner: escrow,
                amount: 0,
                delegate: COption::None,
                state: spl_token::state::AccountState::Initialized,
                is_native: COption::None,
                delegated_amount: 0,
                close_authority: COption::None,
            },
            vault_account.data_as_mut_slice(),
        )
        .unwrap();

        let data = [
            vec![2],           // it should be 1 or 2 ?      // instruction discriminator (2 => Take)
            vec![escrow_bump], // bump
            1_000_000u64.to_le_bytes().to_vec(), // send amount -> taker to maker
            1_000_000u64.to_le_bytes().to_vec(), // receive amount -> vault_account to taker
        ]
        .concat();

        let instruction = Instruction::new_with_bytes(
            ID,
            &data,
            vec![
                AccountMeta::new(taker, true),                    // taker_acc
                AccountMeta::new_readonly(maker, false),          // maker_acc
                AccountMeta::new_readonly(mint_a, false),         // _mint_a
                AccountMeta::new_readonly(mint_b, false),         // _mint_b
                AccountMeta::new(taker_ata_a, false),             // taker_ata_a
                AccountMeta::new(taker_ata_b, false),             // taker_ata_b
                AccountMeta::new(maker_ata_b, false),             // maker_ata_b
                AccountMeta::new(escrow, false),                  // escrow
                AccountMeta::new(vault, false),                   // vault
                AccountMeta::new_readonly(token_program, false),  // token_program
                AccountMeta::new_readonly(system_program, false), // system_program
            ],
        );

        mollusk.process_and_validate_instruction(
            &instruction,
            &vec![
                (taker, taker_account),
                (maker, maker_account),
                (mint_a, mint_a_account),
                (mint_b, mint_b_account),
                (taker_ata_a, taker_ata_a_account),
                (taker_ata_b, taker_ata_b_account),
                (maker_ata_b, maker_ata_b_account),
                (vault, vault_account),
                (escrow, escrow_account),
                (token_program, token_account),
                (system_program, system_account),
            ],
            &[Check::success()],
        );

        log!("here it is ok");
    }
}
