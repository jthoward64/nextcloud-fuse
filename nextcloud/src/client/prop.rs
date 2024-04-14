use super::xml::{ToXml, Xml, XmlTag};

#[derive(Debug, Clone)]
pub enum UnknownStatus {
    Unknown,
    UnknownSuccess,
    UnknownClientError,
    UnknownServerError,
}

#[derive(Debug, Clone)]
pub enum PropStatStatus {
    Unknown(UnknownStatus),
    Ok,
    Unauthorized,
    Forbidden,
    NotFound,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropFind {
    pub props: Vec<XmlTag>,
    pub depth: u8,
}

impl ToXml for PropFind {
    fn to_xml(&self) -> String {
        let props = self
            .props
            .iter()
            .map(|prop| format!("<{} />", prop.full_name()))
            .collect::<Vec<String>>()
            .join("");
        format!(
            r#"<d:propfind
              xmlns:d="DAV:"
              xmlns:oc="http://owncloud.org/ns"
              xmlns:nc="http://nextcloud.org/ns"
              xmlns:ocs="http://open-collaboration-services.org/ns">
              xmlns:ocm="http://open-cloud-mesh.org/ns">
                <d:prop>{}</d:prop>
            </d:propfind>"#,
            props
        )
    }
}

#[derive(Debug, Clone)]
pub struct PropStat {
    pub status: PropStatStatus,
    pub prop_list: Xml,
}

impl PropStat {
    pub fn new(status: PropStatStatus) -> Self {
        Self {
            status,
            prop_list: Xml::new({
                XmlTag {
                    namespace: "d".to_string(),
                    name: "prop".to_string(),
                }
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MultiStatusResponse {
    pub href: String,
    pub prop_stats: Vec<PropStat>,
    pub response_description: Option<String>,
}

#[derive(Debug, Clone)]
pub struct MultiStatus {
    pub responses: Vec<MultiStatusResponse>,
}

#[derive(Debug, Clone)]
pub enum PropPatchStatus {
    Unknown(UnknownStatus),
    Ok,
    Forbidden,
    // ForbiddenProtectedProperty,
    Conflict,
    FailedDependency,
    InsufficientStorage,
}

#[derive(Debug, Clone)]
pub struct PropPatch {
    pub set_props: Vec<Xml>,
    pub remove_props: Vec<XmlTag>,
}

#[derive(Debug, Clone)]
pub enum MkColStatus {
    Unknown(UnknownStatus),
    Created,
    Forbidden,
    MethodNotAllowed,
    Conflict,
    UnsupportedMediaType,
    InsufficientStorage,
}
