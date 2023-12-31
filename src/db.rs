use sqlx::mysql::MySqlPool;
use sqlx::Row;
use crate::movie::{Movie, MovieShort};
use anyhow::Result;

pub async fn create_database(pool: &MySqlPool, database: &String) -> Result<(), sqlx::Error> {
    let query = format!("CREATE DATABASE IF NOT EXISTS {}", database);
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn create_list(pool: &MySqlPool, table: &String) -> Result<(), sqlx::Error> {
    println!("Creating table {}", table);
    let query = format!("
CREATE TABLE IF NOT EXISTS {} (id INT NOT NULL AUTO_INCREMENT, title VARCHAR(255) NOT NULL,
adder VARCHAR(255) NOT NULL, director VARCHAR(255) NOT NULL, actors VARCHAR(511) NOT NULL, language VARCHAR(255) NOT NULL,
country VARCHAR(255) NOT NULL, metascore VARCHAR(255) NOT NULL, imdbrating VARCHAR(255) NOT NULL,
imdbid VARCHAR(255) NOT NULL, year INT UNSIGNED NOT NULL, runtime INT UNSIGNED NOT NULL,
genre VARCHAR(255) NOT NULL, watched BOOLEAN NOT NULL, PRIMARY KEY (id))",
                        table);
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn delete_list(pool: &MySqlPool, table: &String) -> Result<(), sqlx::Error> {
    let query = format!("DROP TABLE {}", table);
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn add_movie(pool: &MySqlPool, table: &String, movie: &Movie) -> Result<(), sqlx::Error> {
    println!("Adding movie: {:?}", movie);
    let query = format!("INSERT INTO {} (title, adder, director, actors, language, country, metascore, imdbrating, imdbid, year, runtime, genre, watched)
VALUES (\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\", {}, {}, \"{}\", {})", table,
                        movie.title.clone(), movie.adder.clone(), movie.director.clone(),movie.actors.clone(), movie.language.clone(),
                        movie.country.clone(), movie.metascore.clone(), movie.imdbrating.clone(), movie.imdbid.clone(),
                        movie.year.clone(), movie.runtime.clone(), movie.genre.clone(), movie.watched.clone());
    let result = sqlx::query(query.as_str())
        .execute(pool).await?;
    println!("Result: {:?}", result);
    if result.rows_affected() == 0 {
        println!("No rows affected");
    } else {
        println!("Rows affected: {}", result.rows_affected());
    }
    Ok(())
}

pub async fn watch_movie(pool: &MySqlPool, table: &String, movie: &Movie) -> Result<bool> {
    let query = format!("UPDATE {} SET watched = \"{}\" WHERE title = \"{}\"", table, movie.watched.clone() as i32, movie.title.clone());
    println!("Query: {}", query);
    let rows_affected = sqlx::query(query.as_str())
        .execute(pool).await?
        .rows_affected();
    Ok(rows_affected > 0)
}

pub async fn delete_movie(pool: &MySqlPool, table: &String, title: &String) -> Result<bool> {
    let query = format!("DELETE FROM {} WHERE title = \"{}\"", table, title);
    let result = sqlx::query(query.as_str())
        .execute(pool).await?
        .rows_affected();
    Ok(result > 0)
}

pub async fn get_movie_by_name(pool: &MySqlPool, table: &String, title: &String) -> Result<Movie> {
    let query = format!("SELECT * FROM {} WHERE title = \"{}\"", table, title);
    let result = sqlx::query_as::<_, Movie>(query.as_str())
        .fetch_one(pool)
         .await;
    match result {
        Ok(m) => return Ok(m),
        Err(_) => return Err(anyhow::anyhow!("Movie not found"))
    }
}

pub async fn get_movies(pool: &MySqlPool, table: &String) -> Result<Vec<Movie>> {
    let query = format!("SELECT * FROM {}", table);
    let result = sqlx::query_as::<_, Movie>(query.as_str())
        .fetch_all(pool)
        .await;
    let mut movies: Vec<Movie> = Vec::new();
    match result {
        Ok(m) => {
            for movie in m {
                println!("Movie: {:?}", movie);
                movies.push(movie);
            }
        },
        Err(_) => return Err(anyhow::anyhow!("No movies found"))
    }
    println!("Movies: {:?}", movies);
    return Ok(movies);
}

// Used only for updating database
pub async fn get_movies_short(pool: &MySqlPool, table: &String) -> Result<Vec<MovieShort>> {
    println!("Getting movies short");
    let query = format!("SELECT title, adder, year, imdbid, watched FROM {}", table);
    let result = sqlx::query_as::<_, MovieShort>(query.as_str())
        .fetch_all(pool)
        .await;
    println!("Result: {:?}", result);
    let mut movies: Vec<MovieShort> = Vec::new();
    match result {
        Ok(m) => {
            for movie in m {
                println!("Movie: {:?}", movie);
                movies.push(movie);
            }
        },
        Err(_) => return Err(anyhow::anyhow!("No movies found"))
    }
    println!("Movies: {:?}", movies);
    return Ok(movies);
}

pub async fn table_exists(pool: &MySqlPool, table: &String) -> anyhow::Result<bool> {
    println!("Does table {} exist?", table);
    let tables = sqlx::query("SHOW TABLES")
        .fetch_all(pool).await?;


    for tab in tables {
        println!("Tab: {:?}", tab);
        let tab: String = tab.get(0);
        if tab == table.clone() {
            println!("Table {} exists", table);
            return Ok(true);
        }
    }
    return Ok(false);
}
