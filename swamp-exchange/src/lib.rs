
use std::sync::{Arc, Weak, Mutex};
use std::rc::Rc;
use std::cell::Cell;
use std::collections::HashMap;

mod node;

use node::{Node, Path, ThreadMode};
impl ThreadMode<SimpleNode> {
    /// Get a copy of the node reference.
    pub fn clone(&self) -> ThreadMode<SimpleNode> {
        match self {
                &ThreadMode::Single(ref node) => ThreadMode::Single(node.clone()),
                &ThreadMode::Multi(ref node) => ThreadMode::Multi(node.clone()),
        }
    }
}


type SimpleNodeRc = ThreadMode<SimpleNode>;
type MsgRc = ThreadMode<Msg>;

pub enum MesgData {
    Bytes(Box<[u8]>),
    String(String),
}

pub struct SimpleNode {
    path: Path,
    children: Vec<ThreadMode<SimpleNode>>,
    subscribers: Vec<ThreadMode<SimpleNode>>,
    parent: Option<ThreadMode<SimpleNode>>,
}

impl Node for SimpleNode {
    fn new(path: Path, parent: Option<ThreadMode<SimpleNode>>) -> SimpleNode {
        SimpleNode {path, parent, children: Vec::new(), subscribers: Vec::new()}
    }

    fn new_single_threaded_root() -> ThreadMode<Self> where Self: Sized {
        ThreadMode::Single(Rc::new(Cell::new(SimpleNode::new(Path::root(), Option::None))))
    }

    fn new_multi_threaded_root() -> ThreadMode<Self> where Self: Sized {
        ThreadMode::Multi(Arc::new(Mutex::new(SimpleNode::new(Path::root(), Option::None))))
    }

    fn subscribe(&mut self, subscriber: ThreadMode<Self>) where Self: Sized {
        match subscriber {
            ThreadMode::Single(r) => self.subscribers.push(ThreadMode::Single(r.clone())),
            ThreadMode::Multi(r) => self.subscribers.push(ThreadMode::Multi(r.clone()))
        };
    }

    fn get_path(&self) -> Path {
        return self.path.clone();
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
    fn create_node(&mut self, path: Path);
    fn insert_node(&mut self, node: SimpleNode);
    fn get_node(&mut self, path: Path) -> Option<ThreadMode<SimpleNode>>;
    fn del_node(&mut self, path: Path);
    fn add_subscription(&mut self, node: Path, subscriber: Path);
    fn del_subscription(&mut self, node: Path, subscriber: Path);
    fn send_mesg(&mut self, mesg: Msg, path: Path);
}

pub struct SimpleExchange {
    root: ThreadMode<SimpleNode>,
    map: HashMap<Path, ThreadMode<SimpleNode>>,
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
    fn create_node(&mut self, path: Path) {
        let parent = self.get_node(path.parent());
        self.insert_node(SimpleNode::new(path, parent))
    }

    fn insert_node(&mut self, node: SimpleNode) {
        let path = node.path.clone();
        let node = Rc::new(Cell::new(node));
        self.map.insert(path, ThreadMode::Single(node));
    }

    fn get_node(&mut self, path: Path) -> Option<ThreadMode<SimpleNode>> {
        if path == Path::root() {
            Some(self.root.clone())
        } else {
            match self.map.get(&path) {
                Some(&ThreadMode::Single(ref node)) => Some(ThreadMode::Single(node.clone())),
                Some(&ThreadMode::Multi(ref node)) => Some(ThreadMode::Multi(node.clone())),
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

