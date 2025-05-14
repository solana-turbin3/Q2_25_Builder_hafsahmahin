// /Users/mansoorsm/Downloads/chaingig/programs/chaingig/instructions/leave_review.rs

use anchor_lang::prelude::*;
use crate::error::CustomError;
use crate::state::{Job, JobStatus, Review};

pub fn leave_review(
    ctx: Context<LeaveReview>,
    review_id: u64,
    rating: u8,
    review_uri: String,
) -> Result<()> {
    let job = &ctx.accounts.job;
    require!(
        job.status == JobStatus::Completed,
        CustomError::JobNotCompleted
    );
    let reviewer = ctx.accounts.reviewer.key();
    require!(
        reviewer == job.client || Some(reviewer) == job.freelancer,
        CustomError::Unauthorized
    );
    let review = &mut ctx.accounts.review;
    review.review_id = review_id;
    review.job_id = job.job_id;
    review.reviewer = reviewer;
    review.reviewee = if reviewer == job.client {
        job.freelancer.unwrap()
    } else {
        job.client
    };
    review.rating = rating;
    review.review_uri = review_uri;
    Ok(())
}

#[derive(Accounts)]
pub struct LeaveReview<'info> {
    #[account(mut)]
    pub job: Account<'info, Job>,
    #[account(init, payer = reviewer, space = 8 + 8 + 8 + 32 + 32 + 1 + 4 + 200)]
    pub review: Account<'info, Review>,
    #[account(mut)]
    pub reviewer: Signer<'info>,
    pub system_program: Program<'info, System>,
}
