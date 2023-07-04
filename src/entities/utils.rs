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
