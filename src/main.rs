use actix_web::{get, App, HttpServer, Result as AwResult};
use maud::{html, Markup};
use std::io;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};
use std::process::Command;
use std::thread;

static mut GL_NEWS_VEC: Vec<String> = Vec::new();
static mut GL_THEME_VEC: Vec<String> = Vec::new();
static mut GL_URL_VEC: Vec<String> = Vec::new();

#[get("/")]
async fn index() -> AwResult<Markup> {
    unsafe {
        if GL_NEWS_VEC.len() != 0 {
            GL_NEWS_VEC.clear();
            GL_THEME_VEC.clear();
            GL_URL_VEC.clear();
        }
    }
    test().await;
    
    Ok(
        unsafe {
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
                                @for i in 0..GL_NEWS_VEC.len() {
                                    a href={(GL_URL_VEC.get(i).unwrap())} {
                                        (GL_NEWS_VEC.get(i).unwrap().to_owned() + " THEME: " + GL_THEME_VEC.get(i).unwrap())
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
        }
    )
}

#[actix_web::main]
async fn main() -> io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    }).bind(("127.0.0.1", 8080))?.run().await
}


async fn test() {
    let resp = reqwest::get("https://habr.com/ru/news/").await.unwrap();
    let doc = Document::from(resp.text().await.unwrap().as_str());

    let mut news_vector: Vec<String> = Vec::new();
    let mut url_vector: Vec<String> = Vec::new();
    
    let mut size: usize = 0;

    for node in doc.find(Class("tm-articles-list__item")) {
        let news: String = node.find(Class("tm-title__link").descendant(Name("span"))).next().unwrap().text();
        news_vector.push(news);

        let url: String = node.find(Class("tm-title__link")).next().unwrap().attr("href").unwrap().to_string();
        url_vector.push(String::from("https://habr.com") + &url);

        size += 1;
    }

    let mut handles = Vec::new();

    for i in 0..size {
        let news: String = news_vector.get(i).unwrap().to_string();
        let url: String = url_vector.get(i).unwrap().to_string();

        let handle = thread::spawn(move || {
            run_cmd(news, url);
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn run_cmd(news: String, url: String) {
    let exec: String = String::from("python3");
    let output = Command::new(exec).args(&["main.py", &news]).output().unwrap();
    let out = String::from_utf8_lossy(&output.stdout).replace("\n", "");

    unsafe {
        GL_NEWS_VEC.push(news);
        GL_THEME_VEC.push(out);
        GL_URL_VEC.push(url);
    }
}