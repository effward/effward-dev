use actix_web::cookie::Key;
use actix_web_flash_messages::{storage::CookieMessageStore, FlashMessagesFramework, Level};
use log::warn;

pub fn init_flash_messages(secret_key: Key) -> FlashMessagesFramework {
    warn!("💡 Initializing Flash Message Framework...");

    let message_store = CookieMessageStore::builder(secret_key.clone()).build();
    FlashMessagesFramework::builder(message_store)
        .minimum_level(Level::Debug)
        .build()
}
