use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

use super::DataLen;

pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amout: [u8; 8],
    pub bump: u8,
}

impl DataLen for Escrow {
    const LEN: usize = core::mem::size_of::<Escrow>();
}

impl Escrow {
    pub fn init(
        escrow_acc: &AccountInfo,
        maker: Pubkey,
        mint_a: Pubkey,
        mint_b: Pubkey,
        receive_amout: [u8; 8],
        bump: u8,
    ) {
        let escrow =
            unsafe { &mut *(escrow_acc.borrow_mut_data_unchecked().as_ptr() as *mut Self) };
        escrow.maker = maker;
        escrow.mint_a = mint_a;
        escrow.mint_b = mint_b;
        escrow.receive_amout = receive_amout;
        escrow.bump = bump;
    }
}
