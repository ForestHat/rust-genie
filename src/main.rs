use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::{html, Markup};
use serde::{Deserialize, Serialize};
use std::io;

#[derive(Serialize, Deserialize)]
struct News {
    #[serde(rename = "Id")]
    id: u16,
    #[serde(rename = "Title")]
    title: String,
    #[serde(rename = "Url")]
    url: String,
    #[serde(rename = "Theme")]
    theme: String
}

#[get("/")]
async fn index() -> AwResult<Markup> {
    let news_feed: Vec<News> = get_news().await;
    
    Ok(
        html! {
            html {
                body {
                    div style="text-align: center;" {
                        header {
                            h1 {
                                "News site!"
                            }
                            }
                            h2 {
                                "Personolized news"
                            }
                            div {
                                @for i in 0..news_feed.len() {
                                    a href={(news_feed.get(i).unwrap().url)} {
                                        (news_feed.get(i).unwrap().title.to_owned() + &String::from(" THEME: ") + &news_feed.get(i).unwrap().theme)
                                        br;
                                    }
                                }
                            }
                        footer {
                            p {
                                "ForestHat"
                            }
                        }
                    }
                }
            }
        }
    )
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    println!("Started!");
    HttpServer::new(|| {
        App::new().service(index)
    }).bind(("127.0.0.1", 8080))?.run().await
}

async fn get_news() -> Vec<News> {
    let client = reqwest::Client::new();
    let request = client.get("http://localhost:4040");
    let resp = request.send().await.expect("Error while parsing!");
    let json = resp.text().await.expect("Error get the json!");

    serde_json::from_str::<Vec<News>>(&json).expect("Error while processing the news data")
}
