use std::collections::HashMap;

use ring::{rand::{SecureRandom, SystemRandom}, signature::{UnparsedPublicKey, ED25519}, };
use thiserror::Error;

use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse, VerifierVerifyResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId, Nonce, PublicKey};

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

        let new_user_id = UserId::new_random();
        let new_user = User::new(public_key);
        self.users.insert(new_user_id, new_user);

        Ok(VerifierRegisterResponse::new(new_user_id))
    }

    pub fn recv_challenge_request(&mut self, request: HolderChallengeRequest) -> Result<VerifierChallengeResponse, VerifierError> {
        let user_id = request.user_id();

        // Check if the user is registered
        if let Some(user) = self.users.get_mut(&user_id) {

            // Generate a secure random nonce
            let mut nonce = [0u8; 32];
            self.rng
                .fill(&mut nonce)
                .map_err(|_| VerifierError::NonceGenerationFailed)?;

            // Store last nonce in User
            user.set_nonce(nonce.clone());

            // TODO: store the nonce to prevent replay attacks?
            Ok(VerifierChallengeResponse::new(nonce))
        } else {
            Err(VerifierError::UserNotFound)
        }
    }

    pub fn recv_verify_request(&mut self, request: HolderVerifyRequest) -> Result<VerifierVerifyResponse, VerifierError> {
        let user_id = request.user_id();
        let signature = request.signature();

        // Retrieve the User associated with the UserId
        let user = self
            .users
            .get(&user_id)
            .ok_or(VerifierError::UserNotFound)?;

        // Get the Public Key
        let public_key = user.public_key();
        let public_key =
            UnparsedPublicKey::new(&ED25519, public_key);

        // Get the Nonce
        let Some(nonce) = user.nonce() else {
            return Err(VerifierError::UserNeverChallenged);
        };

        // Verify the signature over the nonce
        public_key
            .verify(nonce, signature.as_ref())
            .map_err(|_| VerifierError::VerificationFailed)?;

        Ok(VerifierVerifyResponse::new(true))
    }
}

// VerifierError

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("User not found.")]
    UserNotFound,

    #[error("Failed to generate nonce.")]
    NonceGenerationFailed,

    #[error("Invalid nonce.")]
    InvalidNonce,

    #[error("Signature verification failed.")]
    VerificationFailed,

    #[error("User never sent a challenge request, generating a nonce.")]
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