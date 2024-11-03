use std::collections::HashMap;

use ring::rand::SystemRandom;
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
        todo!()
    }

    pub fn recv_challenge_request(&mut self, request: HolderChallengeRequest) -> Result<VerifierChallengeResponse, VerifierError> {
        todo!()
    }

    pub fn recv_verify_request(&mut self, request: HolderVerifyRequest) -> Result<VerifierVerifyResponse, VerifierError> {
        todo!()
    }
}

// VerifierError

#[derive(Error, Debug)]
pub enum VerifierError {
    #[error("Placeholder error occurred in Holder.")]
    Placeholder,
}

// User

struct User {
    public_key: Vec<u8>,
}

impl User {
    fn new(public_key: Vec<u8>) -> Self {
        Self {
            public_key
        }
    }
}