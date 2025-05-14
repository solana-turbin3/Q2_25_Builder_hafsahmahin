use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;

use instructions::{
    assign_freelancer::*, initialize::*, leave_review::*, post_job::*, register_user::*,
    submit_work::*,
};

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("E1C4egpG48nto7qgZ651mMpQmimvUrV9wJHuFyMYxNeo");

#[program]
pub mod chaingig {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }
    pub fn register_user(
        ctx: Context<RegisterUser>,
        profile_uri: String,
        is_freelancer: bool,
    ) -> Result<()> {
        instructions::register_user::register_user(ctx, profile_uri, is_freelancer)
    }
    pub fn post_job(
        ctx: Context<PostJob>,
        job_id: u64,
        job_uri: String,
        budget: u64,
    ) -> Result<()> {
        instructions::post_job::post_job(ctx, job_id, job_uri, budget)
    }
    pub fn assign_freelancer(ctx: Context<AssignFreelancer>, freelancer: Pubkey) -> Result<()> {
        instructions::assign_freelancer::assign_freelancer(ctx, freelancer)
    }
    pub fn submit_work(ctx: Context<SubmitWork>, work_uri: String) -> Result<()> {
        instructions::submit_work::submit_work(ctx, work_uri)
    }
    pub fn leave_review(
        ctx: Context<LeaveReview>,
        review_id: u64,
        rating: u8,
        review_uri: String,
    ) -> Result<()> {
        instructions::leave_review::leave_review(ctx, review_id, rating, review_uri)
    }
}


