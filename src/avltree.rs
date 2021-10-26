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

    pub fn add_inner(mut root: Link<T>, key: T) -> Link<T> {
        match root.take() {
            Some(node) => {
                {
                    let mut x = node.borrow_mut();
                    if key.lt(&x.key) {
                        x.left = Self::add_inner(x.left.take(), key)
                    } else if x.key.lt(&key) {
                        x.right = Self::add_inner(x.right.take(), key)
                    }
                }
                Some(node)
            }
            None => {
                Node::new(key)
            }
        }
    }

    pub fn rotate_left(mut root: Link<T>) -> Link<T> {
        root.take().map(|x| {
            let mut rch = x.borrow_mut().right.take().unwrap();
            let mut rchlch = rch.borrow_mut().left.take();
            x.borrow_mut().right = rchlch;
            rch.borrow_mut().left = Some(x);
            rch
        })
    }

    pub fn rotate_right(mut root: Link<T>) -> Link<T> {
        root.take().map(|x| {
            let mut lch = x.borrow_mut().left.take().unwrap();
            let mut lchrch = lch.borrow_mut().right.take();
            x.borrow_mut().left = lchrch;
            lch.borrow_mut().right = Some(x);
            lch
        })
    }

    pub fn add(&mut self, key: T) {
        self.root = Self::add_inner(self.root.take(), key)
    }

    fn find(root: &Link<T>, key: T) -> Link<T> {
        root.as_ref().and_then(|node| {
            let x = node.borrow();
            if key.lt(&x.key) {
                Self::find(&x.left, key)
            } else if x.key.lt(&key) {
                Self::find(&x.right, key)
            } else {
                Some(node.clone())
            }
        })
    }

    pub fn contains(&self, key: T) -> bool {
        Self::find(&self.root, key).is_some()
    }

    pub fn traverse_inner(node: &Link<T>) {
        node.as_ref().map(|x| {
            Self::traverse_inner(&x.as_ref().borrow().left);
            println!("=>{}", x.borrow().key);
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
        assert_eq!(t.contains(1), true);
        assert_eq!(t.contains(2), true);
        assert_eq!(t.contains(3), true);
        assert_eq!(t.contains(4), true);
        assert_eq!(t.contains(5), true);
        assert_eq!(t.contains(6), false);
        assert_eq!(t.contains(0), false);
    }
}