use sqlx::mysql::MySqlPool;
use crate::movie::*;

pub async fn create_list(pool: &MySqlPool, table: String) -> Result<(), sqlx::Error> {
    let query = format!("CREATE TABLE IF NOT EXISTS {} (name INT NOT NULL AUTO_INCREMENT, name VARCHAR(255) NOT NULL, PRIMARY KEY (id))", table);
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
    let query = format!("INSERT INTO {} (name) VALUES (\"{}\")", table, movie.name.clone());
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn update_movie(pool: &MySqlPool, table: String, movie: &Movie) -> Result<(), sqlx::Error> {
    let query = format!("UPDATE {} SET name = \"{}\" WHERE id = \"{}\"", table, movie.name.clone(), movie.name.clone());
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn delete_movie(pool: &MySqlPool, table: String, movie: &Movie) -> Result<(), sqlx::Error> {
    let query = format!("DELETE FROM {} WHERE id = \"{}\"", table, movie.name.clone());
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn get_movie(pool: &MySqlPool, table: String, movie: &Movie) -> Result<(), sqlx::Error> {
    let query = format!("SELECT * FROM {} WHERE id = \"{}\"", table, movie.name.clone());
    sqlx::query(query.as_str())
        .execute(pool).await?;
    Ok(())
}

pub async fn get_movies(pool: &MySqlPool, table: String) -> Result<(), sqlx::Error> {
    let query = format!("SELECT * FROM {}", table);
    sqlx::query(query.as_str())
        .fetch_all(pool).await?;
    Ok(())
}
