use actix_web::http::header::LOCATION;
use actix_web::HttpResponse;

///
/// redirect to page
///
pub fn redirect_to_page(page: &str) -> HttpResponse {
    HttpResponse::SeeOther()
        .insert_header((LOCATION, page))
        .finish()
}