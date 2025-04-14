#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(clippy::module_inception)]

#[cfg(test)]
mod tests {

    #![no_std]
    extern crate alloc;

    use mollusk_svm::{program, result::Check, Mollusk};
    use solana_sdk::{
        account::{Account, WritableAccount},
        instruction::{AccountMeta, Instruction},
        native_token::LAMPORTS_PER_SOL,
        program_option::COption,
        program_pack::Pack,
        pubkey,
        pubkey::Pubkey,
    };

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
        let maker_account = Account::new(1 * LAMPORTS_PER_SOL, 0, &system_program);

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
            vec![0],
            vec![escrow_bump],
            1_000_000u64.to_le_bytes().to_vec(),
            1_000_000u64.to_le_bytes().to_vec(),
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
}
