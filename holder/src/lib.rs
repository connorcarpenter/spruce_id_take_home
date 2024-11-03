
use ring::{signature::{Ed25519KeyPair, KeyPair}, rand::SystemRandom};
use thiserror::Error;

use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId};

// Holder

pub struct Holder {
    user_id: Option<UserId>,
    key_pair: Ed25519KeyPair,
}

impl Holder {

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
        let public_key_bytes = self.key_pair.public_key().as_ref().to_vec();
        HolderRegisterRequest::new(public_key_bytes)
    }

    pub fn recv_register_response(&mut self, response: VerifierRegisterResponse) -> Result<(), HolderError> {
        let user_id = response.user_id();
        self.user_id = Some(user_id);
        Ok(())
    }

    // Challenge / Verification

    pub fn create_challenge_request(&self) -> Result<HolderChallengeRequest, HolderError> {
        if let Some(user_id) = self.user_id {
            Ok(HolderChallengeRequest::new(user_id))
        } else {
            Err(HolderError::RegistrationIncomplete)
        }
    }

    pub fn recv_challenge_response(&mut self, response: VerifierChallengeResponse) -> Result<HolderVerifyRequest, HolderError> {
        let nonce_bytes = response.nonce_bytes();

        // Ensure the Holder has a UserId
        let user_id = self
            .user_id
            .ok_or(HolderError::RegistrationIncomplete)?;

        // Sign the nonce using the Holder's private key
        let signature = self.key_pair.sign(nonce_bytes);
        let signature_bytes = signature.as_ref().to_vec();

        Ok(HolderVerifyRequest::new(user_id, signature_bytes))
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