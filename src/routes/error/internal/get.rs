use actix_web::{
    dev::ServiceResponse, middleware::ErrorHandlerResponse, web, HttpResponse, Responder, Result,
};
use actix_web_flash_messages::IncomingFlashMessages;
use sqlx::MySqlPool;
use tera::{Context, Tera};

use crate::routes::user_context::{session_state::TypedSession, user_context};

const PAGE_NAME: &str = "error - internal";
const HERO_BG_CLASS: &str = "hero-bg-500";

pub async fn internal(
    session: TypedSession,
    flash_messages: IncomingFlashMessages,
    pool: web::Data<MySqlPool>,
    tera: web::Data<Tera>,
) -> impl Responder {
    let user_context = user_context::build(
        session,
        flash_messages,
        &pool,
        PAGE_NAME,
        Some(HERO_BG_CLASS),
    )
    .await;

    // TODO: handle errors
    build_response(&tera, user_context.context)
}

pub fn render_internal<B>(res: ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    let request = res.into_parts().0;
    let tera = request.app_data::<web::Data<Tera>>().unwrap();

    // Don't try to get authenticated user or other session state
    let context = user_context::get_empty(PAGE_NAME, Some(HERO_BG_CLASS)).context;

    let response = build_response(&tera, context);
    Ok(ErrorHandlerResponse::Response(
        ServiceResponse::new(request, response).map_into_right_body(),
    ))
}

fn build_response(tera: &Tera, context: Context) -> HttpResponse {
    // TODO: handle error
    let rendered = tera.render("500.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}
