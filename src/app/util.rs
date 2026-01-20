use base64::{Engine, prelude::BASE64_URL_SAFE_NO_PAD};
use jsonwebtoken::{self, EncodingKey, Header, decode_header};
use serde_json::{Value, json};

/// encode (鍵なし / alg: none)
pub fn encode<T: AsRef<str>>(s: T) -> Result<String, String> {
    let my_payload = json!(s.as_ref());

    let header = Header::default();

    let token = jsonwebtoken::encode(
        &header,
        &my_payload,
        &EncodingKey::from_secret(&[]), // ダミーのキー
    )
    .expect("failed to encode");

    Ok(token)
}

/// decode (署名検証なし)
pub fn decode<T: AsRef<str>>(s: T) -> Result<Value, String> {
    // --- 1. ヘッダーのデコード (jsonwebtoken の機能を使用) ---
    // let header = decode_header(s.as_ref()).expect("failed to decode");
    // println!("--- Header (v9 decode_header) ---");
    // println!("{:#?}\n", header);

    // --- 2. ペイロードのデコード (手動 Base64 デコード) ---
    // JWTは "Header.Payload.Signature" の形式なので、2番目の要素を取得する
    let parts: Vec<&str> = s.as_ref().split('.').collect();
    if parts.len() < 2 {
        return Err("Invalid token format".to_owned());
    }

    let payload_b64 = parts[1];
    let payload_bytes = BASE64_URL_SAFE_NO_PAD
        .decode(payload_b64)
        .expect("failed to decode base64 payload");

    let payload: Value =
        serde_json::from_slice(&payload_bytes).expect("failed to decode serialization");

    Ok(payload)
}
