use actix_web::{
    dev::ServiceResponse, middleware::ErrorHandlerResponse, web, HttpResponse, Responder, Result,
};
use actix_web_flash_messages::IncomingFlashMessages;

use tera::Tera;

use crate::{
    entities::EntityStores,
    routes::user_context::{
        session_state::TypedSession,
        user_context::{self, UserContext},
    },
};

const PAGE_NAME: &str = "error - not found";
const HERO_BG_CLASS: &str = "hero-bg-404";
const DEFAULT_MISSING_TYPE: &str = "item";

pub async fn not_found(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    stores: web::Data<EntityStores>,
    tera: web::Data<Tera>,
) -> impl Responder {
    // TODO: handle errors
    let mut user_context = user_context::build(
        session,
        flash_messages,
        &stores,
        PAGE_NAME,
        Some(HERO_BG_CLASS),
    )
    .await;

    build_response(&tera, &mut user_context)
}

pub fn render_not_found<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let request = res.into_parts().0;
    let tera = request.app_data::<web::Data<Tera>>().unwrap();

    // Don't try to get authenticated user or other session state
    let mut user_context = user_context::get_empty(PAGE_NAME, Some(HERO_BG_CLASS));

    let response = build_response(tera, &mut user_context);
    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, response).map_into_right_body(),
    ))
}

fn build_response(tera: &Tera, user_context: &mut UserContext) -> HttpResponse {
    let mut missing_type = DEFAULT_MISSING_TYPE;
    if !user_context.flash_messages.is_empty() {
        missing_type = &user_context.flash_messages[0];
    }

    user_context.context.insert("missing_type", missing_type);

    // TODO: handle error
    let rendered = tera.render("404.html", &user_context.context).unwrap();

    HttpResponse::Ok().body(rendered)
}
