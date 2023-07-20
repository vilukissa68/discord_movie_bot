use reqwest::Client;

#[tokio::main]
pub async fn http_get_movie_data(name: String) -> String {
    let response = reqwest::get("http://www.omdbapi.com/?t={}&apikey=5e5a0dbb")
        .await
        .unwrap()
        .text()
        .await;
    println!("{:?}", response);
    return String::from("Hello");
}
