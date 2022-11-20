use rust_embed::RustEmbed;
use salvo::prelude::*;
use salvo::serve_static::{static_embed, StaticDir};

#[derive(RustEmbed)]
#[folder = "./images"]
struct Assets;

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(
            Router::with_path("api")
                .push(Router::with_path("home").get(home))
                .push(Router::with_path("articles").get(articles))
                .push(Router::with_path("books").get(books))
                .push(Router::with_path("images/<**path>").get(static_embed::<Assets>())),
        )
        .push(
            Router::with_path("<**>").get(
                StaticDir::new(["../front/dist"])
                    .with_defaults(["./index.html"])
                    .with_fallback("./index.html"),
            ),
        );

    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}

#[handler]
async fn home() -> &'static str {
    "# this is the home page"
}

#[handler]
async fn books() -> &'static str {
    "## this is the h2 book page"
}

#[handler]
async fn articles() -> &'static str {
    r#"
        # then we have a h1 style \endline
        ## then we have 2 h2 styles \endline
        ## then we have 2 h2 styles \endline
        Those are my articles from backend those are my articles from backend \endline
        those are my articles from backend
        those are my articles from backend
        those are my articles from backend. \endline
        \image [image title] api/images/screenshot.png
        those are my articles from backend,
        those are my articles from backend,
        those are my articles from backend.
    "#
}

struct Publication {
    content: String,
}
