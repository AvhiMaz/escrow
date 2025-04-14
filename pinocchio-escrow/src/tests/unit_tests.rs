#![allow(unused_attributes)]
#![allow(unused_variables)]
#![allow(clippy::module_inception)]

#[cfg(test)]
mod tests {

    #![no_std]
    extern crate alloc;

    use mollusk_svm::{program, Mollusk};
    use solana_sdk::{
        account::{Account, WritableAccount},
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

        let mint_x = Pubkey::new_from_array([0x03; 32]);
        let mut mint_x_account = Account::new(
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
            mint_x_account.data_as_mut_slice(),
        )
        .unwrap();
    }
}
