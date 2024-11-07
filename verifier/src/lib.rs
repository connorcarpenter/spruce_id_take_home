use std::collections::HashMap;

use log::info;
use ring::{rand::{SecureRandom, SystemRandom}, signature::{UnparsedPublicKey, ED25519}, };
use thiserror::Error;

use shared::{HolderChallengeRequest, VerifierVerifyResponse, HolderVerifyRequest, VerifierChallengeResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId, Nonce, PublicKey, ToBase64};

// Verifier

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
            user.set_nonce(nonce.clone());

            // TODO: store the nonce to prevent replay attacks?

            info!("Verifier creates a VerifierChallengeResponse with the User's generated nonce: {:?}", nonce.to_base64());

            Ok(VerifierChallengeResponse::new(nonce))
        } else {
            Err(VerifierError::UserNotFound)
        }
    }

    pub fn recv_verify_request(&mut self, request: HolderVerifyRequest) -> Result<VerifierVerifyResponse, VerifierError> {

        let user_id = request.user_id();
        let signature = request.signature();
        info!("Verifier receives HolderVerifyRequest with {:?}, and Signature {:?}", user_id, signature.to_base64());

        // Retrieve the User associated with the UserId
        let user = self
            .users
            .get(&user_id)
            .ok_or(VerifierError::UserNotFound)?;

        // Get the Public Key
        let public_key = user.public_key();
        let ed25519_public_key =
            UnparsedPublicKey::new(&ED25519, public_key);

        // Get the Nonce
        let Some(nonce) = user.nonce() else {
            return Err(VerifierError::UserNeverChallenged);
        };

        info!(
            "Verifier uses UserId to retrieve the PublicKey associated: {:?}, and the user's last issued nonce: {:?}",
            public_key.to_base64(),
            nonce.to_base64()
        );

        // Verify the signature over the nonce
        info!("Verifier verifies the signature over the nonce with the PublicKey");

        match ed25519_public_key.verify(nonce, signature.as_ref()) {
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

// VerifierError

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("User not found")]
    UserNotFound,

    #[error("Failed to generate nonce")]
    NonceGenerationFailed,

    #[error("Invalid nonce")]
    InvalidNonce,

    #[error("Signature verification failed")]
    VerificationFailed,

    #[error("User never sent a challenge request, generating a nonce")]
    UserNeverChallenged,
}

// User

struct User {
    public_key: PublicKey,
    nonce: Option<Nonce>,
}

impl User {
    fn new(public_key: PublicKey) -> Self {
        Self {
            public_key,
            nonce: None,
        }
    }

    fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    fn nonce(&self) -> Option<&Nonce> {
        self.nonce.as_ref()
    }

    fn set_nonce(&mut self, nonce: Nonce) {
        self.nonce = Some(nonce);
    }
}