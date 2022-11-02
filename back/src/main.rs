use salvo::prelude::*;
use salvo::serve_static::StaticDir;

#[handler]
async fn home() -> &'static str {
    "this is the home page"
}
#[handler]
async fn books() -> &'static str {
    "this is the book page"
}
#[handler]
async fn articles() -> &'static str {
    "those are my articles from backend"
}

#[tokio::main]
async fn main() {
    let router = Router::new()
        .push(
            Router::with_path("api")
                .push(Router::with_path("home").get(home))
                .push(Router::with_path("articles").get(articles))
                .push(Router::with_path("books").get(books)),
        )
        .push(
            Router::with_path("<**>").get(
                StaticDir::new(["dist"])
                    .with_defaults("./index.html")
                    .with_fallback("./index.html"),
            ),
        );

    Server::new(TcpListener::bind("127.0.0.1:7878"))
        .serve(router)
        .await;
}