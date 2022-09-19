use jsonwebtoken::{encode, Header, EncodingKey, errors::Error as JwtError};
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    account_id: String,
    generated: usize,
    #[serde(rename = "calderaGuid")]
    caldera_guid: Uuid,
    #[serde(rename = "acProvider")]
    ac_provider: String,
    notes: String,
    fallback: bool
}

impl Default for Claims {
    /// Fallback caldera as Default
    fn default() -> Self {
        Self::new(
            String::new(),
            String::from("EasyAntiCheat"),
            String::from("doRequest error: Post \"https://caldera-service-prod.ecosec.on.epicgames.com/caldera/api/v1/launcher/racp\": dial tcp: lookup caldera-service-prod.ecosec.on.epicgames.com: no such host"),
            true
        )
    }
}

impl Claims {
    pub fn new(account_id: String, ac_provider: String, notes: String, fallback: bool) -> Self {
        let generated = match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
            Ok(n) => n.as_secs(),
            Err(_) => 0
        } as usize;

        let caldera_guid = Uuid::new_v4();

        Self {
            account_id,
            generated,
            caldera_guid,
            ac_provider,
            notes,
            fallback
        }
    }

    pub fn encode_default(&self) -> Result<String, JwtError> {
        encode(
            &Header::default(),
            self,
            &EncodingKey::from_secret(Uuid::new_v4().as_bytes())
        )
    }
}