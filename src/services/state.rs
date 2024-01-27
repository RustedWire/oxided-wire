pub enum OperatorState {
    Idle,
    KeyExchange(KeyExchangeStep),
    MessageForwarding,
}

pub enum KeyExchangeStep {
    InitShare,
    AckShare,
    ReqVerify,
    AckVerify,
}