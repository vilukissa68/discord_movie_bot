use reqwest;
use serde_json;
use crate::movie::*;

pub async fn http_get_movie_data_name(name: String) -> String {
    println!("Getting data for {}", name);
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


pub async fn http_get_movie_data_imdb(id: String) -> String {
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

pub async fn http_get_movie_object(name: String, adder: String) -> Movie {
    let response = http_get_movie_data_name(name).await;
    let json = serde_json::from_str::<serde_json::Value>(&response).unwrap();
    println!("{:?}", serde_json::to_string_pretty(&json).unwrap());

    // Year needs some extra handling before it can be parsed to u32
    let year = json["Year"].to_string().replace("\"", "").parse::<u32>().unwrap();
    let movie = Movie::new(
        json["Title"].to_string(),
        adder,
        Some(json["Director"].to_string()),
        Some(json["Language"].to_string()),
        Some(json["Country"].to_string()),
        Some(json["Metascore"].to_string()),
        Some(json["imdbRating"].to_string()),
        Some(json["imdbID"].to_string()),
        Some(year),
    );
    return movie;
}
