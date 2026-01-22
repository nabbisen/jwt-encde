use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

/// encode with dummy signature
pub fn encode(header: Option<&Value>, payload: Option<&Value>) -> Result<String, String> {
    // header
    let header = if let Some(header) = header {
        header
    } else {
        &Value::Null
    };

    let header_json = serde_json::to_string(header).expect("Failed to get header hmac");
    let header_b64 = BASE64_URL_SAFE_NO_PAD.encode(header_json.as_bytes());

    // payload
    let payload = if let Some(payload) = payload {
        payload
    } else {
        &Value::Null
    };

    let claims_json = serde_json::to_string(payload).expect("Failed to get payload json str");
    let claims_b64 = BASE64_URL_SAFE_NO_PAD.encode(claims_json.as_bytes());

    // signature source
    let signing_input = format!("{}.{}", header_b64, claims_b64);
    // signing key (dummy)
    let mut mac = HmacSha256::new_from_slice(b"").expect("Failed to generate dummy key");
    mac.update(signing_input.as_bytes());
    let result = mac.finalize();
    let signature_bytes = result.into_bytes();
    // signature
    let signature_b64 = BASE64_URL_SAFE_NO_PAD.encode(signature_bytes);

    // JWT
    let token = format!("{}.{}", signing_input, signature_b64);

    Ok(token)
}

/// decode without signature verified
pub fn decode(s: &str) -> Result<(Option<Value>, Option<Value>), String> {
    // JWT format = "Header.Payload.Signature"
    let parts: Vec<&str> = s.split('.').collect();

    // header
    if parts.len() == 0 {
        return Err("Failed to decode header: invalid token format".to_owned());
    }

    let header_b64 = parts[0];
    let header_bytes = match BASE64_URL_SAFE_NO_PAD.decode(header_b64) {
        Ok(x) => x,
        Err(err) => {
            return Err(format!(
                "Failed to decode base64 header: {}",
                err.to_string()
            ));
        }
    };
    let header = match serde_json::from_slice(&header_bytes) {
        Ok(x) => Some(x),
        Err(err) => {
            return Err(format!(
                "Failed to decode serialization: {}",
                err.to_string()
            ));
        }
    };

    // payload
    if parts.len() < 2 {
        return Err("Failed to decode payload: invalid token format".to_owned());
    }

    let payload_b64 = parts[1];
    let payload_bytes = match BASE64_URL_SAFE_NO_PAD.decode(payload_b64) {
        Ok(x) => x,
        Err(err) => {
            return Err(format!(
                "Failed to decode base64 payload: {}",
                err.to_string()
            ));
        }
    };
    let payload = match serde_json::from_slice(&payload_bytes) {
        Ok(x) => Some(x),
        Err(err) => {
            return Err(format!(
                "Failed to decode serialization: {}",
                err.to_string()
            ));
        }
    };

    Ok((header, payload))
}
