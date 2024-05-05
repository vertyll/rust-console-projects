use super::handlers::*;
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub fn movie_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/movies")
            .route("/", web::post().to(post_new_movie))
            .route("/{movie_year}", web::get().to(get_movies_for_year))
            .route("/{movie_year}/{movie_id}", web::get().to(get_movie_details)),
    );
}