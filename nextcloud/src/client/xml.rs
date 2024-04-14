#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlTag {
    pub namespace: String,
    pub name: String,
}

impl XmlTag {
    pub fn new(namespace: String, name: String) -> Self {
        Self { namespace, name }
    }

    pub fn full_name(&self) -> String {
        format!("{}:{}", self.namespace, self.name)
    }
}

impl<'a> From<quick_xml::name::QName<'a>> for XmlTag {
    fn from(qname: quick_xml::name::QName) -> Self {
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

#[derive(Debug, Clone, PartialEq)]
pub struct Attribute {
    pub key: String,
    pub value: String,
}

impl Attribute {
    pub fn new(key: String, value: String) -> Self {
        Self { key, value }
    }
}

impl ToString for Attribute {
    fn to_string(&self) -> String {
        format!("{}=\"{}\"", self.key.escape_default(), self.value)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Xml {
    pub tag: XmlTag,
    pub attributes: Vec<Attribute>,
    pub text: String,
    pub children: Vec<Xml>,
}

impl ToXml for Xml {
    fn to_xml(&self) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let children = self
            .children
            .iter()
            .map(|child| child.to_xml())
            .collect::<Vec<String>>()
            .join("");

        let tag_full_name = self.tag.full_name();
        if self.text.is_empty() {
            format!(
                "<{} {}>{}</{}>",
                tag_full_name, attributes, children, tag_full_name
            )
        } else {
            format!(
                "<{} {}>{}</{}>",
                tag_full_name, attributes, self.text, tag_full_name
            )
        }
    }
}

pub trait ToXml {
    fn to_xml(&self) -> String;
}
