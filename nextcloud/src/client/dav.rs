pub fn mkcol_method() -> reqwest::Method {
    reqwest::Method::from_bytes(b"MKCOL").unwrap()
}
pub fn propfind_method() -> reqwest::Method {
    reqwest::Method::from_bytes(b"PROPFIND").unwrap()
}
pub fn proppatch_method() -> reqwest::Method {
    reqwest::Method::from_bytes(b"PROPPATCH").unwrap()
}
pub fn copy_method() -> reqwest::Method {
    reqwest::Method::from_bytes(b"COPY").unwrap()
}
pub fn move_method() -> reqwest::Method {
    reqwest::Method::from_bytes(b"MOVE").unwrap()
}

pub enum DavError {
    BadUrl(url::ParseError),
    Network(reqwest::Error),
    XmlParse(quick_xml::Error),
}

pub trait DavProvider {
    fn files_url_string(&self) -> String;
    fn add_auth_header(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder;
}
