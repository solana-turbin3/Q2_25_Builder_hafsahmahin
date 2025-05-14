// /Users/mansoorsm/Downloads/chaingig/programs/chaingig/state.rs

use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub authority: Pubkey,
    pub profile_uri: String,
    pub is_freelancer: bool,
    pub reputation: u64,
    pub jobs_posted: u64,
    pub jobs_completed: u64,
}

#[account]
pub struct Job {
    pub job_id: u64,
    pub client: Pubkey,
    pub freelancer: Option<Pubkey>,
    pub job_uri: String,
    pub budget: u64,
    pub status: JobStatus,
    pub created_at: i64,
    pub completed_at: Option<i64>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum JobStatus {
    Open,
    Assigned,
    Completed,
    Cancelled,
}

#[account]
pub struct Review {
    pub review_id: u64,
    pub job_id: u64,
    pub reviewer: Pubkey,
    pub reviewee: Pubkey,
    pub rating: u8,
    pub review_uri: String,
}
