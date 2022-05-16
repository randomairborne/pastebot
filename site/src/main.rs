#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod types;

use std::net::SocketAddr;

use axum::{
    response::{Html, Redirect},
    routing::get,
};

use types::{Css, JavaScript, Png};

const BOT_INVITE: &str = "https://discord.com/api/oauth2/authorize?client_id=975460814007963738&permissions=0&scope=applications.commands%20bot";

#[tokio::main]
async fn main() {
    let paste = std::fs::read("resources/paste.html").unwrap();
    let index = std::fs::read("resources/index.html").unwrap();
    let about = std::fs::read("resources/about.html").unwrap();
    let privacy = std::fs::read("resources/privacy.html").unwrap();
    let terms = std::fs::read("resources/terms.html").unwrap();
    let css = std::fs::read("resources/main.css").unwrap();
    let highlight_js = std::fs::read("resources/highlight.js").unwrap();
    let highlight_css = std::fs::read("resources/highlight.css").unwrap();
    let logo = std::fs::read("resources/logo.png").unwrap();
    let app = axum::Router::new()
        .route("/", get(move || async { Html(index) }))
        .route(
            "/invite",
            get(move || async { Redirect::permanent(BOT_INVITE) }),
        )
        .route("/about", get(move || async { Html(about) }))
        .route(
            "/:channelid/:messageid/:filename",
            get(move || async { Html(paste) }),
        )
        .route("/css/main.css", get(move || async { Css(css) }))
        .route(
            "/js/highlight.js",
            get(move || async { JavaScript(highlight_js) }),
        )
        .route(
            "/css/highlight.css",
            get(move || async { Css(highlight_css) }),
        )
        .route("/logo.png", get(move || async { Png(logo) }));

    let listen = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("[INFO] Listening on http://{}", &listen);
    axum::Server::bind(&listen)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start the server");
}
