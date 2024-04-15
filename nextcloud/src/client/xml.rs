use std::ptr;

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
enum XmlContent {
    Text(String),
    Xml(Vec<Xml>),
    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Xml {
    tag: XmlTag,
    attributes: Vec<Attribute>,
    content: XmlContent,
}

impl Xml {
    pub fn new(tag: XmlTag) -> Self {
        Self {
            tag,
            attributes: Vec::new(),
            content: XmlContent::Empty,
        }
    }

    pub fn with_attributes(&mut self, attributes: Vec<Attribute>) -> &mut Self {
        self.attributes = attributes;
        self
    }

    pub fn with_text(&mut self, text: String) -> &mut Self {
        self.content = XmlContent::Text(text);
        self
    }

    pub fn with_children(&mut self, children: Vec<Xml>) -> &mut Self {
        self.content = XmlContent::Xml(children);
        self
    }

    pub fn add_attribute(&mut self, attribute: Attribute) -> &mut Self {
        self.attributes.push(attribute);
        self
    }

    pub fn add_child(&mut self, child: Xml) -> &Xml {
        if self.is_xml() {
            match &mut self.content {
                XmlContent::Xml(children) => {
                    children.push(child);
                }
                _ => (),
            }
        } else {
            self.with_children(vec![child]);
        }

        self.children().unwrap().last().unwrap()
    }

    pub fn text(&self) -> Option<&String> {
        match &self.content {
            XmlContent::Text(text) => Some(text),
            _ => None,
        }
    }

    pub fn children(&self) -> Option<&Vec<Xml>> {
        match &self.content {
            XmlContent::Xml(children) => Some(children),
            _ => None,
        }
    }

    pub fn children_vec(&self) -> Vec<&Xml> {
        match &self.content {
            XmlContent::Xml(children) => children.iter().collect(),
            _ => Vec::new(),
        }
    }

    pub fn attributes(&self) -> &Vec<Attribute> {
        &self.attributes
    }

    pub fn tag(&self) -> &XmlTag {
        &self.tag
    }

    pub fn is_empty(&self) -> bool {
        match &self.content {
            XmlContent::Empty => true,
            _ => false,
        }
    }

    pub fn is_text(&self) -> bool {
        match &self.content {
            XmlContent::Text(_) => true,
            _ => false,
        }
    }

    pub fn is_xml(&self) -> bool {
        match &self.content {
            XmlContent::Xml(_) => true,
            _ => false,
        }
    }

    pub fn lookup(&mut self, target: *const Xml) -> Option<&mut Xml> {
        if ptr::eq(ptr::from_ref(self), target) {
            return Some(self);
        }

        match &mut self.content {
            XmlContent::Xml(children) => {
                for child in children {
                    if let Some(found) = child.lookup(target) {
                        return Some(found);
                    }
                }
            }
            _ => (),
        }

        None
    }
}

impl ToXml for Xml {
    fn to_xml(&self) -> String {
        let attributes = self
            .attributes
            .iter()
            .map(|attribute| attribute.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let tag_full_name = self.tag.full_name();

        if let Some(text) = &self.text() {
            return format!(
                "<{} {}>{}</{}>",
                tag_full_name, attributes, text, tag_full_name
            );
        } else if let Some(children) = &self.children() {
            let children_xml = children
                .iter()
                .map(|child| child.to_xml())
                .collect::<Vec<String>>()
                .join("");

            return format!(
                "<{} {}>{}</{}>",
                tag_full_name, attributes, children_xml, tag_full_name
            );
        } else {
            return format!("<{} {} />", tag_full_name, attributes);
        }
    }
}

pub trait ToXml {
    fn to_xml(&self) -> String;
}
