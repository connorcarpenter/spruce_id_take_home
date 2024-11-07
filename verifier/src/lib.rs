mod user;
mod error;
mod verifier;

pub use error::VerifierError;
pub use verifier::Verifier;

pub(crate) const NONCE_EXPIRATION_SECS: u64 = 300; // 5 minutes