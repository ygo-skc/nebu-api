use serde::Serialize;

#[derive(Serialize)]
pub struct APIStatus {
    pub version: String,
}
