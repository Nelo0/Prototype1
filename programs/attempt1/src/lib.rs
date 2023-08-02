use anchor_lang::prelude::*;

declare_id!("4yhK57aLadt8wc9rmqF4QWDhVHQnkCMwXP8V6H9netGZ");

#[program]
pub mod attempt1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
