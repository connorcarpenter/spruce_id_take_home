use uuid::Uuid;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct UserId {
    uuid: Uuid,
}

impl UserId {
    pub fn new_random() -> Self {
        Self {
            uuid: Uuid::new_v4(),
        }
    }
}

// HolderRegisterRequest

pub struct HolderRegisterRequest {
    public_key: PublicKey,
}

impl HolderRegisterRequest {
    pub fn new(public_key: PublicKey) -> Self {
        Self {
            public_key,
        }
    }

    pub fn public_key(&self) -> &PublicKey {
        &self.public_key
    }
}

// VerifierRegisterResponse

pub struct VerifierRegisterResponse {
    user_id: UserId,
}

impl VerifierRegisterResponse {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}

// HolderChallengeRequest

pub struct HolderChallengeRequest {
    user_id: UserId,
}

impl HolderChallengeRequest {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }
}

// VerifierChallengeResponse

pub struct VerifierChallengeResponse {
    nonce: Nonce,
}

impl VerifierChallengeResponse {
    pub fn new(nonce: Nonce) -> Self {
        Self {
            nonce,
        }
    }

    pub fn nonce(&self) -> &Nonce {
        &self.nonce
    }
}

// HolderVerifyRequest

pub struct HolderVerifyRequest {
    user_id: UserId,
    signature: Signature,
}

impl HolderVerifyRequest {
    pub fn new(user_id: UserId, signature: Signature) -> Self {
        Self {
            user_id,
            signature,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn signature(&self) -> &Signature {
        &self.signature
    }
}

// VerifierVerifyResponse

pub struct VerifierVerifyResponse {
    success: bool,
}

impl VerifierVerifyResponse {
    pub fn new(success: bool) -> Self {
        Self {
            success,
        }
    }

    pub fn is_success(&self) -> bool {
        self.success
    }
}

// Types

pub type Nonce = [u8; 32];
pub type PublicKey = [u8; 32];
pub type Signature = [u8; 64];