use anchor_lang::prelude::*;

declare_id!("Au3vHfK6t3Kc7iMV88xtSBpZiWKDYEqjgxrXn4fXTSuo");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
