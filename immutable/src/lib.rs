pub mod h_macro;
pub mod l_macro;
pub mod s_macro;
pub mod v_macro;

pub use std::rc::Rc;

pub trait Seq<T: Clone> {
    fn first(&self) -> Rc<T>;
    fn rest(&self) -> Box<dyn Seq<T>>;
}

#[derive(Debug)]
pub struct List<T: Clone> {
    v: Vec<Rc<T>>,
}

impl<T: Clone> List<T> {
    pub fn new(vector: Vec<Rc<T>>) -> List<T> {
        List { v: vector }
    }
}

impl<T: Clone + 'static> Seq<T> for List<T> {
    fn first(&self) -> Rc<T> {
        self.v[0].clone()
    }
    fn rest(&self) -> Box<dyn Seq<T>> {
        Box::new(List::new(self.v[1..self.v.len()].to_vec()))
    }
}

#[derive(Debug)]
pub struct Set<T: Clone> {
    v: Vec<Rc<T>>,
}

impl<T: Clone> Set<T> {
    pub fn new(vector: Vec<Rc<T>>) -> Set<T> {
        Set { v: vector }
    }
}

#[derive(Debug)]
pub struct Vector<T: Clone> {
    v: Vec<Rc<T>>,
}

impl<T: Clone> Vector<T> {
    pub fn new(vector: Vec<Rc<T>>) -> Vector<T> {
        Vector { v: vector }
    }
}

#[derive(Debug)]
pub struct Pair<T: Clone, U: Clone> {
    first: Rc<T>,
    second: Rc<U>,
}

impl<T: Clone, U: Clone> Pair<T, U> {
    pub fn new(first: Rc<T>, second: Rc<U>) -> Pair<T, U> {
        Pair { first, second }
    }
}

#[derive(Debug)]
pub struct Hashmap<T: Clone, U: Clone> {
    pairs: Vec<Pair<Rc<T>, Rc<U>>>,
}

impl<T: Clone, U: Clone> Hashmap<T, U> {
    pub fn new(pairs: Vec<Pair<Rc<T>, Rc<U>>>) -> Hashmap<T, U> {
        Hashmap { pairs }
    }
}
