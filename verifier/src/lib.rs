use thiserror::Error;

use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse, VerifierVerifyResponse, HolderRegisterRequest, VerifierRegisterResponse};

// Verifier

pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    pub fn recv_register_request(&mut self, request: HolderRegisterRequest) -> Result<VerifierRegisterResponse, VerifierError> {
        todo!()
    }

    pub fn recv_challenge_request(&mut self, request: HolderChallengeRequest) -> Result<VerifierChallengeResponse, VerifierError> {
        todo!()
    }

    pub fn recv_verify_request(&mut self, request: HolderVerifyRequest) -> Result<VerifierVerifyResponse, VerifierError> {
        todo!()
    }
}

// VerifierError

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Placeholder error occurred in Holder.")]
    Placeholder,
}