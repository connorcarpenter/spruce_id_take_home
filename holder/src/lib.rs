use shared::{HolderError, HolderRequest, HolderRequest2, VerifierResponse};

pub struct Holder;

impl Holder {

}

impl Holder {
    pub fn new() -> Self {
        Self
    }

    pub fn request_create(&self) -> HolderRequest {
        todo!()
    }

    pub fn recv_response(&mut self, verifier_response: VerifierResponse) -> Result<HolderRequest2, HolderError> {
        todo!()
    }
}