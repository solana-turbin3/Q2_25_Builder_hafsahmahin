use anchor_lang::prelude::*;
use crate::error::CustomError;
use crate::state::{Job, JobStatus};

pub fn assign_freelancer(ctx: Context<AssignFreelancer>, freelancer: Pubkey) -> Result<()> {
    let job = &mut ctx.accounts.job;
    require_keys_eq!(
        job.client,
        ctx.accounts.client.key(),
        CustomError::Unauthorized
    );
    require!(job.status == JobStatus::Open, CustomError::JobNotOpen);
    job.freelancer = Some(freelancer);
    job.status = JobStatus::Assigned;
    Ok(())
}

#[derive(Accounts)]
pub struct AssignFreelancer<'info> {
    #[account(mut)]
    pub job: Account<'info, Job>,
    #[account(mut)]
    pub client: Signer<'info>,
}
