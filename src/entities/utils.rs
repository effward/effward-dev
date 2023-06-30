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

    Ok(escaped.to_lowercase())
}
