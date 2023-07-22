use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, Tabled)]
pub struct Movie {
    pub title: String,
    pub adder: String,
    pub director: String,
    pub actors: String,
    pub language: String,
    pub country: String,
    pub metascore: String,
    pub imdbrating: String,
    pub imdbid: String,
    pub year: u32,
    pub watched: bool,
}

impl Movie {
    pub fn new(title: String, adder: String, director: Option<String>, actors: Option<String>, language: Option<String>,
    country: Option<String>, metascore: Option<String>, imdbrating: Option<String>, imdbid: Option<String>,
    year: Option<u32>) -> Movie {
        Movie {
            title: title,
            adder: adder,
            director: director.unwrap_or("NULL".to_string()),
            actors: actors.unwrap_or("NULL".to_string()),
            language: language.unwrap_or("NULL".to_string()),
            country: country.unwrap_or("NULL".to_string()),
            metascore: metascore.unwrap_or("NULL".to_string()),
            imdbrating: imdbrating.unwrap_or("NULL".to_string()),
            imdbid: imdbid.unwrap_or("NULL".to_string()),
            year: year.unwrap_or(0),
            watched: false,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MovieList {
    title: String,
    creator: String,
    movies: Vec<Movie>,
}

impl PartialEq for Movie {
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title
    }
}

impl MovieList {
    pub fn new(title: String, creator: String) -> MovieList {
        MovieList {
            title,
            creator,
            movies: Vec::new(),
        }
    }

    pub fn add_movie(&mut self, movie: Movie) {
        self.movies.push(movie);
    }

    pub fn remove_movie(&mut self, movie: Movie) {
        let index = self.movies.iter().position(|x| *x == movie).unwrap();
        self.movies.remove(index);
    }

    pub fn get_movie(&self, title: String) -> Option<&Movie> {
        self.movies.iter().find(|x| x.title == title)
    }
}

pub fn serialize_movie(movie: &Movie) -> String {
    serde_json::to_string(movie).unwrap()
}

pub fn deserialize_movie(json: String) -> Movie {
    let movie: Movie = serde_json::from_str(&json).unwrap();
    movie
}

pub fn serialize_movielist(list: &MovieList) -> String {
    serde_json::to_string(list).unwrap()
}

pub fn deserialize_movielist(json: String) -> MovieList {
    let list: MovieList = serde_json::from_str(&json).unwrap();
    list
}
