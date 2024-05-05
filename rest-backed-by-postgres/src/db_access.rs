use super::models::{Movie, NewMovie};
use sqlx::postgres::PgPool;

pub async fn ping_db(pool: &PgPool) -> Result<(), sqlx::Error> {
    // This is a simple query that doesn't depend on your table structure
    sqlx::query("SELECT 1")
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_movies_for_year_db(pool: &PgPool, movie_year: i32) -> Vec<Movie> {
    let movies = sqlx::query!(
        "SELECT id, movie_name, movie_year, rating FROM movies WHERE movie_year = $1",
        movie_year
    )
        .fetch_all(pool)
        .await
        .unwrap();

    movies
        .iter()
        .map(|movie_row| Movie {
            id: movie_row.id,
            movie_year: movie_row.movie_year.unwrap_or(0), // Provide a default value for movie_year
            movie_name: movie_row.movie_name.clone().unwrap_or_default(), // Default for String is an empty string
            rating: movie_row.rating.unwrap_or_default(), // Default for f64 could be 0.0 or another appropriate value
        })
        .collect()
}

pub async fn get_movie_details_db(pool: &PgPool, movie_year: i32, movie_id: i32) -> Movie {
    // Prepare SQL statement
    let movie_row = sqlx::query!(
        "SELECT id, movie_name, movie_year, rating from movies where id = $2 and movie_year = $1",
        movie_year, movie_id
    )
        .fetch_one(pool)
        .await
        .unwrap();
    // Execute query
    Movie {
        id: movie_row.id,
        movie_year: movie_row.movie_year.unwrap_or(0), // Provide a default value for movie_year
        movie_name: movie_row.movie_name.clone().unwrap_or_default(), // Default for String is an empty string
        rating: movie_row.rating.unwrap_or_default(), // Default for f64 could be 0.0 or another appropriate value
    }
}

pub async fn post_new_movie_db(pool: &PgPool, new_movie: NewMovie) -> Movie {
    let movie_row = sqlx::query!(
        "INSERT INTO movies (movie_name, movie_year, rating)
         VALUES ($1, $2, $3) RETURNING id, movie_name, movie_year, rating",
        new_movie.movie_name,
        new_movie.movie_year,
        new_movie.rating
    )
        .fetch_one(pool)
        .await
        .unwrap();
    // Retrieve result
    Movie {
        id: movie_row.id,
        movie_year: movie_row.movie_year.unwrap_or(0),
        movie_name: movie_row.movie_name.clone().unwrap_or_default(),
        rating: movie_row.rating.unwrap_or_default(),
    }
}
