use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use jsonwebtoken::{self, EncodingKey, Header, decode_header};
use serde_json::Value;

/// encode (鍵なし / alg: none)
pub fn encode(header: Option<&Header>, payload: Option<&Value>) -> Result<String, String> {
    // todo: header
    let header = if let Some(header) = header {
        header
    } else {
        &Header::default()
    };

    let payload = if let Some(payload) = payload {
        payload
    } else {
        &Value::Null
    };

    let token = jsonwebtoken::encode(
        header,
        payload,
        &EncodingKey::from_secret(&[]), // ダミーのキー
    )
    .expect("failed to encode");

    Ok(token)
}

/// decode (署名検証なし)
pub fn decode(s: &str) -> Result<(Option<Header>, Option<Value>), String> {
    let header = match decode_header(s) {
        Ok(x) => Some(x),
        Err(err) => return Err(format!("failed to decode: {}", err.to_string())),
    };

    // JWT は "Header.Payload.Signature" の形式。2 番目の要素を取得
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() < 2 {
        return Err("failed to decode payload: invalid token format".to_owned());
    }
    let payload_b64 = parts[1];
    let payload_bytes = match BASE64_URL_SAFE_NO_PAD.decode(payload_b64) {
        Ok(x) => x,
        Err(err) => {
            return Err(format!(
                "failed to decode base64 payload: {}",
                err.to_string()
            ));
        }
    };
    let payload = match serde_json::from_slice(&payload_bytes) {
        Ok(x) => Some(x),
        Err(err) => {
            return Err(format!(
                "failed to decode serialization: {}",
                err.to_string()
            ));
        }
    };

    Ok((header, payload))
}
