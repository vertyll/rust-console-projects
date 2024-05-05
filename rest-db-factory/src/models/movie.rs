use actix_web::web;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Movie {
    pub id: i32,
    pub movie_year: i32,
    pub movie_name: String,
    pub rating: f64
}
impl From<web::Json<Movie>> for Movie {
    fn from(tweet: web::Json<Movie>) -> Self {
        Movie {
            id: tweet.id,
            movie_year: tweet.movie_year,
            movie_name: tweet.movie_name.clone(),
            rating: tweet.rating,
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewMovie {
    pub movie_year: i32,
    pub movie_name: String,
    pub rating: f64,
}
impl From<web::Json<NewMovie>> for NewMovie {
    fn from(tweet: web::Json<NewMovie>) -> Self {
        NewMovie {
            movie_year: tweet.movie_year,
            movie_name: tweet.movie_name.clone(),
            rating: tweet.rating,
        }
    }
}