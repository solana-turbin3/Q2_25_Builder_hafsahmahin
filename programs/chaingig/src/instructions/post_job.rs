use anchor_lang::prelude::*;
use crate::state::{Job, JobStatus};

pub fn post_job(ctx: Context<PostJob>, job_id: u64, job_uri: String, budget: u64) -> Result<()> {
    let job = &mut ctx.accounts.job;
    job.job_id = job_id;
    job.client = *ctx.accounts.client.key;
    job.freelancer = None;
    job.job_uri = job_uri;
    job.budget = budget;
    job.status = JobStatus::Open;
    job.created_at = Clock::get()?.unix_timestamp;
    job.completed_at = None;
    Ok(())
}

#[derive(Accounts)]
pub struct PostJob<'info> {
    #[account(init, payer = client, space = 8 + 8 + 32 + 1 + 4 + 200 + 8 + 1 + 8 + 9)]
    pub job: Account<'info, Job>,
    #[account(mut)]
    pub client: Signer<'info>,
    pub system_program: Program<'info, System>,
}
