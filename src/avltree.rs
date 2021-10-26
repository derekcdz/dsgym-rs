use std::cell::RefCell;
use std::cmp::{max, Ordering};
use std::rc::Rc;
use std::fmt::Display;

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

impl<T: Ord> AVLTree<T> {
    pub fn new() -> Self {
        AVLTree {
            root: None
        }
    }

    fn add_inner(mut root: Link<T>, key: T) -> Link<T> {
        match root.take() {
            Some(node) => {
                {
                    let mut x = node.borrow_mut();
                    match key.cmp(&x.key) {
                        Ordering::Less => x.left = Self::add_inner(x.left.take(), key),
                        Ordering::Greater => x.right = Self::add_inner(x.right.take(), key),
                        _ => return None
                    }
                    x.height = std::cmp::max(Self::tree_height(&x.left), Self::tree_height(&x.right)) + 1;
                }
                Self::balance(Some(node))
            }
            None => {
                Node::new(key)
            }
        }
    }

    fn rotate_left(mut root: Link<T>) -> Link<T> {
        root.take().map(|x| {
            let rch = x.borrow_mut().right.take().unwrap();
            let rchlch = rch.borrow_mut().left.take();
            {
                let mut x = x.borrow_mut();
                x.right = rchlch;
                x.height = max(Self::tree_height(&x.left), Self::tree_height(&x.right)) + 1;
            }
            {
                let mut y = rch.borrow_mut();
                y.left = Some(x);
                y.height = max(Self::tree_height(&y.left), Self::tree_height(&y.right) + 1);
            }
            rch
        })
    }

    fn rotate_right(mut root: Link<T>) -> Link<T> {
        root.take().map(|x| {
            let lch = {
                x.borrow_mut().left.take().unwrap()
            };
            let lchrch = {
                lch.borrow_mut().right.take()
            };
            {
                let mut x = x.borrow_mut();
                x.left = lchrch;
                x.height = max(Self::tree_height(&x.left), Self::tree_height(&x.right)) + 1;
            }
            {
                let mut y = lch.borrow_mut();
                y.right = Some(x);
                y.height = max(Self::tree_height(&y.left), Self::tree_height(&y.right) + 1);
            }
            lch
        })
    }

    fn tree_height(node: &Link<T>) -> i32 {
        node.as_ref().map_or(0, |x| x.borrow().height)
    }

    fn balance(root: Link<T>) -> Link<T> {
        if let Some(x) = root {
            let diff;
            {
                let x = x.borrow_mut();
                diff = Self::tree_height(&x.left) - Self::tree_height(&x.right);
            }
            if diff >= -1 && diff <= 1 {
                return Some(x);
            }
            match diff {
                -2 => {
                    let diff2;
                    {
                        let y = x.borrow().right.clone().unwrap();
                        let ly = &y.borrow().left;
                        let ry = &y.borrow().right;
                        diff2 = Self::tree_height(ly) - Self::tree_height(ry);
                    }
                    if diff2 > 0 {
                        let mut x = x.borrow_mut();
                        x.right = Self::rotate_right(x.right.take());
                    }
                    Self::rotate_left(Some(x))
                }
                2 => {
                    let diff2;
                    {
                        let y = x.borrow().left.clone().unwrap();
                        let ly = &y.borrow().left;
                        let ry = &y.borrow().right;
                        diff2 = Self::tree_height(ly) - Self::tree_height(ry);
                    }
                    if diff2 < 0 {
                        let mut x = x.borrow_mut();
                        x.left = Self::rotate_left(x.left.take());
                    }
                    Self::rotate_right(Some(x))
                }
                _ => unreachable!(),
            }
        } else {
            None
        }
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

    fn is_balanced(root: &Link<T>) -> bool {
        match root {
            Some(x) => {
                let x = x.borrow();
                let lh = Self::tree_height(&x.left);
                let rh = Self::tree_height(&x.right);
                let lch_ok = Self::is_balanced(&x.left);
                let rch_ok = Self::is_balanced(&x.right);
                // println!("{} {} {} {} {}", lch_ok, rch_ok, x.height, lh, rh);
                lch_ok && rch_ok
                    && x.height == max(lh, rh) + 1
                    && lh - rh <= 1
                    && lh - rh >= -1
            }
            None => true
        }
    }
}

impl<T: Display> AVLTree<T> {
    pub fn traverse_inner(node: &Link<T>) {
        node.as_ref().map(|x| {
            Self::traverse_inner(&x.borrow().left);
            println!("=>{}", x.borrow().key);
            Self::traverse_inner(&x.borrow().right);
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
        assert_eq!(AVLTree::is_balanced(&t.root), true);
        t.add(5);
        assert_eq!(AVLTree::is_balanced(&t.root), true);
        t.add(3);
        assert_eq!(AVLTree::is_balanced(&t.root), true);
        t.add(2);
        assert_eq!(AVLTree::is_balanced(&t.root), true);
        t.add(4);
        assert_eq!(AVLTree::is_balanced(&t.root), true);
        assert_eq!(t.contains(1), true);
        assert_eq!(t.contains(2), true);
        assert_eq!(t.contains(3), true);
        assert_eq!(t.contains(4), true);
        assert_eq!(t.contains(5), true);
        assert_eq!(t.contains(6), false);
        assert_eq!(t.contains(0), false);
    }
}