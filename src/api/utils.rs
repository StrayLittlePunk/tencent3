use chrono::TimeZone;
use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

const HMAC_ALGORITHM: &str = "TC3-HMAC-SHA256";

pub struct SignatureV3Arg<'a> {
    pub content_type: &'a str,
    pub host: &'a str,
    pub request_payload: &'a str,
    pub service: &'a str,
    pub secret_key: &'a str,
    pub secret_id: &'a str,
    pub timestamp: u64,
}

// 生成v3签名
pub fn signature_v3_with_post(arg: SignatureV3Arg) -> String {
    use chrono::Utc;
    // build canonical request string
    let hashed_payload = sha256_hex(arg.request_payload);
    let signed_header = "content-type;host";
    let canonical_request = format!(
        "POST\n{}\n{}\ncontent-type:{}\nhost:{}\n\n{}\n{}",
        "/", "", arg.content_type, arg.host, signed_header, hashed_payload
    );

    // build sign string
    let datetime = if arg.timestamp == 0 {
        Utc::now()
    } else {
        Utc.timestamp_opt(arg.timestamp as i64, 0).unwrap()
    };
    let date = datetime.format("%F").to_string();
    let canonical_scope = format!("{}/{}/tc3_request", date, arg.service);
    let hashed_canonical_request = sha256_hex(canonical_request);
    let sign_string = format!(
        "{}\n{}\n{}\n{}",
        HMAC_ALGORITHM,
        datetime.timestamp(),
        canonical_scope,
        hashed_canonical_request
    );

    // sign string
    let secret_date = hmac_sha256(&date, format!("TC3{}", arg.secret_key));
    let secret_service = hmac_sha256(arg.service, secret_date);
    let secret_key = hmac_sha256("tc3_request", secret_service);
    let signature = to_hex_string(hmac_sha256(sign_string, secret_key).as_slice());

    format!(
        "{HMAC_ALGORITHM} Credential={}/{}, SignedHeaders={}, Signature={signature}",
        arg.secret_id, canonical_scope, signed_header
    )
}

fn hmac_sha256<S, K>(payload: S, key: K) -> Vec<u8>
where
    S: AsRef<[u8]>,
    K: AsRef<[u8]>,
{
    let payload = payload.as_ref();
    let key = key.as_ref();
    let mut hmac: Hmac<Sha256> = Hmac::new_from_slice(key).expect("invalid key");
    hmac.update(payload);
    hmac.finalize().into_bytes().as_slice().to_vec()
}

fn sha256_hex(payload: impl AsRef<[u8]>) -> String {
    let payload = payload.as_ref();
    let mut hasher = Sha256::new();
    hasher.update(payload);
    to_hex_string(hasher.finalize().as_slice())
}

fn to_hex_string(bytes: &[u8]) -> String {
    use std::fmt::Write;
    let mut hex_string = String::new();
    for &byte in bytes {
        write!(hex_string, "{:02x}", byte).unwrap();
    }
    hex_string
}

pub fn to_base64<S: AsRef<[u8]>>(bytes: S) -> String {
    use base64::{engine::general_purpose, Engine as _};
    general_purpose::STANDARD_NO_PAD.encode(bytes.as_ref())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sha256_hex_should_work() {
        let payload = r#"{"Limit": 1, "Filters": [{"Values": ["\u672a\u547d\u540d"], "Name": "instance-name"}]}"#;
        assert_eq!(
            sha256_hex(payload),
            "35e9c5b0e3ae67532d3c9f17ead6c90222632e5b1ff7f6e89887f1398934f064"
        );
    }
}
