use std::cell::{RefCell, Ref, RefMut};
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cmp::Ordering;

enum Color {
    Red,
    Black,
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
    nodes: Vec<Rc<Box<Node<K, V>>>>,
    free: Vec<usize>,
    size: usize,
    root: Option<None>
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
    pub fn get(&self, key: &K) -> Option<Ref<V>>
    where
        K: Ord,
    {
        // let node = RBTreeMap::search_node(&self.root, key);
        // node.map(|node| {
        //     Ref::map(node.borrow(), |node| &node.val)
        // })
        todo!()
    }

    /// Returns the key-value pair corresponding to the key
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        todo!()
    }

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
        todo!()
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

    fn search_node(&self, key: &K) -> Option<usize>
    where
        K: Ord,
    {
        let mut cur = &self.root;
        let mut _;

        while cur.is_some() {
            let cur_rc = cur.as_ref().unwrap();
            let cmp =  key.cmp(&cur_rc.borrow().key);
            match cmp {
                Ordering::Less => {
                    (cur, _) = Ref::map_split(cur_rc.borrow(), |x| (&x.left, _));
                },
                Ordering::Greater => {
                    // cur = &Ref::map(cur_rc.borrow(), |x| &x.right);
                    todo!()
                },
                Ordering::Equal => {
                    todo!()
                }
            }
        }

        todo!()
    }
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;

    #[test]
    fn basics() {

    }

    #[test]
    fn check_builtin() {
        let mut m = HashMap::new();
        m.insert("A", "A");
        m.insert("B", "B");
        m.get("A");
    }
}