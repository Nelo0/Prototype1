use anchor_lang::prelude::*;

declare_id!("4yhK57aLadt8wc9rmqF4QWDhVHQnkCMwXP8V6H9netGZ");

#[program]
pub mod attempt1 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        wallet.initializer = ctx.accounts.initializer.key();
        
        msg!("Account created");
        Ok(())
    }

    pub fn close_account(_ctx: Context<CloseAccount>) -> Result<()> {
        msg!("Account closed");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = Wallet::get_space()
    )]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct CloseAccount<'info> {
    #[account(
        mut,
        seeds = [initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub initializer: Signer<'info>
}

#[account]
pub struct Wallet {
    pub initializer: Pubkey
}

impl Wallet {
    pub fn get_space() -> usize {
        8           // anchor discriminator
        + 32        // initializer
    }
}
