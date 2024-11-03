use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierError, VerifierChallengeResponse, VerifierVerifyResponse};

pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    pub fn recv_challenge_request(&mut self, request: HolderChallengeRequest) -> Result<VerifierChallengeResponse, VerifierError> {
        todo!()
    }

    pub fn recv_verify_request(&mut self, request: HolderVerifyRequest) -> Result<VerifierVerifyResponse, VerifierError> {
        todo!()
    }
}