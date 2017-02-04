//! Reading an OSM XML file.

use std::{fmt, io, str};
use xml::attribute::OwnedAttribute;
use xml::reader::{Error, EventReader, Result, XmlEvent};
use ::elements::{Member, Node, Osm, Relation, Way};

pub fn read_xml<R: io::Read>(source: R) -> Result<Osm> {
    let mut reader = EventReader::new(source);
    while let XmlEvent::ProcessingInstruction{..} = reader.next()? {
    }
    if expect_element(&mut reader, "osm")?.is_none() {
        panic!("Got unexpected end element event");
    }
    read_document(&mut reader)
}

fn read_document<R: io::Read>(reader: &mut EventReader<R>) -> Result<Osm> {
    let mut res = Osm::new();
    loop {
        let (name, attrs) = match reader.next()? {
            XmlEvent::EndDocument => return Ok(res),
            XmlEvent::StartElement{name, attributes, ..} => {
                (name.local_name, attributes)
            }
            _ => return Err(Error::from((&*reader, "expected element"))),
        };
        match name.as_ref() {
            "node" => { res.add_node(read_node(attrs, reader)?); },
            "way" => { res.add_way(read_way(attrs, reader)?); },
            "relation" => { res.add_relation(read_relation(attrs, reader)?); },
            _ => { }
        }
    }
}

fn read_node<R: io::Read>(attrs: Vec<OwnedAttribute>,
                          reader: &mut EventReader<R>) -> Result<Node> {
    let (mut id, mut lat, mut lon) = (None, None, None);
    for item in attrs {
        match item.name.local_name.as_ref() {
            "id" => id = Some(item.value),
            "lat" => lat = Some(item.value),
            "lon" => lon = Some(item.value),
            _ => { }
        }
    }
    let id = from_attr(id, reader, "id")?;
    let lat = from_attr(lat, reader, "lat")?;
    let lon = from_attr(lon, reader, "lon")?;
    let mut node = Node::new(id, lat, lon);
    while let Some((name, attrs)) = expect_any_element(reader)? {
        if name == "tag" {
            let (k, v) = read_tag(attrs, reader)?;
            node.insert_tag(k, v);
        }
    }
    Ok(node)
}

fn read_way<R: io::Read>(attrs: Vec<OwnedAttribute>,
                         reader: &mut EventReader<R>) -> Result<Way> {
    let mut id = None;
    for item in attrs {
        match item.name.local_name.as_ref() {
            "id" => id = Some(item.value),
            _ => { }
        }
    }
    let id = from_attr(id, reader, "id")?;
    let mut way = Way::new(id);
    while let Some((name, attrs)) = expect_any_element(reader)? {
        match name.as_ref() {
            "nd" => {
                way.push_node(read_nd(attrs, reader)?);
            }
            "tag" => {
                let (k, v) = read_tag(attrs, reader)?;
                way.insert_tag(k, v)
            }
            _ => { }
        }
    }
    Ok(way)
}

fn read_relation<R: io::Read>(attrs: Vec<OwnedAttribute>,
                              reader: &mut EventReader<R>)
                              -> Result<Relation> {
    let mut id = None;
    for item in attrs {
        match item.name.local_name.as_ref() {
            "id" => id = Some(item.value),
            _ => { }
        }
    }
    let id = from_attr(id, reader, "id")?;
    let mut relation = Relation::new(id);
    while let Some((name, attrs)) = expect_any_element(reader)? {
        match name.as_ref() {
            "member" => {
                relation.push_member(read_member(attrs, reader)?);
            }
            "tag" => {
                let (k, v) = read_tag(attrs, reader)?;
                relation.insert_tag(k, v);
            }
            _ => { }
        }
    }
    Ok(relation)
}



fn read_tag<R: io::Read>(attrs: Vec<OwnedAttribute>,
                         reader: &mut EventReader<R>)
                         -> Result<(String, String)> {
    let (mut k, mut v) = (None, None);
    for item in attrs {
        match item.name.local_name.as_ref() {
            "k" => k = Some(item.value),
            "v" => v = Some(item.value),
            _ => { }
        }
    }
    let k = from_attr(k, reader, "k")?;
    let v = from_attr(v, reader, "v")?;
    Ok((k, v))
}

fn read_nd<R: io::Read>(attrs: Vec<OwnedAttribute>,
                        reader: &mut EventReader<R>) -> Result<i64> {
    let mut id = None;
    for item in attrs {
        match item.name.local_name.as_ref() {
            "ref" => id = Some(item.value),
            _ => { }
        }
    }
    let id = from_attr(id, reader, "ref")?;
    Ok(id)
}

fn read_member<R: io::Read>(attrs: Vec<OwnedAttribute>,
                            reader: &mut EventReader<R>) -> Result<Member> {
    let (mut mtype, mut id, mut role) = (None, None, None);
    for item in attrs {
        match item.name.local_name.as_ref() {
            "type" => mtype = Some(item.value),
            "ref" => id = Some(item.value),
            "role" => role = Some(item.value),
            _ => { }
        }
    }
    let mtype = from_attr(mtype, reader, "type")?;
    let id = from_attr(id, reader, "ref")?;
    let role = from_attr(role, reader, "role")?;
    Ok(Member::new(mtype, id, role))
}


//------------ Helpers -------------------------------------------------------

fn expect_element<R: io::Read>(reader: &mut EventReader<R>, elem: &str)
                               -> Result<Option<Vec<OwnedAttribute>>> {
    match reader.next()? {
        XmlEvent::StartElement{name, attributes, ..} => {
            if name.local_name == elem {
                Ok(Some(attributes))
            }
            else {
                Err(Error::from((&*reader,
                                 format!("expected element '{}'", elem))))
            }
        }
        XmlEvent::EndElement{..} => Ok(None),
        _ => return Err(Error::from((&*reader, "expected element"))),
    }
}

fn expect_any_element<R: io::Read>(reader: &mut EventReader<R>)
                            -> Result<Option<(String, Vec<OwnedAttribute>)>> {
    match reader.next()? {
        XmlEvent::StartElement{name, attributes, ..} => {
            Ok(Some((name.local_name, attributes)))
        }
        XmlEvent::EndElement{..} => Ok(None),
        _ => return Err(Error::from((&*reader, "expected element"))),
    }
}

fn from_attr<R, T>(val: Option<String>, reader: &EventReader<R>,
                   attr: &str) -> Result<T>
            where R: io::Read, T: str::FromStr, T::Err: fmt::Display {
    match val {
        Some(val) => {
            str::FromStr::from_str(&val)
                         .map_err(|err| Error::from((&*reader,
                                                     format!("{}", err))))
        }
        None => {
            Err(Error::from((&*reader,
                             format!("missing '{}' attribute", attr))))
        }
    }
}
