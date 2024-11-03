use shared::{HolderError, HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse};

pub struct Holder;

impl Holder {

}

impl Holder {
    pub fn new() -> Self {
        Self
    }

    pub fn create_challenge_request(&self) -> HolderChallengeRequest {
        todo!()
    }

    pub fn recv_challenge_response(&mut self, response: VerifierChallengeResponse) -> Result<HolderVerifyRequest, HolderError> {
        todo!()
    }
}