use anchor_lang::prelude::*;

declare_id!("3o7GmW2VxQWJ6gFPWyUajWYXpXt1ic46tpVBNGrzyXxF");

#[program]
pub mod counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = ctx.accounts.user.key();
        counter.bump = ctx.bumps.counter;
        msg!("Counter initialized, authority: {}", counter.authority);
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = counter
            .count
            .checked_add(1)
            .ok_or(CounterError::Overflow)?;
        msg!("Counter is now {}", counter.count);
        Ok(())
    }
}

// Step 2: the account. 8 (discriminator) + 8 + 32 + 1 = 49 bytes
#[account]
#[derive(InitSpace)]
pub struct Counter {
    pub count: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

// Step 3: creates one counter per wallet at a PDA
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + Counter::INIT_SPACE,
        seeds = [b"counter", user.key().as_ref()],
        bump
    )]
    pub counter: Account<'info, Counter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

// Only the owner of the counter can increment it
#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(
        mut,
        seeds = [b"counter", authority.key().as_ref()],
        bump = counter.bump,
        has_one = authority
    )]
    pub counter: Account<'info, Counter>,
    pub authority: Signer<'info>,
}

// Step 4: errors and safe math
#[error_code]
pub enum CounterError {
    #[msg("Counter would overflow")]
    Overflow,
}
