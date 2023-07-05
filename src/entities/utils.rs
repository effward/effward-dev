use base64::{engine::general_purpose, Engine};
use sha2::{Digest, Sha256};
use shortguid::ShortGuid;
use uuid::Uuid;

use super::EntityError;

pub fn sanitize_text(
    text: &str,
    min: usize,
    max: usize,
    param_name: &'static str,
) -> Result<String, EntityError> {
    if text.len() < min {
        return Err(EntityError::InvalidInput(param_name, "too short"));
    }

    let escaped = html_escape::encode_text(text);
    if escaped.len() > max {
        return Err(EntityError::InvalidInput(param_name, "too long"));
    }

    Ok(escaped.to_string())
}

pub fn get_readable_public_id(uuid_vec: Vec<u8>) -> String {
    let mut bytes: [u8; 16] = [0; 16];
    let mut i = 0;
    for byte in uuid_vec {
        bytes[i] = byte;
        i += 1;

        if i >= 16 {
            break;
        }
    }

    let public_uuid = Uuid::from_bytes(bytes);
    ShortGuid::from(public_uuid).to_string()
}

pub fn parse_public_id(public_id: &str) -> Result<Uuid, EntityError> {
    match ShortGuid::try_parse(public_id) {
        Ok(id) => Ok(*id.as_uuid()),
        Err(_) => Err(EntityError::InvalidInput("public_id", "invalid uuid")),
    }
}

// This must not be changed without special considerations.
// It is used to encode content stored in cache keys, and changing it could result in different cache results
pub fn vec_to_base64_string(vec: Vec<u8>) -> String {
    general_purpose::URL_SAFE_NO_PAD.encode(vec)
}

// This must not be changed without special considerations.
// It is used to hash content stored in the DB, and changing it could result in different hashes
pub fn hash_content(content: &str, min: usize, max: usize) -> Result<Vec<u8>, EntityError> {
    if content.len() < min {
        return Err(EntityError::InvalidInput("content", "content is too short"));
    }
    if content.len() > max {
        return Err(EntityError::InvalidInput("content", "content is too long"));
    }

    let mut hasher = Sha256::new();
    hasher.update(content);

    let hash = hasher.finalize()[..].to_vec();
    Ok(hash)
}
