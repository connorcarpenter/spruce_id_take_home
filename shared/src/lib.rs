use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UserId {
    uuid: Uuid,
}

impl UserId {
    pub fn new() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

// HolderRegisterRequest

pub struct HolderRegisterRequest;

impl HolderRegisterRequest {
    pub fn new() -> Self {
        Self
    }
}

// VerifierRegisterResponse

pub struct VerifierRegisterResponse;

impl VerifierRegisterResponse {
    pub fn new() -> Self {
        Self
    }
}

// HolderChallengeRequest

pub struct HolderChallengeRequest;

impl HolderChallengeRequest {
    pub fn new() -> Self {
        Self
    }
}

// VerifierChallengeResponse

pub struct VerifierChallengeResponse;

impl VerifierChallengeResponse {
    pub fn new() -> Self {
        Self
    }
}

// HolderVerifyRequest

pub struct HolderVerifyRequest;

impl HolderVerifyRequest {
    pub fn new() -> Self {
        Self
    }
}

// VerifierVerifyResponse

pub struct VerifierVerifyResponse;

impl VerifierVerifyResponse {
    pub fn new() -> Self {
        Self
    }
}