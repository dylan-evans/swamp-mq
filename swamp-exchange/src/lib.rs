#![warn(dead_code, unused_imports)]
mod node;

use std::rc::Rc;
use std::cell::Cell;
use std::collections::HashMap;
use std::sync::{Mutex, Arc};

use node::*;


pub enum MesgData {
    Bytes(Box<[u8]>),
    String(String),
}

pub struct SimpleNode {
    path: Path,
    links: Vec<NodeLink>
}

impl AnyRef<SimpleNode> {
    pub fn clone(&self) -> AnyRef<SimpleNode> {
        match self {
            &ThreadMode::Single(ref mode) => ThreadMode::Single(Rc::clone(mode)),
            &ThreadMode::Multi(ref mode) => ThreadMode::Multi(Arc::clone(mode)),
        }
    }
}

impl Node for SimpleNode {
    fn new(path: Path, parent: Option<NodeRef>) -> SimpleNode {
        let node = SimpleNode {
            path,
            links: Vec::new()
        };

        match parent {
            Some(p) => node.links.push(
                NodeLink {to: p, relationship: NodeLinkType::Parent}),
            None => {}
        };

        node
    }

    fn new_single_threaded_root() -> AnyRef<Self> where Self: Sized {
        ThreadMode::Single(Rc::new(Cell::new(Box::new(
            SimpleNode {
                path: Path::root(),
                links: Vec::new()
            }))))
    }

    fn new_multi_threaded_root() -> AnyRef<Self> where Self: Sized {
        ThreadMode::Multi(Arc::new(Mutex::new(Box::new(
            SimpleNode {
                path: Path::root(),
                links: Vec::new()
            }))))
    }

    fn get_path(&self) -> Path {
        return self.path.clone();
    }

    fn create_link(&mut self, to: NodeRef, relationship: NodeLinkType) {
        self.links.push(NodeLink {to, relationship});
    }

}

pub struct Msg {
    data: MesgData,
    dest: Path
}

impl Msg {
    pub fn new(dest: Path, data: MesgData) -> Msg {
        Msg {dest, data}
    }
}

/// The Exchange trait represents the interface used to create a runtime with a particular
/// threading model.
pub trait Exchange {
    fn insert_node(&mut self, node: NodeRef);
    fn get_node(&mut self, path: Path) -> Option<NodeRef>;
    fn del_node(&mut self, path: Path);
    fn add_subscription(&mut self, node: Path, subscriber: Path);
    fn del_subscription(&mut self, node: Path, subscriber: Path);
    fn send_mesg(&mut self, mesg: Msg, path: Path);
}

pub struct SimpleExchange {
    root: AnyRef<SimpleNode>,
    map: HashMap<Path, NodeRef>,
}

impl SimpleExchange {
    fn new() -> SimpleExchange {
        SimpleExchange {
            root: SimpleNode::new_single_threaded_root(),
            map: HashMap::new(),
        }
    }
}

impl Exchange for SimpleExchange {
    fn insert_node(&mut self, node: NodeRef) {
        let path = node.get_path().clone();
        self.map.insert(path, node);
    }

    fn get_node(&mut self, path: Path) -> Option<NodeRef> {
        if path == Path::root() {
            //Some(self.root.clone())
            None
        } else {
            match self.map.get(&path) {
                Some(&noderef) => Some(noderef.clone()),
                None => None
            }
        }
    }

    fn del_node(&mut self, path: Path) {

    }

    fn add_subscription(&mut self, node: Path, subscriber: Path) {

    }

    fn del_subscription(&mut self, node: Path, subscriber: Path) {

    }

    fn send_mesg(&mut self, mesg: Msg, path: Path) {

    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_a_message() {
        let mut x = SimpleExchange::new();
        let mut m = Msg::new(Path::new_str("/foo/bar"), MesgData::String("Hello, World!".to_string()));

        x.create_node(Path::new_str("/foo"));
        x.send_mesg(m, Path::new_str("/foo/bar"));
    }

    #[test]
    fn path() {
        assert!(Path::root().path == "");
        assert!(Path::root() == Path::new_str(""));
        let p = Path::new_str("/foo/bar");
        assert!(p.path == "/foo/bar");
        assert!(p.name() == "bar");
        assert!(p.parent() == Path::new_str("/foo"));
        assert!(p.parent().parent() == Path::root());
    }

}

