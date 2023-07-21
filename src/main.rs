pub mod movie;
pub mod http;
pub mod db;

//use crate::movie::*;
use crate::http::*;
use crate::db::*;

use dotenv::dotenv;
use sqlx::mysql::MySqlPool;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;

    // sqlx::query("CREATE TABLE IF NOT EXISTS movies (name INT NOT NULL AUTO_INCREMENT, name VARCHAR(255) NOT NULL, PRIMARY KEY (id))")
    //     .execute(&pool).await?;

    // sqlx::query("INSERT INTO movies (name) VALUES (?)")
    //     .bind("The Godfather")
    //     .execute(&pool).await?;

    // let rows = sqlx::query!("SELECT * FROM movies")
    //     .fetch_all(&pool).await?;

    // for row in rows {
    //     println!("Movie: {}", row.name);
    // }

    // let mut list = MovieList::new(String::from("My List"), String::from("Me"));
    // let movie = Movie::new(String::from("The Godfather"), String::from("Me"));
    // let movie2 = Movie::new(String::from("The Godfather 2"), String::from("Me"));
    // add_movie(&pool, String::from("movies"), &movie2).await?;
    // add_movie(&pool, String::from("movies"), &movie).await?;
    // list.add_movie(movie);
    // list.add_movie(movie2);
    http_get_movie_object(String::from("The Godfather part III"), "CLI".to_string()).await;

    Ok(())
}
