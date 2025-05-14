use anchor_lang::prelude::*;

#[error_code]
pub enum CustomError {
    #[msg("Unauthorized: Only the client can assign a freelancer or submit work, or only client/freelancer can leave review.")]
    Unauthorized,
    #[msg("Job is not open for assignment.")]
    JobNotOpen,
    #[msg("Job is not assigned to a freelancer.")]
    JobNotAssigned,
    #[msg("Job is not completed.")]
    JobNotCompleted,
}
