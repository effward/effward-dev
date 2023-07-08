use log::warn;
use tera::Tera;

use super::ServerError;

pub fn init_tera() -> Result<Tera, ServerError> {
    warn!("🌎 Initializing Tera static templates...");
    Ok(Tera::new("templates/**/*")?)
}
