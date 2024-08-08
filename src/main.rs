use axum::{
    extract::Path, response::Html, routing, Router
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new()
        .route("/", routing::get(home))
        .route("/page/:name", routing::get(page))
        .route("/edit/:name", routing::get(edit));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}

async fn home() -> Html<&'static str> {
    Html("<h1>Home</h1>")
}
async fn page(Path(name): Path<String>) -> Html<String> {
    Html(format!("<h1>{name}</h1>"))
}
async fn edit(Path(name): Path<String>) -> Html<String> {
    todo!()
}