
use anchor_lang::prelude::*;
use crate::state::User;

pub fn register_user(
    ctx: Context<RegisterUser>,
    profile_uri: String,
    is_freelancer: bool,
) -> Result<()> {
    let user = &mut ctx.accounts.user;
    user.authority = *ctx.accounts.authority.key;
    user.profile_uri = profile_uri;
    user.is_freelancer = is_freelancer;
    user.reputation = 0;
    user.jobs_posted = 0;
    user.jobs_completed = 0;
    Ok(())
}

#[derive(Accounts)]
pub struct RegisterUser<'info> {
    #[account(init, payer = authority, space = 8 + 32 + 4 + 200 + 1 + 8 + 8 + 8)]
    pub user: Account<'info, User>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
