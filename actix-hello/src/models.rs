use serde::{Serialize, Deserialize};

#[derive(Serialize)]
pub struct EchoResponse {
    pub uppercase: String,
    pub lowercase: String,
    pub echoed: String,
}

#[derive(Deserialize)]
pub struct EchoRequest {
    pub message: String,
}