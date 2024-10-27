use axum::{
    body::Body,
    extract::State,
    http::{Request, Uri},
    response::IntoResponse,
    routing::get,
};

use crate::{
    error::AppResult,
    state::{AppState, Assets},
};

pub fn create_views_router() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(file_route))
        .route("/*path", get(file_route))
}

async fn file_route(
    State(state): State<AppState>,
    uri: Uri,
    request: Request<Body>,
) -> AppResult<impl IntoResponse> {
    log::trace!("Accessed {}", uri.path());
    let path = &uri.path()[1..];

    if let Some(response) = Assets::get_response(path) {
        Ok(response)
    } else {
        let mut path = path.to_string();
        if uri.path().ends_with("/") {
            path.push_str("index.html");
        } else {
            path.push_str(".html");
        }

        // This also render 404.html if can't be found
        let partial = request
            .headers()
            .get("hx-request")
            .is_some_and(|h| h == "true");
        state.render_jinja(&path, partial)
    }
}
