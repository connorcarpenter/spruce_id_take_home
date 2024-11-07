use std::collections::HashMap;

use log::info;
use ring::{rand::{SecureRandom, SystemRandom}, signature::{UnparsedPublicKey, ED25519}, };

use shared::{HolderChallengeRequest, VerifierVerifyResponse, HolderVerifyRequest, VerifierChallengeResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId, ToBase64};

use crate::{error::VerifierError, user::User};

pub struct Verifier {
    rng: SystemRandom,
    users: HashMap<UserId, User>,
}

impl Verifier {
    pub fn new() -> Self {
        Self {
            rng: SystemRandom::new(),
            users: HashMap::new(),
        }
    }

    pub fn recv_register_request(&mut self, request: HolderRegisterRequest) -> Result<VerifierRegisterResponse, VerifierError> {

        let public_key = request.public_key().clone();
        info!("Verifier receives HolderRegisterRequest with Public Key: {:?}", public_key.to_base64());

        info!("Verifier creates a new User for the Holder, assigns a randomly generated UUID as a \"UserId\", and stores the Holder's Public Key for later use");

        let new_user_id = UserId::new_random();
        let new_user = User::new(public_key);
        self.users.insert(new_user_id, new_user);

        info!("Verifier creates a VerifierRegisterResponse with the assigned {:?}", new_user_id);

        Ok(VerifierRegisterResponse::new(new_user_id))
    }

    pub fn recv_challenge_request(&mut self, request: HolderChallengeRequest) -> Result<VerifierChallengeResponse, VerifierError> {

        let user_id = request.user_id();
        info!("Verifier receives HolderChallengeRequest with {:?}", user_id);

        // Check if the user is registered
        info!("Verifier checks if the UserId is registered");

        if let Some(user) = self.users.get_mut(&user_id) {

            // Generate a secure random nonce
            info!("Verifier generates a random nonce and stores it in the User");

            let mut nonce = [0u8; 32];
            self.rng
                .fill(&mut nonce)
                .map_err(|_| VerifierError::NonceGenerationFailed)?;

            // Store last nonce in User
            user.add_nonce(nonce.clone());

            info!("Verifier creates a VerifierChallengeResponse with the User's generated nonce: {:?}", nonce.to_base64());

            Ok(VerifierChallengeResponse::new(nonce))
        } else {
            Err(VerifierError::UserNotFound)
        }
    }

    pub fn recv_verify_request(&mut self, request: HolderVerifyRequest) -> Result<VerifierVerifyResponse, VerifierError> {

        let user_id = request.user_id();
        let nonce = request.nonce();
        let signature = request.signature();
        info!("Verifier receives HolderVerifyRequest with {:?}, nonce: {:?}, and signature {:?}", user_id, nonce.to_base64(), signature.to_base64());

        // Retrieve the User associated with the UserId
        let user = self
            .users
            .get_mut(&user_id)
            .ok_or(VerifierError::UserNotFound)?;

        // Get the Public Key
        let public_key = user.public_key().clone();
        let ed25519_public_key =
            UnparsedPublicKey::new(&ED25519, public_key);

        // Take the User's Nonce, destroying it
        let Some(nonce) = user.take_nonce(&nonce) else {
            return Err(VerifierError::UserNonceNoMatch);
        };

        info!(
            "Verifier uses UserId to retrieve the PublicKey associated: {:?}, and the user's last issued nonce: {:?}",
            public_key.to_base64(),
            nonce.to_base64()
        );

        // Verify the signature over the nonce
        info!("Verifier verifies the signature over the nonce with the PublicKey");

        match ed25519_public_key.verify(&nonce, signature.as_ref()) {
            Ok(_) => {
                info!("Verifier creates a VerifierVerifyResponse indicating verification success!");
                return Ok(VerifierVerifyResponse::new(true));
            },
            Err(_) => {
                info!("Verifier creates a VerifierVerifyResponse indicating verification failure!");
                return Ok(VerifierVerifyResponse::new(false));
            }
        }
    }
}