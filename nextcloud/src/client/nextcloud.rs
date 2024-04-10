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
    pub fn files_url(&self) -> String {
        format!("{}/{}/files/{}/", self.origin, self.dav_path, self.username)
    }

    pub fn new(origin: String, dav_path: String, username: String, password: String) -> Self {
        Self {
            origin,
            dav_path,
            username,
            password,
        }
    }

    async fn add_auth_header(&self, req: reqwest::RequestBuilder) -> reqwest::RequestBuilder {
        req.basic_auth(&self.username, Some(&self.password))
    }
}
