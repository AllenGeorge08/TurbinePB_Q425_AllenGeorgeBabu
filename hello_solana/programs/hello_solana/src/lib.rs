use anchor_lang::prelude::*;

declare_id!("3L2UUNCduXAZsk8qtKtaEewnQPww8NMVrsowXa5nnUwP");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, name: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.greeting = format!("Hello, {}!", name);
        greeting_account.name = name;
        Ok(())
    }

    pub fn greet(ctx: Context<UpdateGreeting>, new_name: String) -> Result<()> {
        let greeting_account = &mut ctx.accounts.greeting_account;
        greeting_account.greeting = format!("Hello, {}!", new_name);
        greeting_account.name = new_name;
        msg!("Greeting Updated: {}", greeting_account.greeting);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + 32 + 200,
        seeds = [b"greeting", name.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub greeting_account: Account<'info, GreetingAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateGreeting<'info> {
    #[account(
        mut,
        seeds = [b"greeting", greeting_account.name.as_bytes(), user.key().as_ref()],
        bump
    )]
    pub greeting_account: Account<'info, GreetingAccount>,

    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct GreetingAccount {
    pub name: String,
    pub greeting: String,
}
