use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum ProtoError {
    Signature,
    BadFormat(String),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum ProtoMessage {
    KeyExchange(ProtoKeys),
    Message(Vec<u8>),
    Error(ProtoError),
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProtoKeys {
    pub key: Option<[u8; 32]>,
    pub signature: Option<Signature>,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ProtoTransaction {
    pub uuid: Uuid,
    pub step: u8,
    pub data: ProtoMessage,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct Signature {
    pub key: [u8; 32],
    pub c: [u8; 32],
    pub z: [u8; 32],
}

impl ProtoTransaction {
    pub fn new_format_error(err: String) -> Self {
        ProtoTransaction {
            uuid: Default::default(),
            step: 0,
            data: ProtoMessage::Error(ProtoError::BadFormat(err)),
        }
    }

    pub fn new_sign_error(transaction: &ProtoTransaction) -> Self {
        ProtoTransaction {
            data: ProtoMessage::Error(ProtoError::Signature),
            ..transaction.clone()
        }
    }
}