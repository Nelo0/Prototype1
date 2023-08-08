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

    pub fn send_lamports(ctx: Context<SendLamports>, amount_lamports: u64) -> Result<()> {
        msg!("Sending {} Lamports to {}", amount_lamports, ctx.accounts.receiver.key());
        let sender = ctx.accounts.sending_wallet.to_account_info();
        
        if **sender.try_borrow_lamports()? < amount_lamports {
            return err!(WalletError::InsufficientFundsForTransaction);
        }

        **sender.try_borrow_mut_lamports()? -= amount_lamports;
        **ctx.accounts.receiver.try_borrow_mut_lamports()? += amount_lamports;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"wallet", initializer.key().as_ref()],
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
        seeds = [b"wallet", initializer.key().as_ref()],
        bump,
        close = initializer
    )]
    pub wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub initializer: Signer<'info>
}

#[derive(Accounts)]
pub struct SendLamports<'info> {
    #[account(
        mut,
        seeds = [b"wallet", initializer.key().as_ref()],
        bump,
        signer
    )]
    pub sending_wallet: Account<'info, Wallet>,
    
    /// CHECK: Receiver account does not need to be checked
    #[account(mut)]
    pub receiver: AccountInfo<'info>,
    #[account(mut)]
    pub initializer: Signer<'info>,
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


#[error_code]
pub enum WalletError {
    #[msg("Transaction fail")]
    TransactionFail,

    #[msg("Insufficent funds for the transaction; fail")]
    InsufficientFundsForTransaction
}