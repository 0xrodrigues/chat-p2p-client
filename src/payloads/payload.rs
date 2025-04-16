use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatPayload {
    pub msg: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeRequestPayload {
    pub nonce: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeResponsePayload {
    pub nonce: String,
    pub signature: String,
}
