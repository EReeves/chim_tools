#![feature(io_error_more)]
use axum::{
    routing::{get}, Router, response::Html,
};
use render::{html, rsx};
use std::net::SocketAddr;
use render;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> Html<String> {


    let world = "World!";
    Html::from(html! {
        <html>
          <head><title>{"chim_tool"}</title></head>
          <body>
            {format!("Hello {}", world)}
          </body>
        </html>
    })

}