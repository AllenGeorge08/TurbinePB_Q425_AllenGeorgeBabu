use anchor_lang::prelude::*;


declare_id!("BrNGrZQhHz2YT1BBEgiS3APm6JJud7jhBs79gmk2Tx5Q");

mod states;
use crate::states::*;

#[program]
pub mod cinepass {
    use super::*;

    pub fn initialize(ctx: Context<InitializeCounter>) -> Result<()> {
        ctx.accounts.counter.nft_count = 0 ;
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn increment(ctx: Context<Count>) -> Result<()>{
        ctx.accounts.counter.nft_count =  ctx.accounts.counter.nft_count.checked_add(1).unwrap();
        msg!("Counter Incremented");
        Ok(())
    }

    pub fn decrement(ctx: Context<Count>) -> Result<()>{
        ctx.accounts.counter.nft_count = ctx.accounts.counter.nft_count.checked_sub(1).unwrap();
        msg!("Counter Decremented");
        Ok(())
    }
}
