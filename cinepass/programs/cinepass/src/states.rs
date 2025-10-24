use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeCounter<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        space = 8 + 8,
        seeds = [b"counter",signer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info,Counter>,
    pub system_program: Program<'info,System>,
}


#[derive(Accounts)]
pub struct Count<'info>{
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"counter",signer.key().as_ref()],
        bump,
    )]
    pub counter: Account<'info,Counter>,
    pub system_program: Program<'info,System>
}

#[account]
pub struct Counter{
    pub nft_count: u64,
}