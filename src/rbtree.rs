use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::{Rc, Weak};

type Child<K, V> = Option<Rc<RefCell<Node<K, V>>>>;
type Parent<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

enum Color {
    Red,
    Black,
}

struct Node<K, V> {
    key: K,
    val: V,
    left: Child<K, V>,
    right: Child<K, V>,
    parent: Parent<K, V>,
    color: Color,
}

pub struct RBTreeMap<K, V> {
    root: Child<K, V>,
    length: usize,
}

impl<K, V> RBTreeMap<K, V> {
    /// Makes a new, empty `RBTreeMap`.
    pub fn new() -> RBTreeMap<K, V> {
        Self {
            root: None,
            length: 0,
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
        todo!()
    }

    /// Returns `true` if the map contains a value for the specified key.
    pub fn contains_key(&self, key: &K) -> bool
    where
        K: Ord,
    {
        todo!()
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
}
