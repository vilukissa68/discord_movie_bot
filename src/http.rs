use reqwest;
use serde_json;
use crate::movie::*;
use crate::utils;

pub async fn http_get_movie_data_name(name: &String, year: Option<u32>) -> String {
    println!("Getting data for {}", name);
    if year.is_some() {
        let response = reqwest::get(format!("http://www.omdbapi.com/?t={name}&y={year}&apikey={key}",
                                            name=name,
                                            year=year.unwrap(),
                                            key=std::env::var("OMDB_API_TOKEN").expect("OBDB_API_TOKEN not set")))
            .await
            .unwrap()
            .text()
            .await;
        println!("{:?}", response);
        return response.unwrap();
    }
    let response = reqwest::get(format!("http://www.omdbapi.com/?t={name}&apikey={key}",
                                        name=name,
                                        key=std::env::var("OMDB_API_TOKEN").expect("OBDB_API_TOKEN not set")))
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
    return response.unwrap();
}


pub async fn http_get_movie_data_imdb(id: &String) -> String {
    let response = reqwest::get(format!("http://www.omdbapi.com/?i={id}&apikey={key}",
                                        id=id,
                                        key=std::env::var("OMDB_API_TOKEN").expect("OBDB_API_TOKEN not set")))
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
    return response.unwrap();
}

pub async fn http_get_dummy_data(_name: String) -> String {
    return "Hello".to_string();
}

pub async fn http_get_movie(name: &String, adder: &String, year: Option<u32>) -> anyhow::Result<Movie> {
    let response;
    if utils::is_imdbid(name) {
       response = http_get_movie_data_imdb(name).await;
    } else {
        response = http_get_movie_data_name(name, year).await;
    }
    let json = serde_json::from_str::<serde_json::Value>(&response).unwrap();
    if json["Response"] == "False" {
        return Err(anyhow::anyhow!("Movie not found"));
    }

    // Year needs some extra handling before it can be parsed to u32
    let year = json["Year"].to_string().replace("\"", "").parse::<u32>().unwrap();
    let movie = Movie::new(
        json["Title"].to_string().replace("\"", ""),
        adder.to_string(),
        Some(json["Director"].to_string().replace("\"", "")),
        Some(json["Actors"].to_string().replace("\"", "")),
        Some(json["Language"].to_string().replace("\"", "")),
        Some(json["Country"].to_string().replace("\"", "")),
        Some(json["Metascore"].to_string().replace("\"", "")),
        Some(json["imdbRating"].to_string().replace("\"", "")),
        Some(json["imdbID"].to_string().replace("\"", "")),
        Some(year),
    );
    return Ok(movie);
}
