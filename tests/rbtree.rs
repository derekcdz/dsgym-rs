use dsgym_rs::rbtree::RBTreeMap;
use std::collections::{BTreeMap, HashMap};

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

    assert_eq!(map.remove(&"B"), Some(2));
    assert_eq!(map.len(), 3);
    assert_eq!(map.remove(&"B"), None);
}

#[test]
fn iter() {
    let mut map = RBTreeMap::new();
    map.insert("D", 1);
    map.insert("C", 2);
    map.insert("B", 3);
    map.insert("A", 4);

    for (k, v) in map.iter() {
        println!("{} {}", k, v);
    }
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

#[quickcheck]
fn sorted_like_btreemap(v: Vec<i32>) -> bool {
    let mut btmap = BTreeMap::new();
    let mut rbtmap = RBTreeMap::new();

    for &x in v.iter() {
        btmap.insert(x, x);
        rbtmap.insert(x, x);
    }

    let mut rbtit = rbtmap.iter();
    for (k1, v1) in btmap.iter() {
        let x = rbtit.next();
        if let Some((k2, v2)) = x {
            if k1 != k2 || v1 != v2 {
                return false;
            }
            continue;
        }
        return false;
    }
    if rbtit.next().is_some() {
        return false;
    }
    true
}

#[quickcheck]
fn insert_and_remove(v: Vec<i32>) -> bool {
    let mut btmap = BTreeMap::new();
    let mut rbtmap = RBTreeMap::new();

    for &x in v.iter() {
        btmap.insert(x, x);
        rbtmap.insert(x, x);
    }

    for (&k1, &v1) in btmap.iter() {
        let x = rbtmap.remove_entry(&k1);
        if let Some((k2, v2)) = x {
            if k1 != k2 || v1 != v2 {
                return false;
            }
            continue;
        }
        return false;
    }
    if rbtmap.len() != 0 {
        return false;
    }
    true
}
