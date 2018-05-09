
use std::sync::{Arc, Mutex};
use std::rc::Rc;
use std::cell::Cell;


pub enum ThreadMode<T> {
    Single(Rc<Cell<T>>),
    Multi(Arc<Mutex<T>>),
}


/// Represents a node path from an `Exchange` root node
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Path {
    pub path: String,
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


pub enum NodeLinkType {
    Parent,
    Child,
    Subscriber,
    Other(String),
}

pub struct NodeLink {
    pub to: NodeRef,
    pub relationship: NodeLinkType
}



pub trait Node {
    fn new(path: Path, parent: Option<NodeRef>) -> Self where Self: Sized;

    fn get_path(&self) -> Path;

    fn create_link(&mut self, to: NodeRef, relationship: NodeLinkType);

    fn new_single_threaded_root() -> AnyRef<Self> where Self: Sized;

    fn new_multi_threaded_root() -> AnyRef<Self> where Self: Sized;

}

pub type AnyRef<T> = ThreadMode<Box<T>>;
pub type NodeRef = AnyRef<Node>;

impl NodeRef {

    /// Get a copy of the nodes reference
    pub fn clone(&self) -> AnyRef<Node> {
        match self {
            &ThreadMode::Single(ref mode) => ThreadMode::Single(Rc::clone(mode)),
            &ThreadMode::Multi(ref mode) => ThreadMode::Multi(Arc::clone(mode)),
        }
    }

    pub fn get_path(&mut self) -> Path {
        match *self {
            ThreadMode::Single(ref mut node) => node.get_mut().get_path(),
            ThreadMode::Multi(ref node) => (*(*node).lock().unwrap()).get_path()
        }
    }


}


