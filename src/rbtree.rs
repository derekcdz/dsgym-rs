use std::cell::RefCell;
use std::collections::BTreeSet;
use std::rc::{Rc, Weak};

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;
type WeakLink<K, V> = Option<Weak<RefCell<Node<K, V>>>>;

enum Color {
    Red,
    Black,
}

struct Node<K, V> {
    key: K,
    val: V,
    left: Link<K, V>,
    right: Link<K, V>,
    parent: WeakLink<K, V>,
    color: Color,
}

pub struct RBTreeMap<K, V> {
    root: Link<K, V>,
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


    fn search_node(root: &Link<K, V>) -> Link<K, V> {

    }
}
