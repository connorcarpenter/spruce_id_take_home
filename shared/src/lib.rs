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
    public_key_bytes: Vec<u8>,
}

impl HolderRegisterRequest {
    pub fn new(public_key_bytes: Vec<u8>) -> Self {
        Self {
            public_key_bytes,
        }
    }

    pub fn public_key_bytes(&self) -> &[u8] {
        self.public_key_bytes.as_ref()
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
    nonce_bytes: Vec<u8>,
}

impl VerifierChallengeResponse {
    pub fn new(nonce_bytes: Vec<u8>) -> Self {
        Self {
            nonce_bytes,
        }
    }

    pub fn nonce_bytes(&self) -> &[u8] {
        self.nonce_bytes.as_ref()
    }
}

// HolderVerifyRequest

pub struct HolderVerifyRequest {
    user_id: UserId,
    signature_bytes: Vec<u8>,
}

impl HolderVerifyRequest {
    pub fn new(user_id: UserId, signature_bytes: Vec<u8>) -> Self {
        Self {
            user_id,
            signature_bytes,
        }
    }

    pub fn user_id(&self) -> UserId {
        self.user_id
    }

    pub fn signature_bytes(&self) -> &[u8] {
        self.signature_bytes.as_ref()
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