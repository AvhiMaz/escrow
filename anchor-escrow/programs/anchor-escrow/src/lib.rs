#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;
use instructions::*;

mod instructions;
mod states;

declare_id!("CoY5d32Ym1WvRsM7ZxcJDMWiixusxNCtWNu1wsJANeGq");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn make(ctx: Context<Make>, seed: u64, receive_amount: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive_amount, ctx.bumps)?;

        Ok(())
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit()?;
        ctx.accounts.withdraw_and_close_vault()?;
        Ok(())
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.refund_and_close_vault()?;
        Ok(())
    }
}
