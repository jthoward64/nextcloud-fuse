use super::dav::{
    copy_method, mkcol_method, move_method, propfind_method, proppatch_method, DavError,
    DavProvider,
};

fn start_request(
    provider: &dyn DavProvider,
    method: reqwest::Method,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    let url_string = provider.files_url_string() + path;
    let url = url::Url::parse(&url_string).map_err(DavError::BadUrl)?;
    let client = reqwest::Client::new();
    let request = client
        .request(method, url)
        .header("User-Agent", "provider-fuse");

    Ok(provider.add_auth_header(request))
}

pub fn start_propfind(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, propfind_method(), path)
}

pub fn start_proppatch(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, proppatch_method(), path)
}

pub fn start_mkcol(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, mkcol_method(), path)
}

pub fn start_get(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, reqwest::Method::GET, path)
}

pub fn start_put(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, reqwest::Method::PUT, path)
}

pub fn start_delete(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, reqwest::Method::DELETE, path)
}

pub fn start_copy(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, copy_method(), path)
}

pub fn start_move(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, move_method(), path)
}

pub fn start_post(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, reqwest::Method::POST, path)
}

pub fn start_head(
    provider: &dyn DavProvider,
    path: &str,
) -> Result<reqwest::RequestBuilder, DavError> {
    start_request(provider, reqwest::Method::HEAD, path)
}
