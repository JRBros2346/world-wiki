use axum::{extract::Path, response::Html};

pub async fn home() -> Html<&'static str> {
    Html("<h1>Home</h1>")
}
pub async fn page(Path(name): Path<String>) -> Html<String> {
    Html(format!("<h1>{name}</h1>"))
}
pub async fn edit(Path(name): Path<String>) -> Html<String> {
    todo!()
}
