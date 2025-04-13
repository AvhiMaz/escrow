use bytemuck::{Pod, Zeroable};
use pinocchio::{account_info::AccountInfo, pubkey::Pubkey};

use super::DataLen;

#[repr(C)] // takes care of the predictable memory/data alignment
#[derive(Clone, Copy, Pod, Zeroable)]

// pod = plain old data -> checks all the data are plain in raw bytes and change it to bytecaste
// (deserilization)
// zeroable = all the bits to 0
// pod + zeroable = it ensures and gautrenetegeetete that this struct is prefectly safe for
// bytecaseting || converting the raw data to your type (which the struct escrow)

pub struct EscrowState {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub receive_amout: [u8; 8],
    pub bump: u8,
}

impl DataLen for EscrowState {
    const LEN: usize = core::mem::size_of::<EscrowState>();
}

impl EscrowState {
    pub fn initialize(
        escrow_acc: &AccountInfo,
        maker: Pubkey,
        mint_a: Pubkey,
        mint_b: Pubkey,
        receive_amout: [u8; 8],
        bump: u8,
    ) {
        let escrow =
            unsafe { &mut *(escrow_acc.borrow_mut_data_unchecked().as_ptr() as *mut Self) }; // bytecasting
        escrow.maker = maker;
        escrow.mint_a = mint_a;
        escrow.mint_b = mint_b;
        escrow.receive_amout = receive_amout;
        escrow.bump = bump;
    }
}
