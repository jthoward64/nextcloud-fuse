use quick_xml::name::QName;

pub trait ToXml {
    fn to_xml(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropTag {
    pub namespace: String,
    pub name: String,
}

impl<'a> From<QName<'a>> for PropTag {
    fn from(qname: QName) -> Self {
        Self {
            namespace: match qname.prefix() {
                Some(prefix) => match String::from_utf8(Vec::from(prefix.into_inner())) {
                    Ok(prefix) => prefix,
                    Err(_) => "".to_string(),
                },
                None => "".to_string(),
            },
            name: match String::from_utf8(Vec::from(qname.local_name().into_inner())) {
                Ok(name) => name,
                Err(_) => "".to_string(),
            },
        }
    }
}

impl ToXml for PropTag {
    fn to_xml(&self) -> String {
        format!("<{}:{} />", self.namespace, self.name)
    }
}

#[derive(Debug, Clone)]
pub enum PropContent {
    Empty,
    Text(String),
    Props(Vec<Prop>),
}

#[derive(Debug, Clone)]
pub struct Prop {
    pub tag: PropTag,
    pub content: PropContent,
}

impl ToXml for Prop {
    fn to_xml(&self) -> String {
        let content = match &self.content {
            PropContent::Text(text) => text.clone(),
            PropContent::Props(props) => props
                .iter()
                .map(|prop| prop.to_xml())
                .collect::<Vec<String>>()
                .join(""),
            PropContent::Empty => "".to_string(),
        };
        format!(
            r#"<{}:{}>{}</{}:{}>"#,
            self.tag.namespace, self.tag.name, content, self.tag.namespace, self.tag.name
        )
    }
}

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
    pub props: Vec<PropTag>,
    pub depth: u8,
}

impl ToXml for PropFind {
    fn to_xml(&self) -> String {
        let props = self
            .props
            .iter()
            .map(|prop| prop.to_xml())
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
    pub props: Vec<Prop>,
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
    pub set_props: Vec<Prop>,
    pub remove_props: Vec<PropTag>,
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
