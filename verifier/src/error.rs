use thiserror::Error;

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("User not found")]
    UserNotFound,

    #[error("Failed to generate nonce")]
    NonceGenerationFailed,

    #[error("Invalid nonce")]
    InvalidNonce,

    #[error("Signature verification failed")]
    VerificationFailed,

    #[error("User has no matching nonce. It could have expired or been used already")]
    UserNonceNoMatch,
}