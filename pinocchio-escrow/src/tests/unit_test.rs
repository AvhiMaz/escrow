#![allow(unused_attributes)]
#![allow(unused_variables)]
#![cfg(test)]

mod tests {
    #![no_std]
    extern crate alloc;

    use mollusk_svm::{program, Mollusk};
    use pinocchio::pubkey::find_program_address;
    use pinocchio_log::log;
    use solana_sdk::{
        account::{Account, WritableAccount},
        msg,
        native_token::LAMPORTS_PER_SOL,
        pubkey,
        pubkey::Pubkey,
    };
    const ID: Pubkey = pubkey!("E6UcK3dSFc2yaFtEb35pc1WsBVcrPhEbnB87YoNDXhqy");
    const MINT_LEN: usize = 82;

    pub fn mollusk() -> Mollusk {
        Mollusk::new(&ID, "target/deploy/pinocchio_escrow")
    }

    #[test]
    fn make_test() {
        /*
        create instance
        init system_program and system_account
        add spl token program
        init maker
        init maker account
        find the pda
        init escrow account
        init mint_a
        init mint_b
        init maker_ata_a
        init vault
        load data
        create instruction
        process and validate instruction
        */

        let mut mollusk = mollusk();

        let (system_program, _system_account) =
            mollusk_svm::program::keyed_account_for_system_program();

        mollusk.add_program(
            &spl_token::ID,
            "src/tests/spl_token-3.5.0",
            &mollusk_svm::program::loader_keys::LOADER_V3,
        );

        let (token_program, token_account) = (
            spl_token::ID,
            program::create_program_account_loader_v3(&spl_token::ID),
        );

        let maker = Pubkey::new_from_array([0x02; 32]);

        log!("maker: {}", maker.to_string().as_str());
        msg!("maker: {:?}", maker.to_string().as_str());

        let maker_acc = Account::new(2 * LAMPORTS_PER_SOL, 0, &system_program);

        let (escrow, escrow_bump) = solana_sdk::pubkey::Pubkey::find_program_address(
            &[(b"escrow"), &maker.to_bytes()],
            &ID,
        );

        let escrow_acc = Account::new(0, 0, &system_program);

        let mint_a = Pubkey::new_from_array([0x03; 32]);
        let mut mint_a_acc = Account::new(
            mollusk.sysvars.rent.minimum_balance(MINT_LEN),
            MINT_LEN,
            &token_program,
        );
        solana_sdk::program_pack::Pack::pack(
            spl_token::state::Mint {
                mint_authority: solana_sdk::program_option::COption::None,
                supply: 100_000_000,
                decimals: 6,
                is_initialized: true,
                freeze_authority: solana_sdk::program_option::COption::None,
            },
            mint_a_acc.data_as_mut_slice(),
        )
        .unwrap();
    }
}
