use axum::{routing, Router};
use tokio::process::{Child, Command};
use tower_http::trace::TraceLayer;

mod handlers;

struct Server(Child);
impl Drop for Server {
    fn drop(&mut self) {
        self.0.start_kill().expect("Nah, I'd Win!");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _server = Server(
        Command::new("surreal start")
            .args(["-u", "root"])
            .args(["-p", "root"])
            .arg(".")
            .spawn()?,
    );
    let app = Router::new()
        .layer(TraceLayer::new_for_http())
        .route("/", routing::get(handlers::home))
        .route("/page/:name", routing::get(handlers::page))
        .route("/edit/:name", routing::get(handlers::edit));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
