use actix_web::Responder;
use actix_web_flash_messages::FlashMessage;

use crate::routes::{user_context::session_state::TypedSession, utils};

pub async fn process_logout(session: TypedSession) -> impl Responder {
    match session.get_user_id() {
        Ok(_) => {
            session.log_out();
            FlashMessage::success("successfully logged out").send();
            utils::redirect("/")
        }
        Err(_) => utils::redirect("/"),
    }
}
