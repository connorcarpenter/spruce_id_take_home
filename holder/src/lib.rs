
use ring::{signature::Ed25519KeyPair, rand::SystemRandom};
use thiserror::Error;

use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId};

// Holder

pub struct Holder {
    user_id: Option<UserId>,
    key_pair: Ed25519KeyPair,
}

impl Holder {

    // TODO: Should generate keys here
    pub fn new() -> Self {

        let rng = SystemRandom::new();
        let pkcs8_bytes =
            Ed25519KeyPair::generate_pkcs8(&rng).expect("Failed to generate key pair");
        let key_pair =
            Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).expect("Failed to parse key pair");

        Self {
            user_id: None,
            key_pair
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