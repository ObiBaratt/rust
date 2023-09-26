use serde::Serialize;

#[derive(Serialize)]
pub struct EchoResponse {
    pub uppercase: String,
    pub lowercase: String,
    pub echoed: String,
}
