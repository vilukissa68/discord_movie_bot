pub mod movie;
pub mod http;

use crate::movie::*;
use crate::http::*;

use dotenv::dotenv;
use sqlx::mysql::MySqlPoolOptions;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error>{
    dotenv().ok();
    let api_token = std::env::var("API_TOKEN").expect("API_TOKEN not set");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;

    let row: (i64,) = sqlx::query_as("SELECT $1")
        .bind(150_i64)
        .fetch_one(&pool).await?;

    assert_eq!(row.0, 150);
    println!("{:?}", row.0);

    Ok(())


    /*let mut list = MoveList::new(String::from("My List"), String::from("Me"));
    let movie = Movie::new(String::from("The Godfather"), String::from("Me"));
    let movie2 = Movie::new(String::from("The Godfather 2"), String::from("Me"));
    list.add_movie(movie);
    list.add_movie(movie2);
    http_get_movie_data(String::from("The Godfather"));
    println!("{:?}", list);*/
}
