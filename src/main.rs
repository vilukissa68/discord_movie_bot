pub mod movie;
pub mod http;
pub mod db;
pub mod utils;

use crate::movie::*;

use dotenv::dotenv;
use serenity::utils::MessageBuilder;
use sqlx::mysql::MySqlPool;
use regex::Regex;
use std::time::Duration;

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
#[commands(ping, create_list, show_list, add_movie, search, watch, unwatch, remove, help)]
struct General;

#[group("collector")]
#[commands(delete_list)]
struct Collector;

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
#[aliases("list", "ls")]
async fn show_list(ctx: &Context, msg: &Message) -> CommandResult {
    let split = utils::split_string(msg.content.clone());
    match &split[..] {
        [_, table] => {
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            if !db::table_exists(&pool, &table_name).await? {
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
#[aliases("add")]
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
            match db::table_exists(&pool, &table_name).await? {
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
            match db::table_exists(&pool, &table_name).await? {
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
#[aliases("movie")]
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
#[aliases("w")]
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
#[aliases("uw")]
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

#[command]
#[aliases("rm")]
async fn remove(ctx: &Context, msg: &Message) -> CommandResult {
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
            let result =  db::delete_movie(&pool, &table_name, &matched_title).await?;
            if result {
                msg.reply(ctx, format!("Removed movie {} from list {}", matched_title, table.to_string())).await?;
            } else {
                msg.reply(ctx, format!("Movie {} not found in list {}", matched_title, table.to_string())).await?;
            }
        }
        _ => {msg.reply(ctx, "Invalid arguments").await?;}
    }
    Ok(())
}

#[command]
async fn delete_list(ctx: &Context, msg: &Message) -> CommandResult {
    println!("delete_list");
    let split = utils::split_string(msg.content.clone());
    match &split[..] {
        [_, table] => {
            // Check if addressing movie with name of id
            let pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;
            let table_name = format!("{}_{}", msg.guild_id.unwrap().0, table);
            if !db::table_exists(&pool, &table_name).await? {
                msg.reply(ctx, format!("List {} doesn't exist to begin with", table.to_string())).await?;
                return Ok(());
            }
            let _ = msg.reply(ctx, format!("Are you absolutely sure you want to delete list {}? (yes/no)", table.to_string())).await?;
            if let Some(answer) = &msg.author.await_reply(ctx).timeout(Duration::from_secs(20)).await {
                if answer.content.to_lowercase() != "yes" {
                    msg.reply(ctx, "Aborting").await?;
                    return Ok(());
                }
            }
            db::delete_list(&pool, &table_name).await?;

            if !db::table_exists(&pool, &table_name).await? {
                msg.reply(ctx, format!("Deleted list {}", table.to_string())).await?;
            } else {
                msg.reply(ctx, format!("List {} not found", table.to_string())).await?;
            }
        }
        _ => {msg.reply(ctx, "Invalid arguments").await?;}
    }

    Ok(())
}

#[command]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    let card = MessageBuilder::new()
        .push_line("Get started by creating a list with `!create_list {list_name}` and adding movies with `!add_movie {list_name} {movie_name}`. After watching a movie mark it watched rather then removing from list with `!watch {list_name} {movie_name}`. Remember to use quotes when typing titles with multiple words.")
        .push_line("")
        .push_bold_line("Commands:")
        .push_line("`!search {movie_name}` - Search for movie")
        .push_line("`!search {movie_name} {year}` - Search for movie with year")
        .push_line("`!create_list {list_name}` - Create a list")
        .push_line("`!add_movie {list_name} {movie_name}` - Add a movie to a list")
        .push_line("`!add_movie {list_name} {movie_name} {year}` - Add a movie to a list with year")
        .push_line("`!remove_movie {list_name} {movie_name}` - Remove a movie from a list")
        .push_line("`!watch {list_name} {movie_name}` - Mark a movie as watched")
        .push_line("`!unwatch {list_name} {movie_name}` - Mark a movie as unwatched")
        .push_line("`!list_movies {list_name}` - List all movies in a list")
        .push_line("`!delete_list {list_name}` - Delete a list")
        .push_line("`!help` - Show this message")
        .build();

    msg.channel_id.say(&ctx.http, &card).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let _pool = MySqlPool::connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL not set")).await?;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("!"))
        .group(&GENERAL_GROUP)
        .group(&COLLECTOR_GROUP);

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
