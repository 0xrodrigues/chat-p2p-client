use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum IncomingMessage {
    #[serde(rename = "message")]
    Chat {
        from: String,
        to: String,
        payload: ChatPayload,
    },
    #[serde(rename = "challenge-request")]
    ChallengeRequest {
        from: String,
        to: String,
        payload: ChallengeRequestPayload,
    },
    #[serde(rename = "challenge-response")]
    ChallengeResponse {
        from: String,
        to: String,
        payload: ChallengeResponsePayload,
    },
}

use super::payload::{ChatPayload, ChallengeRequestPayload, ChallengeResponsePayload};
