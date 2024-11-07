use std::collections::HashMap;

use chrono::{DateTime, Utc};

use shared::{Nonce, PublicKey};

use crate::NONCE_EXPIRATION_SECS;

pub(crate) struct User {
    public_key: PublicKey,
    nonces: HashMap<Nonce, DateTime<Utc>>,
}

impl User {
    pub(crate) fn new(public_key: PublicKey) -> Self {
        Self {
            public_key,
            nonces: HashMap::new(),
        }
    }

    pub(crate) fn public_key(&self) -> &PublicKey {
        &self.public_key
    }

    pub(crate) fn add_nonce(&mut self, nonce: Nonce) {
        let now = Utc::now();
        self.cleanup_expired_nonces(&now);

        self.nonces.insert(nonce, Utc::now());
    }

    pub(crate) fn take_nonce(&mut self, nonce: &Nonce) -> Option<Nonce> {
        let timestamp = self.nonces.remove(nonce)?;
        let now = Utc::now();
        if timestamp_expired(&now, &timestamp) {
            return None;
        }
        return Some(nonce.clone());
    }

    fn cleanup_expired_nonces(&mut self, now: &DateTime<Utc>) {
        self.nonces
            .retain(|_, creation_time| !timestamp_expired(now, creation_time));
    }
}

fn timestamp_expired(now: &DateTime<Utc>, timestamp: &DateTime<Utc>) -> bool {
    (*now - *timestamp).num_seconds() > NONCE_EXPIRATION_SECS as i64
}
