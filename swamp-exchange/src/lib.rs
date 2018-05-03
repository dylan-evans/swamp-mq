
use std::sync::{Arc, Weak, Mutex};
use std::rc::Rc;
use std::cell::Cell;
use std::collections::HashMap;


pub enum ThreadMode<T> {
    Single(Rc<Cell<T>>),
    Multi(Arc<Mutex<T>>),
//    MultiWeak(Weak<Mutex<T>>),
}

impl ThreadMode<Node> {
    pub fn clone(&self) -> ThreadMode<Node> {
        match self {
                &ThreadMode::Single(ref node) => ThreadMode::Single(node.clone()),
                &ThreadMode::Multi(ref node) => ThreadMode::Multi(node.clone()),
        }
    }
}

type NodeRc = ThreadMode<Node>;
type MsgRc = ThreadMode<Msg>;

pub enum MesgData {
    Bytes(Box<[u8]>),
    String(String),
}

/// Represents a node path from an `Exchange` root node
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Path {
    path: String,
}

impl Path {
    pub fn new(path: String) -> Path {
        Path {path}
    }

    pub fn new_str(path: &str) -> Path {
        Path::new(path.to_string())
    }

    pub fn root() -> Path {
        Path::new_str("")
    }

    pub fn split(&self) -> Vec<&str> {
        self.path.split('/').collect()
    }

    pub fn name(&self) -> String {
        let parts: Vec<&str> = self.path.rsplitn(2, '/').collect();
        parts[0].to_string()
    }

    pub fn parent(&self) -> Path {
        let parts: Vec<&str> = self.path.rsplitn(2, '/').collect();
        Path::new(parts[1].to_string())
    }
}

pub struct Node {
    path: Path,
    children: Vec<ThreadMode<Node>>,
    subscribers: Vec<ThreadMode<Node>>,
    parent: Option<ThreadMode<Node>>,
}

impl Node {
    pub fn new(path: Path, parent: Option<ThreadMode<Node>>) -> Node {
        Node {path, parent, children: Vec::new(), subscribers: Vec::new()}
    }

    pub fn new_single_threaded_root() -> ThreadMode<Node> {
        ThreadMode::Single(Rc::new(Cell::new(Node::new(Path::root(), Option::None))))
    }

    pub fn new_multi_threaded_root() -> ThreadMode<Node> {
        ThreadMode::Multi(Arc::new(Mutex::new(Node::new(Path::root(), Option::None))))
    }

    pub fn subscribe(&mut self, subscriber: ThreadMode<Node>) {
        match subscriber {
            ThreadMode::Single(r) => self.subscribers.push(ThreadMode::Single(r.clone())),
            ThreadMode::Multi(r) => self.subscribers.push(ThreadMode::Multi(r.clone()))
        };
    }

    pub fn notify(&mut self, mesg: ThreadMode<Msg>) {

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
    fn insert_node(&mut self, node: Node);
    fn get_node(&mut self, path: Path) -> Option<ThreadMode<Node>>;
    fn del_node(&mut self, path: Path);
    fn add_subscription(&mut self, node: Path, subscriber: Path);
    fn del_subscription(&mut self, node: Path, subscriber: Path);
    fn send_mesg(&mut self, mesg: Msg, path: Path);
}

pub struct SimpleExchange {
    root: ThreadMode<Node>,
    map: HashMap<Path, ThreadMode<Node>>,
}

impl SimpleExchange {
    fn new() -> SimpleExchange {
        SimpleExchange {
            root: Node::new_single_threaded_root(),
            map: HashMap::new(),
        }
    }
}

impl Exchange for SimpleExchange {
    fn create_node(&mut self, path: Path) {
        let parent = self.get_node(path.parent());
        self.insert_node(Node::new(path, parent))
    }

    fn insert_node(&mut self, node: Node) {
        let path = node.path.clone();
        let node = Rc::new(Cell::new(node));
        self.map.insert(path, ThreadMode::Single(node));
    }

    fn get_node(&mut self, path: Path) -> Option<ThreadMode<Node>> {
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

