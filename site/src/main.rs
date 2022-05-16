#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod types;

use std::net::SocketAddr;

use axum::{
    response::{Html, Redirect},
    routing::get,
};

use types::{Css, JavaScript, Png};

const BOT_INVITE: &str = "https://discord.com/api/oauth2/authorize?client_id=975460814007963738&permissions=0&scope=applications.commands%20bot";

struct Files {
    paste: String,
    index: String,
    highlight_js: String,
    highlight_css: String,
    logo: Vec<u8>,
}

#[tokio::main]
async fn main() {
    let files = Files {
        paste: std::fs::read_to_string("resources/paste.html").unwrap(),
        index: std::fs::read_to_string("resources/index.html").unwrap(),
        highlight_js: std::fs::read_to_string("resources/highlight.js").unwrap(),
        highlight_css: std::fs::read_to_string("resources/highlight.css").unwrap(),
        logo: std::fs::read("resources/logo.png").unwrap(),
    };

    let app = axum::Router::new()
        .route("/", get(move || async { Html(files.index) }))
        .route(
            "/invite",
            get(move || async { Redirect::permanent(BOT_INVITE) }),
        )
        .route(
            "/:channelid/:messageid/:filename",
            get(move || async { Html(files.paste) }),
        )
        .route(
            "/js/highlight.js",
            get(move || async { JavaScript(files.highlight_js) }),
        )
        .route(
            "/css/highlight.css",
            get(move || async { Css(files.highlight_css) }),
        )
        .route(
            "/logo.png", 
            get(move || async { Png(files.logo) })
            );

    let listen = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("[INFO] Listening on http://{}", &listen);
    axum::Server::bind(&listen)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start the server");
}
