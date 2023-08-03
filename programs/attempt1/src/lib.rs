use anchor_lang::prelude::*;
use anchor_spl::token::Transfer;
//use solana_program::system_instruction::transfer;

declare_id!("4yhK57aLadt8wc9rmqF4QWDhVHQnkCMwXP8V6H9netGZ");

#[program]
pub mod attempt1 {
    use anchor_lang::solana_program::system_instruction;

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

    pub fn send_sol(ctx: Context<SendLamports>, amount: u64) -> Result<()> {
        msg!("Sending {} Lamports to {}", amount, ctx.accounts.receiver.key());

        let from_account = &ctx.accounts.sender;
        let to_account = &ctx.accounts.receiver;

        let transfer_instruction = system_instruction::transfer(from_account.key, to_account.key, amount);

        anchor_lang::solana_program::program::invoke_signed(
            &transfer_instruction,
            &[
                from_account.to_account_info(),
                to_account.clone(),
                ctx.accounts.system_program.to_account_info(),
            ],
            &[],
        )?;

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

#[derive(Accounts)]
pub struct SendLamports<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
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
