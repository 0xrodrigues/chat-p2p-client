use serde::{Serialize, Deserialize};
use crate::payloads::payload::{ChatPayload, ChallengeRequestPayload, ChallengeResponsePayload};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
pub enum OutgoingMessage {
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
