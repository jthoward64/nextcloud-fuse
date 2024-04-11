use super::{dav::DavError, prop::MultiStatus};

pub async fn pase_propfind(response: reqwest::Response) -> Result<MultiStatus, DavError> {
    let body = response.text().await.map_err(DavError::Network)?;
    let parsed = MultiStatus { responses: vec![] };

    let mut reader = quick_xml::Reader::from_str(&body);
    reader.trim_text(true);

    let mut stack = Vec::new();

    loop {
        match reader.read_event() {
            Err(e) => {
                return Err(DavError::XmlParse(e));
            }
            Ok(quick_xml::events::Event::Eof) => {
                break;
            }
            Ok(quick_xml::events::Event::Start(ref e)) => {
                stack.push(e.name());
            }
            Ok(quick_xml::events::Event::End(ref e)) => {
                stack.pop();
            }
            _ => {}
        }
    }
    Ok(parsed)
}
