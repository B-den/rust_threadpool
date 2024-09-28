use std::rc::Rc;

pub struct Node<T: Copy> {
    pub val: T,
    next: Option<Rc<Node<T>>>,
}

pub struct Stack<T: Copy> {
    head: Option<Rc<Node<T>>>,
}

impl<T: Copy> Stack<T> {
    pub fn new() -> Self {
        Self {
            head: None,
        }
    }

    pub fn push(&mut self, val: &T) {
        let new_node = Rc::new(Node::<T>{
            val: *val,
            next: self.head.clone(),
        });

        self.head = Some(new_node);
    }
}

impl<T: Copy+std::fmt::Display> std::fmt::Display for Stack<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        fn resursive_print<T: Copy+std::fmt::Display>(f: &mut std::fmt::Formatter<'_>, node: &Option<Rc<Node<T>>>) -> Result<(), std::fmt::Error> {
            if let Some(node) = node {
                resursive_print(f, &node.next)?;
                return write!(f, "{},", node.val);
            };

            Ok(())
        }

        resursive_print(f, &self.head)
    }
}
