use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize {}

pub fn handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Testing Cowshit: {:?}", ctx.program_id);
    Ok(())
}
