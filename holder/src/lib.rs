use thiserror::Error;

use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId};

// Holder

pub struct Holder {
    user_id: Option<UserId>,
}

impl Holder {

    // TODO: Should generate keys here
    pub fn new() -> Self {
        Self {
            user_id: None,
        }
    }

    // User Registration

    pub fn create_register_request(&mut self) -> HolderRegisterRequest {
        todo!()
    }

    pub fn recv_register_response(&mut self, response: VerifierRegisterResponse) -> Result<(), HolderError> {
        todo!()
    }

    // Challenge / Verification

    pub fn create_challenge_request(&self) -> Result<HolderChallengeRequest, HolderError> {
        let Some(user_id) = self.user_id else {
            return Err(HolderError::RegistrationIncomplete);
        };
        todo!()
    }

    pub fn recv_challenge_response(&mut self, response: VerifierChallengeResponse) -> Result<HolderVerifyRequest, HolderError> {
        todo!()
    }
}

// HolderError

#[derive(Error, Debug)]
pub enum HolderError {
    #[error("Placeholder error occurred in Holder.")]
    Placeholder,
    #[error("Registration incomplete.")]
    RegistrationIncomplete,
}