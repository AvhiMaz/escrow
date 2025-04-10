#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use instructions::*;

mod instructions;
mod states;

declare_id!("CoY5d32Ym1WvRsM7ZxcJDMWiixusxNCtWNu1wsJANeGq");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64, deposit: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}
