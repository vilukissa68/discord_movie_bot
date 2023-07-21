pub mod movie;
pub mod http;
pub mod db;
pub mod utils;

use crate::db::*;
use crate::movie::*;

use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use regex::Regex;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    macros::{
        command,
        group
    }
};


#[group]
#[commands(ping, create_list, show_list, add_movie, search, watch, unwatch)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler{}


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong Pong!").await?;
    Ok(())
}

#[command]
async fn create_list(ctx: &Context, msg: &Message) -> CommandResult {
    let split = utils::split_string(msg.content.clone());
    match &split[..] {
        [_, table] => {
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            db::create_list(&pool, table_name).await?;
            msg.reply(ctx, format!("Created list {}", table.to_string())).await?;
        }
        _ => {
            msg.reply(ctx, "Invalid arguments").await?;
        }
    }
    Ok(())
}

#[command]
async fn show_list(ctx: &Context, msg: &Message) -> CommandResult {
    let split = utils::split_string(msg.content.clone());
    match &split[..] {
        [_, table] => {
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            if !db::table_exists(&pool, table_name.to_string()).await? {
                msg.reply(ctx, format!("List {} does not exist", table)).await?;
                return Ok(());
            }
            let movies: Option<Vec<Movie>> = db::get_movies(&pool, table_name).await;
            if movies.is_none() {
                msg.reply(ctx, "No movies in list").await?;
                return Ok(());
            }
            let card = utils::create_movie_list_card(&movies.unwrap(), &table.to_string());
            msg.channel_id.say(&ctx.http, card).await?;
        }
        _ => {msg.reply(ctx, "Invalid arguments").await?;}
    }
        Ok(())
}

#[command]
async fn add_movie(ctx: &Context, msg: &Message) -> CommandResult {
    let split = utils::split_string(msg.content.clone());
    let adder = msg.author.name.clone();
    match &split[..] {
        [_, table, title, year] => {
            let year = year.parse::<u32>();
            if year.is_err() {
                msg.reply(ctx, "Invalid year").await?;
                return Ok(());
            }
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            match db::table_exists(&pool, table_name.to_string()).await? {
                true => {
                    let movie = http::http_get_movie(&title, &adder, Some(year.unwrap())).await;
                    if movie.is_err() {
                        msg.reply(ctx, format!("Movie {} not found", title)).await?;
                        return Ok(());
                    }
                    let movie = movie.unwrap();
                    db::add_movie(&pool, table_name, &movie).await?;
                    msg.reply(ctx, format!("Added movie {} to list {}", movie.title, table.to_string())).await?;
                }
                false => {msg.reply(ctx, format!("List {} does not exist", table.to_string())).await?;}
            }
        }
        [_, table, title] => {
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            match db::table_exists(&pool, table_name.to_string()).await? {
                true => {
                    let movie = http::http_get_movie(&title, &adder, None).await;
                    if movie.is_err() {
                        msg.reply(ctx, format!("Movie {} not found", title)).await?;
                        return Ok(());
                    }
                    let movie = movie.unwrap();
                    db::add_movie(&pool, table_name, &movie).await?;
                    msg.reply(ctx, format!("Added movie {} to list {}", movie.title, table.to_string())).await?;
                }
                false => {msg.reply(ctx, format!("List {} does not exist", table.to_string())).await?;}
            }
        }
        _ => {msg.reply(ctx, "Invalid arguments").await?;}
    }
    Ok(())
}

#[command]
async fn search(ctx: &Context, msg: &Message) -> CommandResult {
    let split = utils::split_string(msg.content.clone());
    match &split[..] {
        [_, title, year] => {
            let year = year.parse::<u32>();
            if year.is_err() {
                msg.reply(ctx, "Invalid year").await?;
                return Ok(());
            }
            let movie = http::http_get_movie(&title, &"None".to_string(), Some(year.unwrap())).await;
            if movie.is_err() {
                msg.reply(ctx, format!("Movie {} not found", title)).await?;
                return Ok(());
            }
            let movie = movie.unwrap();
            let card = utils::create_movie_card(&movie);
            msg.channel_id.say(&ctx.http, card).await?;
        }
        [_, title] => {
            let movie = http::http_get_movie(&title, &"None".to_string(), None).await;
            if movie.is_err() {
                msg.reply(ctx, format!("Movie {} not found", title)).await?;
                return Ok(());
            }
            let movie = movie.unwrap();
            let card = utils::create_movie_card(&movie);
            msg.channel_id.say(&ctx.http, card).await?;
        }
            _ => {msg.reply(ctx, "Invalid arguments").await?;}

    }
    Ok(())
}

#[command]
async fn watch(ctx: &Context, msg: &Message) -> CommandResult {
    let split = utils::split_string(msg.content.clone());
    match &split[..] {
        [_, table, title] => {
            // Check if addressing movie with name of id
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            let mut matched_title = title.to_string();
            let re = Regex::new("^[0-9]+$").unwrap();
            if re.is_match(title) {
                let idx: u32 = title.parse::<u32>().unwrap();
                let t = utils::match_idx_to_name(&pool, idx, &table_name).await;

                if t.is_none() {
                    msg.reply(ctx, "Invalid movie id").await?;
                    return Ok(());
                }
                matched_title = t.unwrap();
            }
            let mut movie = db::get_movie_by_name(&pool, &table_name, &matched_title).await?;
            movie.watched = true;
            let result =  db::watch_movie(&pool, &table_name, &movie).await?;
            if result {
                msg.reply(ctx, format!("Watched movie {} from list {}", movie.title, table.to_string())).await?;
            } else {
                msg.reply(ctx, format!("Movie {} not found in list {}", movie.title, table.to_string())).await?;
            }
        }
        _ => {msg.reply(ctx, "Invalid arguments").await?;}
    }
    Ok(())
}

#[command]
async fn unwatch(ctx: &Context, msg: &Message) -> CommandResult {
    let split = utils::split_string(msg.content.clone());
    match &split[..] {
        [_, table, title] => {
            // Check if addressing movie with name of id
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            let mut matched_title = title.to_string();
            let re = Regex::new("^[0-9]+$").unwrap();
            if re.is_match(title) {
                let idx: u32 = title.parse::<u32>().unwrap();
                let t = utils::match_idx_to_name(&pool, idx, &table_name).await;

                if t.is_none() {
                    msg.reply(ctx, "Invalid movie id").await?;
                    return Ok(());
                }
                matched_title = t.unwrap();
            }
            let mut movie = db::get_movie_by_name(&pool, &table_name, &matched_title).await?;
            movie.watched = false;
            let result =  db::watch_movie(&pool, &table_name, &movie).await?;
            if result {
                msg.reply(ctx, format!("Watched movie {} from list {}", movie.title, table.to_string())).await?;
            } else {
                msg.reply(ctx, format!("Movie {} not found in list {}", movie.title, table.to_string())).await?;
            }
        }
        _ => {msg.reply(ctx, "Invalid arguments").await?;}
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let _pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP);

    let discord_token = std::env::var("DISCORD_API_TOKEN").expect("DISCORD_API_TOKEN not set");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(discord_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    //create_list(&pool, String::from("movies")).await?;
    //let movie = http_get_movie_object(String::from("The Godfather part III"), "CLI".to_string()).await;
    //add_movie(&pool, String::from("movies"), &movie).await?;
    //let movie = get_movie_by_name(&pool, String::from("movies"), String::from("The Godfather part III")).await;
    //println!("{:?}", movie.unwrap());
    Ok(())
}
