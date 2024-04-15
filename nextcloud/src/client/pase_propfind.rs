use std::ops::Deref;

use quick_xml::{events::Event, Reader};

use super::{
    dav::DavError,
    prop::{MultiStatus, MultiStatusResponse, PropStat, PropStatStatus, UnknownStatus},
    xml::{Xml, XmlTag},
};

pub fn pase_propfind(body: String) -> Result<MultiStatus, DavError> {
    let mut reader = Reader::from_str(&body);
    reader.trim_text(true);

    let mut multi_status: Option<MultiStatus> = None;

    let mut response: Option<MultiStatusResponse> = None;
    let mut propstat: Option<PropStat> = None;
    let mut propstat_status: Option<PropStatStatus> = None;

    let mut stack: Vec<XmlTag> = Vec::new();

    let mut prop_list_stack: Vec<*const Xml> = Vec::new();

    loop {
        match reader.read_event().unwrap() {
            Event::Start(e) => {
                let tag = XmlTag::from(e.name());

                if tag.namespace == "d" && tag.name == "multistatus" && multi_status.is_none() {
                    // d:multistatus is the root element
                    multi_status = Some(MultiStatus {
                        responses: Vec::new(),
                    });
                } else if tag.namespace == "d" && tag.name == "response" && response.is_none() {
                    // d:response is a child of multistatus
                    response = Some(MultiStatusResponse {
                        href: "".to_string(),
                        prop_stats: Vec::new(),
                        response_description: None,
                    });
                } else if tag.namespace == "d" && tag.name == "propstat" && propstat.is_none() {
                    // d:propstat is a child of response
                    propstat = Some(PropStat::new(PropStatStatus::Unknown(
                        UnknownStatus::Unknown,
                    )));
                } else if tag.namespace == "d" && tag.name == "status" && propstat_status.is_none()
                {
                    // d:status is a child of propstat
                    propstat_status = Some(PropStatStatus::Unknown(UnknownStatus::Unknown));
                } else if tag.namespace == "d" && tag.name == "response-description" {
                    // d:response-description is a child of response
                    // ...but we don't care about it for now
                } else if let Some(ref mut p) = propstat {
                    if tag.namespace == "d" && tag.name == "prop" && p.prop_list.is_empty() {
                        // d:prop is always the first child of propstat
                        p.prop_list.with_children(vec![]);
                        prop_list_stack.push(&p.prop_list);
                    } else if let Some(ref parent) = prop_list_stack.last() {
                        if let Some(mut_parent) = p.prop_list.lookup(**parent) {
                            // Anything else is a child of prop
                            prop_list_stack.push(mut_parent.add_child(Xml::new(tag.clone())));
                        } else {
                            return Err(DavError::InvariantViolation);
                        }
                    } else {
                        return Err(DavError::InvariantViolation);
                    }
                }

                stack.push(tag);
            }
            Event::End(e) => {
                let tag = XmlTag::from(e.name());

                if tag.namespace == "d" && tag.name == "response" {
                    // If we have a response, add it to the multi_status
                    if let Some(ref mut m) = multi_status {
                        if let Some(ref r) = response {
                            m.responses.push(r.clone());

                            response = None;
                        }
                    }
                } else if tag.namespace == "d" && tag.name == "propstat" {
                    // If we have a propstat, add it to the response
                    if let Some(ref mut r) = response {
                        if let Some(ref mut p) = propstat {
                            r.prop_stats.push(p.clone());

                            propstat = None;
                        }
                    }
                } else if tag.namespace == "d" && tag.name == "status" {
                    // If we have a status, add it to the propstat
                    if let Some(ref mut p) = propstat {
                        if let Some(ref mut s) = propstat_status {
                            p.status = s.clone();

                            propstat_status = None;
                        }
                    }
                } else if tag.namespace == "d" && tag.name == "response-description" {
                    // ignored for now
                } else if let Some(ref mut p) = propstat {
                    if let Some(ref parent) = prop_list_stack.last() {
                        if let Some(mut_parent) = p.prop_list.lookup(**parent) {
                            if mut_parent.tag().namespace == tag.namespace
                                && mut_parent.tag().name == tag.name
                            {
                                // Close the current prop
                                prop_list_stack.pop();
                            }
                        }
                    }
                }

                stack.pop();
            }
            Event::Empty(e) => {
                let tag = XmlTag::from(e.name());

                let this_prop = Xml::new(tag.clone());

                if let Some(ref mut p) = propstat {
                    if let Some(ref parent) = prop_list_stack.last() {
                        let prop_list_empty = p.prop_list.is_empty();
                        if let Some(mut_parent) = p.prop_list.lookup(**parent) {
                            if tag.namespace == "d" && tag.name == "prop" && prop_list_empty {
                                // d:prop is always the first child of propstat
                                p.prop_list.with_children(vec![this_prop]);
                            } else {
                                // Anything else is a child of prop
                                mut_parent.add_child(this_prop);
                            }
                        }
                    }
                }
            }
            Event::Text(e) => match stack.last() {
                Some(tag) => {
                    // d:href, d:status, and props can have text content

                    if tag.namespace == "d" && tag.name == "href" {
                        if let Some(ref mut r) = response {
                            r.href = match e.unescape() {
                                Ok(h) => h.to_string(),
                                Err(_) => "".to_string(),
                            };
                        }
                    } else if tag.namespace == "d" && tag.name == "status" {
                        if response.is_some() {
                            propstat_status = match e.unescape() {
                                Ok(h) => {
                                    let status = h.to_string();
                                    parse_prop_stat_code(status)
                                }
                                Err(_) => Some(PropStatStatus::Unknown(UnknownStatus::Unknown)),
                            };
                        }
                    } else if let Some(ref mut parent) = prop_list_stack.last() {
                        if let Some(ref mut p) = propstat {
                            if let Some(mut_parent) = p.prop_list.lookup(parent.clone().clone()) {
                                mut_parent.with_text(match e.unescape() {
                                    Ok(h) => h.to_string(),
                                    Err(_) => "".to_string(),
                                });
                            }
                        }
                    }
                }
                None => (),
            },
            Event::Eof => break,
            _ => (),
        }
    }

    match multi_status {
        Some(m) => Ok(m),
        None => Err(DavError::NoContent),
    }
}

