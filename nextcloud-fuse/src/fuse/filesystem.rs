use std::io::Empty;

use fuse3::{
    path::{
        reply::{DirectoryEntry, ReplyDirectory, ReplyInit},
        PathFilesystem,
    },
    raw::Request,
    Errno,
};
use nextcloud::Nextcloud;

struct NextcloudFilesystem {
    client: Nextcloud,
}

impl NextcloudFilesystem {
    fn new(client: Nextcloud) -> Self {
        let mut config = include_str!("../../.config").split_whitespace();
        let username = config.next().unwrap();
        let password = config.next().unwrap();
        let base_url = config.next().unwrap();
        let dav_path = config.next().unwrap();

        let provider = client::Nextcloud::new(
            base_url.to_string(),
            dav_path.to_string(),
            username.to_string(),
            password.to_string(),
        );

        self { client: provider }
    }
}

impl PathFilesystem for NextcloudFilesystem {
    async fn init(&self, req: Request) -> Result<ReplyInit> {}

    fn destroy(&self, req: Request) {}

    type DirEntryStream<'a> = Empty<Result<DirectoryEntry, Errno>> where Self: 'a;
    type DirEntryPlusStream<'a> = Empty<Result<DirectoryEntry, Errno>> where Self:'a;

    async fn readdir(
        &'a self,
        req: Request,
        path: &'a OsStr,
        fh: u64,
        offset: i64,
    ) -> Result<ReplyDirectory<Self::DirEntryStream>, Errno> {
        match self.client.ls(path.to_str().unwrap()).await {
            Ok(items) => {
                let entries = items.into_iter().map(|item| {
                    let kind = match item.is_dir {
                        true => FileType::Directory,
                        false => FileType::RegularFile,
                    };

                    DirEntry {
                        kind,
                        name: item.name,
                    }
                });

                Ok(ReplyDirectory::new(entries))
            }
            Err(_) => Err(Errno::ENOENT),
        }
    }
}
