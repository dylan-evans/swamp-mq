
use std::sync::{Arc, Weak, Mutex};
use std::rc::Rc;
use std::cell::Cell;
use std::collections::HashMap;


pub enum ThreadMode<T> {
    Single(Rc<Cell<T>>),
    Multi(Arc<Mutex<T>>),
//    MultiWeak(Weak<Mutex<T>>),
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



pub trait Node {
    fn new(path: Path, parent: Option<ThreadMode<Self>>) -> Self where Self: Sized;

    fn new_single_threaded_root() -> ThreadMode<Self> where Self: Sized;

    fn new_multi_threaded_root() -> ThreadMode<Self> where Self: Sized;

    fn subscribe(&mut self, subscriber: ThreadMode<Self>) where Self: Sized;

    fn get_path(&self) -> Path;
}