fn parse_prop_stat_code(status: String) -> Option<PropStatStatus> {
    if status.starts_with("HTTP/1.1 ") {
        let status_code = status.split_whitespace().nth(1);
        match status_code {
            Some(val) => {
                let code = match val.parse::<u16>() {
                    Ok(c) => c,
                    Err(_) => 0,
                };

                match code {
                    200 => Some(PropStatStatus::Ok),
                    404 => Some(PropStatStatus::NotFound),
                    403 => Some(PropStatStatus::Forbidden),
                    401 => Some(PropStatStatus::Unauthorized),
                    _ => match status_code {
                        Some(val) => {
                            if val.starts_with("2") {
                                Some(PropStatStatus::Ok)
                            } else if val.starts_with("4") {
                                Some(PropStatStatus::Unknown(UnknownStatus::UnknownClientError))
                            } else if val.starts_with("5") {
                                Some(PropStatStatus::Unknown(UnknownStatus::UnknownServerError))
                            } else {
                                Some(PropStatStatus::Unknown(UnknownStatus::Unknown))
                            }
                        }
                        None => Some(PropStatStatus::Unknown(UnknownStatus::Unknown)),
                    },
                }
            }
            None => Some(PropStatStatus::Unknown(UnknownStatus::Unknown)),
        }
    } else {
        Some(PropStatStatus::Unknown(UnknownStatus::Unknown))
    }
}

#[test]
fn test_parse() {
    // let _ = pase_propfind(include_str!("../../../text.xml").to_string());
    println!(
        "{:#?}",
        pase_propfind(include_str!("../../../text.xml").to_string())
    );
}
