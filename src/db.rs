use sqlx::mysql::MySqlPool;
use crate::movie::{Movie};
use anyhow::Result;

pub async fn create_list(pool: &MySqlPool, table: String) -> Result<(), sqlx::Error> {
    let query = format!("
CREATE TABLE IF NOT EXISTS {} (id INT NOT NULL AUTO_INCREMENT, title VARCHAR(255) NOT NULL,
adder VARCHAR(255) NOT NULL, director VARCHAR(255) NOT NULL, actors VARCHAR(511) NOT NULL, language VARCHAR(255) NOT NULL,
country VARCHAR(255) NOT NULL, metascore VARCHAR(255) NOT NULL, imdbrating VARCHAR(255) NOT NULL,
imdbid VARCHAR(255) NOT NULL, year INT UNSIGNED NOT NULL, watched BOOLEAN NOT NULL,
PRIMARY KEY (id))",
                        table);
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn remove_list(pool: &MySqlPool, table: String) -> Result<(), sqlx::Error> {
    let query = format!("DROP TABLE {}", table);
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn add_movie(pool: &MySqlPool, table: String, movie: &Movie) -> Result<(), sqlx::Error> {
    println!("Adding movie: {:?}", movie);
    let query = format!("INSERT INTO {} (title, adder, director, actors, language, country, metascore, imdbrating, imdbid, year, watched)
VALUES (\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\",\"{}\", {}, {})", table,
                        movie.title.clone(), movie.adder.clone(), movie.director.clone(),movie.actors.clone(), movie.language.clone(),
                        movie.country.clone(), movie.metascore.clone(), movie.imdbrating.clone(), movie.imdbid.clone(),
                        movie.year.clone(), movie.watched.clone());
    let result = sqlx::query(query.as_str())
        .execute(pool).await?;
    if result.rows_affected() == 0 {
        println!("No rows affected");
    } else {
        println!("Rows affected: {}", result.rows_affected());
    }
    Ok(())
}

pub async fn update_movie(pool: &MySqlPool, table: String, movie: &Movie) -> Result<(), sqlx::Error> {
    let query = format!("UPDATE {} SET title = \"{}\" WHERE id = \"{}\"", table, movie.title.clone(), movie.title.clone());
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn delete_movie(pool: &MySqlPool, table: String, movie: &Movie) -> Result<(), sqlx::Error> {
    let query = format!("DELETE FROM {} WHERE id = \"{}\"", table, movie.title.clone());
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn get_movie_by_name(pool: &MySqlPool, table: String, title: String) -> Option<Movie> {
    let query = format!("SELECT * FROM {} WHERE title = \"{}\"", table, title);
    let result = sqlx::query_as::<_, Movie>(query.as_str())
        .fetch_one(pool)
         .await;
    match result {
        Ok(movie) => Some(movie),
        Err(_) => None
    }
}

pub async fn get_movies(pool: &MySqlPool, table: String) -> Option<Vec<Movie>> {
    let query = format!("SELECT * FROM {}", table);
    let result = sqlx::query_as::<_, Movie>(query.as_str())
        .fetch_all(pool)
        .await;
    let mut movies: Vec<Movie> = Vec::new();
    match result {
        Ok(m) => {
            for movie in m {
                movies.push(movie);
            }
        },
        Err(_) => return None
    }
    return Some(movies);
}

pub async fn table_exists(pool: &MySqlPool, table: String) -> anyhow::Result<bool> {
    let tables = sqlx::query!("SHOW TABLES")
        .fetch_all(pool).await?;

    for tab in tables {
        if tab.Tables_in_discord == table {
            println!("Table {} exists", table);
            return Ok(true);
        }
    }
    return Ok(false);

}
