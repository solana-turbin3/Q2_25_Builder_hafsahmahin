
use anchor_lang::prelude::*;
use crate::error::CustomError;
use crate::state::{Job, JobStatus};

pub fn submit_work(ctx: Context<SubmitWork>, work_uri: String) -> Result<()> {
    let job = &mut ctx.accounts.job;
    require!(
        job.status == JobStatus::Assigned,
        CustomError::JobNotAssigned
    );
    require!(
        job.freelancer == Some(ctx.accounts.freelancer.key()),
        CustomError::Unauthorized
    );
    job.job_uri = work_uri;
    job.status = JobStatus::Completed;
    job.completed_at = Some(Clock::get()?.unix_timestamp);
    Ok(())
}

#[derive(Accounts)]
pub struct SubmitWork<'info> {
    #[account(mut)]
    pub job: Account<'info, Job>,
    #[account(mut)]
    pub freelancer: Signer<'info>,
}
