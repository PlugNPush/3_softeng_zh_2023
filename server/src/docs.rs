use axum::{
    http::{header, HeaderMap, StatusCode, Uri},
    response::Redirect,
};

static FOUR_O_FOUR_HTML: &str = "404.html";

fn serve_file(path: &str) -> Result<(HeaderMap, Vec<u8>), StatusCode> {
    let file = docs::EmbeddedDocs::get(path).ok_or(StatusCode::NOT_FOUND)?;

    let mut headers = HeaderMap::new();
    let mime = mime_guess::from_path(path).first_or_octet_stream();
    headers.append(header::CONTENT_TYPE, mime.as_ref().parse().unwrap());

    Ok((headers, file.data.into()))
}

pub async fn docs_handler(uri: Uri) -> Result<(HeaderMap, Vec<u8>), Redirect> {
    let path = uri.path().trim_start_matches('/');
    if path.is_empty() {
        return Err(Redirect::to("docs/overview.html"));
    }
    Ok(serve_file(path)
        .or_else(|_| serve_file(FOUR_O_FOUR_HTML))
        .unwrap())
}
