//! Versioned application envelopes shared by every transport.
//! Encryption is deliberately outside this crate: transports receive ciphertext only.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub const CURRENT_PROTOCOL_VERSION: u16 = 1;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolEnvelope {
    pub version: u16,
    pub message_id: Uuid,
    pub conversation_id: Uuid,
    pub sender_device_id: Uuid,
    pub message_type: MessageType,
    pub sequence: u64,
    pub created_at_ms: i64,
    /// Authenticated ciphertext. Plaintext never belongs in an envelope.
    pub ciphertext: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MessageType {
    Hello,
    Identity,
    Auth,
    Message,
    MessageEdit,
    MessageDelete,
    DeliveryAck,
    ReadAck,
    Typing,
    SyncRequest,
    SyncResponse,
    Error,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EnvelopeError {
    #[error("unsupported protocol version {0}")]
    UnsupportedVersion(u16),
    #[error("ciphertext must not be empty")]
    EmptyCiphertext,
}

impl ProtocolEnvelope {
    pub fn validate(&self) -> Result<(), EnvelopeError> {
        if self.version != CURRENT_PROTOCOL_VERSION {
            return Err(EnvelopeError::UnsupportedVersion(self.version));
        }
        if self.ciphertext.is_empty() {
            return Err(EnvelopeError::EmptyCiphertext);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn envelope() -> ProtocolEnvelope {
        ProtocolEnvelope {
            version: CURRENT_PROTOCOL_VERSION,
            message_id: Uuid::new_v4(),
            conversation_id: Uuid::new_v4(),
            sender_device_id: Uuid::new_v4(),
            message_type: MessageType::Message,
            sequence: 1,
            created_at_ms: 0,
            ciphertext: vec![1],
        }
    }
    #[test]
    fn accepts_current_ciphertext_envelope() {
        assert_eq!(envelope().validate(), Ok(()));
    }
    #[test]
    fn rejects_plaintext_placeholder() {
        let mut value = envelope();
        value.ciphertext.clear();
        assert_eq!(value.validate(), Err(EnvelopeError::EmptyCiphertext));
    }
}
