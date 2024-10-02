use std::iter::Iterator;

pub struct Node<T: Copy> {
    pub val: T,
    children: Vec<Node<T>>,
}

impl<T: Copy> Node<T> {
    pub fn new(val: T) -> Self {
        Self{
            val,
            children: vec![],
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = &Node<T>> {
        self.children.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Node<T>> {
        self.children.iter_mut()
    }

    pub fn push(&mut self, val: T) {
        self.children.push(Node::<T>::new(val));
    }
}


// pub struct Tree<'a, T: Copy> {
//     pub root: &'a mut Node::<T>,
// }

// impl<T: Copy> Tree<T> {
//     pub fn new(val: T) -> Self {
//         Self { root: Some(Rc::new(Node::new(val))) }
//     }

//     pub fn root(&mut self) -> Option<Rc<Node<T>>> {
//         self.root.clone()
//     }
// }

