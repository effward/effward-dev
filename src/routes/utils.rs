use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;
use actix_web_flash_messages::FlashMessage;
use log::error;

use crate::entities::EntityError;

pub fn redirect(location: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, location))
        .finish()
}

pub fn success_redirect(location: &str, success_message: &str) -> HttpResponse {
    FlashMessage::success(success_message).send();
    redirect(location)
}

pub fn info_redirect(location: &str, info_message: &str) -> HttpResponse {
    FlashMessage::info(info_message).send();
    redirect(location)
}

pub fn warning_redirect(location: &str, warning_message: &str) -> HttpResponse {
    FlashMessage::warning(warning_message).send();
    redirect(location)
}

pub fn error_redirect(location: &str, error_message: &str) -> HttpResponse {
    FlashMessage::error(error_message).send();
    redirect(location)
}

pub fn redirect_entity_error(error: EntityError, entity_type: &str) -> HttpResponse {
    match error {
        EntityError::NotFound => {
            FlashMessage::debug(entity_type).send();
            error_redirect(
                "/error/404",
                &format!("{} not found in database", entity_type),
            )
        },
        EntityError::InvalidInput("public_id", _) => {
            FlashMessage::debug(entity_type).send();
            error_redirect("/error/404", "invalid uuid")
        },
        _ => {
            error!("🔥 Entity Error: {:?}", error);
            redirect("/error/generic")
        }
    }
}
