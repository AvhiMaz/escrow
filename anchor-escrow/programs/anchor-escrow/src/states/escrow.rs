use anchor_lang::prelude::*;

#[account]
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,
    pub mint_b: Pubkey,
    pub recieve: u64,
    pub seed: u64,
    pub bump: u8,
}
