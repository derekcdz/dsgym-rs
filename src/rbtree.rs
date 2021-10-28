use std::cell::{Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

enum Color {
    Red,
    Black,
}

enum Direction {
    Left,
    Right,
}

struct Node<K, V> {
    key: K,
    val: V,
    left: Option<usize>,
    right: Option<usize>,
    parent: Option<usize>,
    color: Color,
}

pub struct RBTreeMap<K, V> {
    nodes: Vec<Box<Node<K, V>>>,
    free: Vec<usize>,
    size: usize,
    root: Option<usize>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V, color: Color) -> Node<K, V> {
        Node {
            key: key,
            val: value,
            left: None,
            right: None,
            parent: None,
            color: color,
        }
    }
}

impl<K, V> RBTreeMap<K, V> {
    /// Makes a new, empty `RBTreeMap`.
    pub fn new() -> RBTreeMap<K, V> {
        Self {
            nodes: Vec::new(),
            free: Vec::new(),
            size: 0,
            root: None,
        }
    }

    /// Clears the map
    pub fn clear(&mut self) {
        *self = RBTreeMap::new();
    }

    /// Returns the value corresponding to the key
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord,
    {
        let node = self.search_node(key);
        node.map(|x| &x.val)
    }

    // /// Returns the key-value pair corresponding to the key
    // pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)>
    // where
    //     K: Ord,
    // {
    //     let node = self.search_node(key);
    //     node.map(|x| (&x.borrow().key, &x.borrow().val))
    // }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Ord,
    {
        self.get(key).is_some()
    }

    /// Inserts a key-value pair into the map.
    ///
    /// If the map did not have this key present, `None` is returned.
    ///
    /// If the map did have this key present, the value is updated, and the old
    /// value is returned.
    pub fn insert(&mut self, key: K, value: V) -> Option<V>
    where
        K: Ord,
    {
        let mut cur = self.root;
        let mut p = None;
        let mut dir = Direction::Left;

        while let Some(cur_index) = cur {
            p = cur;
            let x = self.nodes.get(cur_index).unwrap();
            match key.cmp(&x.key) {
                Ordering::Less => {
                    cur = x.left;
                    dir = Direction::Left;
                }
                Ordering::Greater => {
                    cur = x.right;
                    dir = Direction::Right;
                }
                Ordering::Equal => {
                    break;
                }
            }
        }
        match cur {
            Some(index) => {
                if p == cur {
                    // key already exists
                    let mut existing = self
                        .nodes
                        .get_mut(index)
                        .expect("Invalid index to nodes vector");
                    let mut old_value = value;
                    std::mem::swap(&mut old_value, &mut existing.val); // because V is not Copy

                    Some(old_value) // old value is returned
                } else {
                    // new node inserting
                    let next_index = self.next_index();
                    let mut parent_node = self
                        .nodes
                        .get_mut(p.unwrap())
                        .expect("Invalid index to nodes vector");
                    match dir {
                        Direction::Left => parent_node.left = Some(next_index),
                        Direction::Right => parent_node.right = Some(next_index),
                    }
                    let mut new_node = Box::new(Node::new(key, value, Color::Red));
                    new_node.parent = p;
                    self.push_node(new_node);
                    self.fix_after_insertion(next_index);
                    None
                }
            }
            None => {
                // empty tree case, set new root
                self.root = Some(self.push_node(Box::new(Node::new(key, value, Color::Black))));
                None
            }
        }
    }

    /// Removes a key from the map, returning the stored key and value if the key
    /// was previously in the map.
    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Ord,
    {
        todo!()
    }

    /// Removes a key from the map, returning the stored key and value if the key
    /// was previously in the map.
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)>
    where
        K: Ord,
    {
        todo!()
    }

    fn search_node(&self, key: &K) -> Option<&Box<Node<K, V>>>
    where
        K: Ord,
    {
        let mut cur = self.root;

        while let Some(cur_index) = cur {
            let cur_node = self.nodes.get(cur_index);
            match cur_node {
                Some(x) => {
                    match key.cmp(&x.key) {
                        Ordering::Less => cur = x.left,
                        Ordering::Greater => cur = x.right,
                        Ordering::Equal => return cur_node,
                    }
                }
                None => return None,
            }
        }
        None
    }

    fn fix_after_insertion(&mut self, index: usize) {
        todo!()
    }

    // Pushes a new node to inner node vector and returns index of the node
    fn push_node(&mut self, node: Box<Node<K, V>>) -> usize {
        let free_index = self.free.pop();
        match free_index {
            Some(index) => {
                self.nodes.insert(index, node);
                index
            }
            None => {
                self.nodes.push(node);
                self.nodes.len() - 1
            }
        }
    }

    // Returns the next index of node if a new node will be pushed to inner vector
    fn next_index(&self) -> usize {
        if !self.free.is_empty() {
            *self.free.last().unwrap()
        } else {
            self.nodes.len()
        }
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn basics() {}

    #[test]
    fn check_builtin() {
        let mut m = HashMap::new();
        m.insert("A", "A");
        m.insert("B", "B");
        m.get("A");
    }
}
