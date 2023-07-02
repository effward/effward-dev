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

pub fn redirect_entity_error(error: EntityError, entity_type: &str) -> HttpResponse {
    match error {
        EntityError::NotFound => {
            FlashMessage::debug(entity_type).send();
            FlashMessage::error(format!("{} not found in database", entity_type)).send();
            redirect("/error/404")
        }
        _ => {
            error!("Entity Error: {:?}", error);
            redirect("/error/500")
        }
    }
}
