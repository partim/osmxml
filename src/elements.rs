//! The elements of an Openstreetmap XML file.
//!
//! For now, the types only cotain a limited set of the most important
//! attributes.

use std::{borrow, hash};
use std::collections::{HashMap, HashSet};


//------------ Osm ----------------------------------------------------------

/// An OSM data set.
/// 
/// Contains a set each for nodes, ways, and relations.
pub struct Osm {
    nodes: HashSet<Node>,
    ways: HashSet<Way>,
    relations: HashSet<Relation>,
}

impl Osm {
    pub fn new() -> Self {
        Osm {
            nodes: HashSet::new(),
            ways: HashSet::new(),
            relations: HashSet::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> bool {
        self.nodes.insert(node)
    }

    pub fn add_way(&mut self, way: Way) -> bool {
        self.ways.insert(way)
    }

    pub fn add_relation(&mut self, rel: Relation) -> bool {
        self.relations.insert(rel)
    }
}

impl Osm {
    pub fn nodes(&self) -> &HashSet<Node> {
        &self.nodes
    }

    pub fn has_node(&self, id: u64) -> bool {
        self.nodes.contains(&id)
    }

    pub fn get_node(&self, id: u64) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn ways(&self) -> &HashSet<Way> {
        &self.ways
    }

    pub fn has_way(&self, id: u64) -> bool {
        self.ways.contains(&id)
    }

    pub fn get_way(&self, id: u64) -> Option<&Way> {
        self.ways.get(&id)
    }

    pub fn relations(&self) -> &HashSet<Relation> {
        &self.relations
    }

    pub fn has_relation(&self, id: u64) -> bool {
        self.relations.contains(&id)
    }

    pub fn get_relation(&self, id: u64) -> Option<&Relation> {
        self.relations.get(&id)
    }
}


//------------ Node ---------------------------------------------------------

pub struct Node {
    id: u64,
    lat: f64,
    lon: f64,
    tags: Tags,
}

impl Node {
    pub fn new(id: u64, lat: f64, lon: f64) -> Self {
        Node {
            id: id,
            lat: lat,
            lon: lon,
            tags: Tags::new()
        }
    }

    pub fn set_lat(&mut self, lat: f64) {
        self.lat = lat
    }

    pub fn set_lon(&mut self, lon: f64) {
        self.lon = lon
    }

    pub fn insert_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }
}

impl Node {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn lat(&self) -> f64 {
        self.lat
    }

    pub fn lon(&self) -> f64 {
        self.lon
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}

impl borrow::Borrow<u64> for Node {
    fn borrow(&self) -> &u64 {
        &self.id
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Node { }

impl hash::Hash for Node {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}


//------------ Way ----------------------------------------------------------

pub struct Way {
    id: u64,
    nodes: Vec<u64>,
    tags: Tags,
}

impl Way {
    pub fn new(id: u64) -> Self {
        Way {
            id: id,
            nodes: Vec::new(),
            tags: Tags::new(),
        }
    }

    pub fn push_node(&mut self, id: u64) {
        self.nodes.push(id)
    }

    pub fn insert_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value);
    }
}

impl Way {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn nodes(&self) -> &[u64] {
        &self.nodes
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}

impl borrow::Borrow<u64> for Way {
    fn borrow(&self) -> &u64 {
        &self.id
    }
}

impl PartialEq for Way {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Way { }

impl hash::Hash for Way {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}


//------------ Relation ------------------------------------------------------

pub struct Relation {
    id: u64,
    members: Vec<Member>,
    tags: Tags,
}

impl Relation {
    pub fn new(id: u64) -> Self {
        Relation {
            id: id,
            members: Vec::new(),
            tags: Tags::new(),
        }
    }

    pub fn push_member(&mut self, member: Member) {
        self.members.push(member)
    }

    pub fn insert_tag(&mut self, key: String, value: String) {
        self.tags.insert(key, value)
    }
}

impl Relation {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn members(&self) -> &[Member] {
        &self.members
    }

    pub fn tags(&self) -> &Tags {
        &self.tags
    }
}

impl borrow::Borrow<u64> for Relation {
    fn borrow(&self) -> &u64 {
        &self.id
    }
}

impl PartialEq for Relation {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Relation { }

impl hash::Hash for Relation {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}


//------------ Member --------------------------------------------------------

pub struct Member {
    mtype: MemberType,
    id: u64,
    role: String
}

impl Member {
    pub fn new(mtype: MemberType, id: u64, role: String) -> Self {
        Member {
            mtype: mtype,
            id: id,
            role: role
        }
    }
}

impl Member {
    pub fn mtype(&self) -> MemberType {
        self.mtype
    }
    
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn role(&self) -> &str {
        &self.role
    }
}


//------------ MemberType ----------------------------------------------------

#[derive(Clone, Copy, Debug)]
pub enum MemberType {
    Way,
    Node,
    Relation
}


//------------ Tags ----------------------------------------------------------

pub struct Tags(HashMap<String, String>);

impl Tags {
    pub fn new() -> Self {
        Tags(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.0.remove(key);
    }
}

impl Tags {
    pub fn has(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(AsRef::as_ref)
    }
}

