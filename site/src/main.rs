#![warn(clippy::all, clippy::pedantic, clippy::nursery)]

mod types;

use std::{fs::read, net::SocketAddr};

use axum::{
    response::{Html, Redirect},
    routing::get,
};

use types::{Css, Ico, Png, JS};

static BOT_INVITE: &str = "https://discord.com/api/oauth2/authorize?client_id=975460814007963738&permissions=0&scope=applications.commands%20bot";

#[tokio::main]
async fn main() {
    let paste = read("resources/html/paste.html").unwrap();
    let index = read("resources/html/index.html").unwrap();
    let about = read("resources/html/about.html").unwrap();
    let privacy = read("resources/html/privacy.html").unwrap();
    let terms = read("resources/html/terms.html").unwrap();
    let main_css = read("resources/css/main.css").unwrap();
    let paste_css = read("resources/css/paste.css").unwrap();
    let color_js = read("resources/js/color.js").unwrap();
    let highlight_js = read("resources/js/highlight.js").unwrap();
    let highlight_css = read("resources/css/highlight.css").unwrap();
    let logo = read("resources/img/logo.png").unwrap();
    let favicon = read("resources/img/favicon.ico").unwrap();
    let invite = Redirect::permanent(BOT_INVITE);
    let app = axum::Router::new()
        .route("/", get(move || async { Html(index) }))
        .route("/invite", get(move || async { invite }))
        .route("/about", get(move || async { Html(about) }))
        .route("/terms", get(move || async { Html(terms) }))
        .route("/privacy", get(move || async { Html(privacy) }))
        .route("/css/main.css", get(move || async { Css(main_css) }))
        .route("/css/paste.css", get(move || async { Css(paste_css) }))
        .route("/js/color.js", get(move || async { JS(color_js) }))
        .route("/img/logo.png", get(move || async { Png(logo) }))
        .route("/favicon.ico", get(move || async { Ico(favicon) }))
        .route(
            "/:channelid/:messageid/:filename",
            get(move || async { Html(paste) }),
        )
        .route("/js/highlight.js", get(move || async { JS(highlight_js) }))
        .route(
            "/css/highlight.css",
            get(move || async { Css(highlight_css) }),
        );

    let listen = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("[INFO] Listening on http://{}", &listen);
    axum::Server::bind(&listen)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start the server");
}
