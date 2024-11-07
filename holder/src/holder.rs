use log::info;
use ring::{signature::{Ed25519KeyPair, KeyPair}, rand::SystemRandom};

use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId, PublicKey, VerifierVerifyResponse, ToBase64, Signature};

use crate::error::HolderError;

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

    pub fn create_register_request(&self) -> HolderRegisterRequest {

        let public_key: PublicKey = self.key_pair.public_key().as_ref().try_into().expect("This should never fail, because ring::signature::PublicKey is always 32 bytes.");

        info!("Holder creates HolderRegisterRequest which includes the Holder's Public Key: {:?}", public_key.to_base64());
        HolderRegisterRequest::new(public_key)
    }

    pub fn recv_register_response(&mut self, response: VerifierRegisterResponse) -> Result<(), HolderError> {

        let user_id = response.user_id();
        info!("Holder receives VerifierRegisterResponse with {:?}", user_id);

        info!("Holder stores UserId for future use");

        self.user_id = Some(user_id);

        Ok(())
    }

    // Challenge

    pub fn create_challenge_request(&self) -> Result<HolderChallengeRequest, HolderError> {

        if let Some(user_id) = self.user_id {

            info!("Holder creates HolderChallengeRequest which includes the Holder's {:?}", user_id);

            Ok(HolderChallengeRequest::new(user_id))
        } else {
            Err(HolderError::RegistrationIncomplete)
        }
    }

    pub fn recv_challenge_response(&self, response: VerifierChallengeResponse) -> Result<HolderVerifyRequest, HolderError> {

        let nonce = response.nonce().clone();
        info!("Holder receives VerifierChallengeResponse with nonce: {:?}", nonce.to_base64());

        // Ensure the Holder has a UserId
        let user_id = self
            .user_id
            .ok_or(HolderError::RegistrationIncomplete)?;

        // Sign the nonce using the Holder's private key

        let signature: Signature = self.key_pair.sign(&nonce).as_ref().try_into().expect("This should never fail, because ring::signature::Signature is always 64 bytes.");
        info!("Holder signs the nonce with the Holder's private key, creating a signature: {:?}", signature.to_base64());

        info!("Holder creates HolderVerifyRequest which includes the Holder's UserId and the signature");

        Ok(HolderVerifyRequest::new(user_id, nonce, signature))
    }

    pub fn recv_verify_response(&self, response: VerifierVerifyResponse) -> Result<(), HolderError> {
        if response.is_success() {

            info!("Holder receives VerifierVerifyResponse indicating verification success!");

            Ok(())
        } else {
            Err(HolderError::VerificationFailed)
        }

    }
}