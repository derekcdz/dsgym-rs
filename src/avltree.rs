use std::cell::{Ref, RefMut, RefCell};
use std::rc::Rc;
use std::collections::BTreeSet;
use std::fmt::Display;
use std::mem;

pub struct Node<T> {
    key: T,
    left: Link<T>,
    right: Link<T>,
    height: i32,
}

pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;

pub struct AVLTree<T> {
    root: Link<T>,
}

impl<T> Node<T> {
    fn new(key: T) -> Link<T> {
        Some(Rc::new(RefCell::new(Node {
            key: key,
            left: None,
            right: None,
            height: 1,
        })))
    }
}

impl<T: PartialOrd + Display> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree {
            root: None
        }
    }

    pub fn add_inner(root: &mut Link<T>, key: T) -> Link<T> {
        match root.take() {
            Some(node) => {
                {
                    let mut x = node.as_ref().borrow_mut();
                    if key.lt(&x.key) {
                        x.left = Self::add_inner(&mut x.left.take(), key)
                    } else if x.key.lt(&key) {
                        x.right = Self::add_inner(&mut x.right.take(), key)
                    }
                }
                Some(node)
            }
            None => {
                Node::new(key)
            }
        }
    }

    pub fn rotate_left(root: &mut Link<T>) -> Link<T> {
        root.take().map(|x| {
            let mut rch = x.borrow_mut().right.take().unwrap();
            let mut rchlch = rch.borrow_mut().left.take().unwrap();
            x.borrow_mut().right = Some(rchlch);
            rch.borrow_mut().left = Some(x);
            rch
        })
    }

    pub fn rotate_right(root: &mut Link<T>) -> Link<T> {
        root.take().map(|x| {
            let mut lch = x.borrow_mut().left.take().unwrap();
            let mut lchrch = lch.borrow_mut().right.take().unwrap();
            x.borrow_mut().left = Some(lchrch);
            lch.borrow_mut().right = Some(x);
            lch
        })
    }

    pub fn add(&mut self, key: T) {
        self.root = Self::add_inner(&mut self.root.take(), key)
    }

    pub fn traverse_inner(node: &Link<T>) {
        node.as_ref().map(|x| {
            Self::traverse_inner(&x.as_ref().borrow().left);
            println!("=>{}", x.as_ref().borrow().key);
            Self::traverse_inner(&x.as_ref().borrow().right);
        });
    }

    pub fn traverse(&self) {
        Self::traverse_inner(&self.root);
    }
}


#[cfg(test)]
mod test {
    use crate::avltree::AVLTree;

    #[test]
    fn basics() {
        let mut t = AVLTree::new();
        t.add(1);
        t.add(5);
        t.add(3);
        t.add(2);
        t.add(4);
        t.traverse();
    }
}