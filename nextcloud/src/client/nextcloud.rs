use super::{
    dav::{DavItem, DavProvider},
    pase_propfind::pase_propfind,
    start_dav::start_propfind,
};

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

    pub async fn ls(&self, path: &str) -> Result<Vec<super::dav::DavItem>, super::dav::DavError> {
        let request =
            start_propfind(self, path)?.body("<propfind xmlns=\"DAV:\"><allprop /></propfind>");
        let response = request
            .send()
            .await
            .map_err(super::dav::DavError::Network)?;
        let body = response
            .text()
            .await
            .map_err(super::dav::DavError::Network)?;
        let value = pase_propfind(body)?;

        let contents: Vec<DavItem> = vec![];

        for response in value.responses {
            println!("{:?}", response)
        }

        Ok(contents)
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
