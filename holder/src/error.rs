use thiserror::Error;

#[derive(Error, Debug)]
pub enum HolderError {
    #[error("Registration incomplete.")]
    RegistrationIncomplete,

    #[error("Verification failed.")]
    VerificationFailed,
}
