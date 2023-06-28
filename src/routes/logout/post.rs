use actix_web::{http::header::LOCATION, HttpResponse, Responder};
use actix_web_flash_messages::FlashMessage;

use crate::routes::session_state::TypedSession;

pub async fn process_logout(session: TypedSession) -> impl Responder {
    match session.get_user_id() {
        Ok(_) => {
            session.log_out();
            FlashMessage::success("successfully logged out").send();
            HttpResponse::SeeOther()
                .insert_header((LOCATION, "/"))
                .finish()
        }
        Err(_) => HttpResponse::SeeOther()
            .insert_header((LOCATION, "/"))
            .finish(),
    }
}
