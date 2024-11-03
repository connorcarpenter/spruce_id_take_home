use std::collections::HashMap;

use ring::{rand::{SecureRandom, SystemRandom}, signature::{UnparsedPublicKey, ED25519}, };
use thiserror::Error;

use shared::{HolderChallengeRequest, HolderVerifyRequest, VerifierChallengeResponse, VerifierVerifyResponse, HolderRegisterRequest, VerifierRegisterResponse, UserId};

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
        let public_key_bytes = request.public_key_bytes();

        let new_user_id = UserId::new_random();
        let new_user = User::new(public_key_bytes.to_vec());
        self.users.insert(new_user_id, new_user);

        Ok(VerifierRegisterResponse::new(new_user_id))
    }

    pub fn recv_challenge_request(&mut self, request: HolderChallengeRequest) -> Result<VerifierChallengeResponse, VerifierError> {
        let user_id = request.user_id();

        // Check if the user is registered
        if let Some(user) = self.users.get_mut(&user_id) {

            // Generate a secure random nonce
            let mut nonce = vec![0u8; 16];
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
        let signature_bytes = request.signature_bytes();

        // Retrieve the User associated with the UserId
        let user = self
            .users
            .get(&user_id)
            .ok_or(VerifierError::UserNotFound)?;

        // Get the Public Key
        let public_key_bytes = user.public_key();
        let public_key =
            UnparsedPublicKey::new(&ED25519, public_key_bytes);

        // Get the Nonce
        let Some(nonce_bytes) = user.nonce() else {
            return Err(VerifierError::UserNeverChallenged);
        };

        // Verify the signature over the nonce
        public_key
            .verify(nonce_bytes, signature_bytes)
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
    public_key: Vec<u8>,
    nonce: Option<Vec<u8>>,
}

impl User {
    fn new(public_key: Vec<u8>) -> Self {
        Self {
            public_key,
            nonce: None,
        }
    }

    fn public_key(&self) -> &[u8] {
        self.public_key.as_ref()
    }

    fn nonce(&self) -> Option<&[u8]> {
        self.nonce.as_deref()
    }

    fn set_nonce(&mut self, nonce_bytes: Vec<u8>) {
        self.nonce = Some(nonce_bytes);
    }
}