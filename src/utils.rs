use regex::Regex;
use sqlx::mysql::MySqlPool;
use serenity::utils::MessageBuilder;
use tabled::{
    settings::{Modify, Alignment, object::Segment},
    builder::Builder, settings::Style};

use crate::movie::Movie;
use crate::db::{get_movies};
use anyhow::Result;

// Split string is quotes aware
pub fn split_string(s: String) -> Vec<String>{
    let re = Regex::new(r#""([^"]+)"|(\S+)"#).unwrap();
    let split = re.captures_iter(&s)
        .map(|cap| {
            if cap.get(1).is_some() {
                cap.get(1).unwrap().as_str().to_string()
            } else {
                cap.get(2).unwrap().as_str().to_string()
            }
        })
        .collect::<Vec<String>>();
    split
}

pub fn is_imdbid(s: &String) -> bool {
    let re = Regex::new(r"tt\d{7}").unwrap();
    re.is_match(s)
}

pub fn imdburl(id: &String) -> String {
    format!("https://www.imdb.com/title/{}", id)
}

pub fn create_movie_card(movie: &Movie) -> String {
    let card = MessageBuilder::new()
        .push_bold_line(&movie.title)
        .push_line(&format!("Year: {}", movie.year))
        .push_line(&format!("Director: {}", movie.director))
        .push_line(&format!("Actors: {}", movie.actors))
        .push_line(&format!("Language: {}", movie.language))
        .push_line(&format!("Country: {}", movie.country))
        .push_line(&format!("Genre: {}", movie.genre))
        .push_line(&format!("Metascore: {}", movie.metascore))
        .push_line(&format!("Runtime: {} min", movie.runtime))
        .push_line(&format!("IMDB Rating: {}", movie.imdbrating))
        .push_line(&format!("{}", imdburl(&movie.imdbid)))
        .build();
    card
}

pub fn create_movie_list_card(movies: &Vec<Movie>, table: &String) -> String {
    // Generate list of movies as discord markdown table
    let mut card = MessageBuilder::new();
    card.push_bold_line(&format!("Movies in list {}:", table));
    card.push_mono_line("Title | Year | Director | Imdb Score");
    let mut i = 1;
    for movie in movies {
        match movie.watched {
            true => {
                card.push_strike_line(&format!("`{} | {}  |  {}  |  {}  |  {}`", i, movie.title, movie.year, movie.director, movie.imdbrating));
            },
            false => {
                card.push_mono_line(&format!("{} | {}  |  {}  |  {}  |  {}", i, movie.title, movie.year, movie.director, movie.imdbrating));
            },
        }
        i = i + 1;
    }
    card.build()
}

pub fn create_movies_list_table(movies: &Vec<Movie>, table: &String) -> String {
    let greeting: String = format!("Movies in list {}:\n", table);

    // Show Title, year, director and imdb score
    // Watch status is shown by strikethrough
    let mut builder = Builder::default();
    builder.set_header(vec!["Title", "Year", "Director", "Runtime", "Imdb Score", "W"]);

    for movie in movies {
        if movie.watched {
            builder.push_record(vec![format!("---{}---", movie.title),
                                     format!("---{}---", movie.year),
                                     format!("---{}---", movie.director),
                                     format!("---{}---", movie.runtime),
                                     format!("---{}---", movie.imdbrating),
                                     format!("X")]);
        } else {
            builder.push_record(vec![movie.title.to_string(),
                                     movie.year.to_string(),
                                     movie.director.to_string(),
                                     format!("{} min", movie.runtime),
                                     movie.imdbrating.to_string(),
                                     format!(" ")]);
        }
    }
    let mut table = builder.build();
    table.with(Modify::new(Segment::all()).with(Alignment::center()));

    return format!("{}\n`{}`", greeting, table.with(Style::sharp()));
}

pub async fn match_idx_to_name(pool: &MySqlPool, idx: u32, table: &String) -> Result<String> {
    format!("{}-{}", table, idx);
    let movies = get_movies(pool, &table.to_string()).await?;
    let movie = &movies[idx as usize - 1];
    if movie.title.is_empty() {
        return Err(anyhow::anyhow!("Invalid id {}", idx));
    }
    Ok(movie.title.clone())
}
