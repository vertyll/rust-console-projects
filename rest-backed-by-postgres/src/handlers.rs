use super::db_access::*;
use super::models::{NewMovie};
use super::state::AppState;

use actix_web::{web, HttpResponse};


pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let mut visit_count = app_state.visit_count.lock().unwrap();
    *visit_count += 1;

    match ping_db(&app_state.db).await {
        Ok(_) => {
            let response = format!("{} {} times. Database connection is okay.",
                                   app_state.health_check_response, visit_count);
            HttpResponse::Ok().json(&response)
        },
        Err(_) => {
            HttpResponse::ServiceUnavailable().json("Database connection error")
        }
    }
}

pub async fn get_movies_for_year(app_state: web::Data<AppState>, params: web::Path<(i32,)>,
) -> HttpResponse {
    let tuple = params.0;
    let movie_year: i32 = tuple;
    let movies = get_movies_for_year_db(&app_state.db, movie_year).await;
    HttpResponse::Ok().json(movies)
}

pub async fn get_movie_details(app_state: web::Data<AppState>, params: web::Path<(i32, i32)>,
) -> HttpResponse {
    let (movie_year, movie_id) = (params.0, params.1);
    let movie = get_movie_details_db(&app_state.db, movie_year, movie_id).await;
    HttpResponse::Ok().json(movie)
}

/* curl -X POST localhost:3000/movies/ \
-H "Content-Type: application/json" \
 -d '{"movie_name":"Test Movie", "movie_year": 2020, "rating": 4.5}'
*/
pub async fn post_new_movie(new_movie: web::Json<NewMovie>, app_state: web::Data<AppState>,
) -> HttpResponse {
    let movie = post_new_movie_db(&app_state.db, new_movie.into()).await;

    HttpResponse::Ok().json(movie)
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPool;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_courses_success() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let tutor_id: web::Path<(i32,)> = web::Path::from((1,));
        let resp = get_movies_for_year(app_state, tutor_id).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_course_detail_test() {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
        let pool: PgPool = PgPool::connect(&database_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1994, 10));
        let resp = get_movie_details(app_state, params).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }
}