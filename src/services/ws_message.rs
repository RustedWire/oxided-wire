use uuid::Uuid;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub enum ProtoMessage {
    KeyExchange(ProtoKeys),
    Message(Vec<u8>), 
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
    pub c: [u8; 32],
    pub z: [u8; 32],
}