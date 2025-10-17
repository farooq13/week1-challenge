use anchor_lang::prelude::*;

declare_id!("6TqrJpUi8Y9FAS9a5Zns2Ch8hBj1g45iEihTtF3gCqg5");

#[program]
pub mod vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
