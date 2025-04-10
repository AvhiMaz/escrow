use anchor_lang::prelude::*;

declare_id!("CoY5d32Ym1WvRsM7ZxcJDMWiixusxNCtWNu1wsJANeGq");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
