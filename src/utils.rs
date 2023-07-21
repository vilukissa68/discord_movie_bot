use regex::Regex;
use crate::movie::Movie;
use serenity::utils::MessageBuilder;

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
        .push_line(&format!("Metascore: {}", movie.metascore))
        .push_line(&format!("IMDB Rating: {}", movie.imdbrating))
        .push_line(&format!("{}", imdburl(&movie.imdbid)))
        .build();
    card
}
