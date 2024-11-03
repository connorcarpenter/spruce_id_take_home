use shared::{HolderRequest, HolderRequest2, VerifierError, VerifierResponse, VerifierResponse2};

pub struct Verifier;

impl Verifier {
    pub fn new() -> Self {
        Self
    }

    pub fn recv_request(&mut self, request: HolderRequest) -> Result<VerifierResponse, VerifierError> {
        todo!()
    }

    pub fn recv_request_2(&mut self, p0: HolderRequest2) -> Result<VerifierResponse2, VerifierError> {
        todo!()
    }
}