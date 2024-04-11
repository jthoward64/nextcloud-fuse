use super::dav::DavProvider;

#[derive(Debug, Clone)]
pub struct Nextcloud {
    // URL of the Nextcloud server
    origin: String,
    // Path to the WebDAV API (should be something like remote.php/dav)
    dav_path: String,
    username: String,
    password: String,
}

impl Nextcloud {
    pub fn new(origin: String, dav_path: String, username: String, password: String) -> Self {
        Self {
            origin,
            dav_path,
            username,
            password,
        }
    }
}

impl DavProvider for Nextcloud {
    fn files_url_string(&self) -> String {
        format!("{}/{}/files/{}/", self.origin, self.dav_path, self.username)
    }

    fn add_auth_header(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.basic_auth(&self.username, Some(&self.password))
    }
}
