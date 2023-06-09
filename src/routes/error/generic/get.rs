use actix_web::{
    dev::ServiceResponse, middleware::ErrorHandlerResponse, web, HttpResponse, Responder, Result,
};
use actix_web_flash_messages::IncomingFlashMessages;
use tera::{Context, Tera};

use crate::{
    entities::EntityStores,
    routes::{
        user_context::{session_state::TypedSession, user_context},
        utils,
    },
};

const PAGE_NAME: &str = "error - internal";
const HERO_BG_CLASS: &str = "hero-bg-500";

pub async fn generic(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    stores: web::Data<EntityStores>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context = user_context::build(
        session,
        flash_messages,
        &stores,
        PAGE_NAME,
        Some(HERO_BG_CLASS),
    )
    .await;

    // TODO: handle errors
    build_response(&tera, user_context.context)
}

pub fn render_generic<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let (request, response) = res.into_parts();

    // TODO: use session to get current user for navbar

    let response = match response.error() {
        Some(error) => utils::error_redirect(
            "/error/generic",
            &format!("{}: {}", response.status(), error),
        ),
        None => utils::error_redirect("/error/generic", &format!("{}", response.status())),
    };

    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, response).map_into_right_body(),
    ))
}

fn build_response(tera: &Tera, context: Context) -> HttpResponse {
    // TODO: handle error
    let rendered = tera.render("500.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
