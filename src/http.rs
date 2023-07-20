use reqwest::Client;

#[tokio::main]
pub async fn http_get_movie_data(name: String) -> String {
    let response = reqwest::get(format!("http://www.omdbapi.com/?t={name}&apikey={key}",
                                        name=name,
                                        key=std::env::var("OMDB_API_TOKEN").expect("OBDB_API_TOKEN not set")))
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
    return String::from("Hello");
}
