use std::collections::{HashMap, VecDeque};
use dsgym_rs::rbtree::RBTreeMap;

extern crate quickcheck;
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

#[test]
fn basics() {
    let mut map = RBTreeMap::new();
    map.insert("A", 1);
    map.insert("B", 2);
    map.insert("C", 3);
    map.insert("D", 4);
    assert_eq!(map.get(&"A"), Some(&1));
    assert_eq!(map.get(&"D"), Some(&4));
    assert_eq!(map.get(&"ZZZ"), None);
    assert_eq!(map.len(), 4);
    assert_eq!(map.contains_key(&"A"), true);
    assert_eq!(map.contains_key(&"ZZZ"), false);
}

#[quickcheck]
fn size_is_set_properly(v: Vec<i32>) -> bool {
    let mut hashmap = HashMap::new();
    let mut map = RBTreeMap::new();
    let mut it = v.iter();
    for x in v.iter() {
        map.insert(x, x);
        hashmap.insert(x, x);
    }
    hashmap.len() == map.len()
}