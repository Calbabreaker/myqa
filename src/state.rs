use crate::{error::AppResult, store::Store};
use axum::{http::header, response::IntoResponse};

#[derive(Clone)]
pub struct AppState {
    pub jinja: minijinja::Environment<'static>,
    pub store: Store,
}

impl AppState {
    pub fn new(store: Store) -> Self {
        let mut jinja = minijinja::Environment::new();
        jinja.set_loader(|name| {
            Templates::get(name)
                .map(|f| String::from_utf8(f.data.into_owned()))
                .transpose()
                .map_err(|_| minijinja::ErrorKind::SyntaxError.into())
        });

        Self { jinja, store }
    }

    pub fn render_jinja(&self, name: &str, partial: bool) -> AppResult<axum::response::Response> {
        let headers = [get_cache_header()];
        let (status_code, template) = match self.jinja.get_template(name) {
            Ok(t) => (axum::http::StatusCode::OK, t),
            Err(_) => (
                axum::http::StatusCode::NOT_FOUND,
                self.jinja.get_template("404.html")?,
            ),
        };

        let text = template.render(minijinja::context! {partial})?;
        let html = axum::response::Html(text);
        Ok((status_code, headers, html).into_response())
    }
}

#[derive(rust_embed::Embed)]
#[folder = "templates"]
struct Templates;

#[derive(rust_embed::Embed)]
#[folder = "static"]
pub struct Assets;

impl Assets {
    pub fn get_response(path: &str) -> Option<axum::response::Response> {
        let file = Assets::get(path)?;
        let mime = mime_guess::from_path(path).first_or_text_plain();
        let headers = [(header::CONTENT_TYPE, mime.to_string()), get_cache_header()];
        Some((headers, file.data).into_response())
    }
}

fn get_cache_header() -> (axum::http::HeaderName, String) {
    if cfg!(debug_assertions) {
        (header::CACHE_CONTROL, "no-cache".to_string())
    } else {
        (header::CACHE_CONTROL, "max-age=604800".to_string())
    }
}
