use dsgym_rs::rbtree::RBTreeMap;

#[test]
fn test_rbtreemap() {
    let mut map = RBTreeMap::new();
    map.insert("A", 1);
    map.insert("B", 2);
    map.insert("C", 3);
    map.insert("D", 4);
    assert_eq!(map.get(&"A"), Some(&1));
    assert_eq!(map.get(&"D"), Some(&4));

}