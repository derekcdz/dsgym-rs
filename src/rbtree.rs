use std::cmp::Ordering;
use std::fmt::Display;
use std::mem::{replace, swap};
use std::ptr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Color {
    Red,
    Black,
}

enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Node<K, V> {
    key: K,
    value: V,
    left: *mut Node<K, V>,
    right: *mut Node<K, V>,
    parent: *mut Node<K, V>,
    color: Color,
}

pub struct RBTreeMap<K, V> {
    size: usize,
    root: *mut Node<K, V>,
}

// An iterator over the entries of a RBTreeMap.
pub struct Iter<'a, K: 'a, V: 'a> {
    stack: Vec<*mut Node<K, V>>,
    phantom: *mut &'a Node<K, V>,
}

impl<K, V> Node<K, V> {
    pub fn new(key: K, value: V, color: Color) -> *mut Node<K, V> {
        Box::into_raw(Box::new(Node {
            key: key,
            value: value,
            left: ptr::null_mut(),
            right: ptr::null_mut(),
            parent: ptr::null_mut(),
            color: color,
        }))
        // use unsafe { Box::from_raw(node); } to destruct a Node
    }

    unsafe fn free_node(node: *mut Node<K, V>) {
        if !node.is_null() {
            Box::from_raw(node);
        }
    }

    unsafe fn left_of(node: *mut Node<K, V>) -> *mut Node<K, V> {
        if !node.is_null() {
            (*node).left
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn right_of(node: *mut Node<K, V>) -> *mut Node<K, V> {
        if !node.is_null() {
            (*node).right
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn parent_of(node: *mut Node<K, V>) -> *mut Node<K, V> {
        if !node.is_null() {
            (*node).parent
        } else {
            ptr::null_mut()
        }
    }

    unsafe fn set_color(node: *mut Node<K, V>, color: Color) {
        if !node.is_null() {
            (*node).color = color;
        }
    }

    unsafe fn is_red(node: *mut Node<K, V>) -> bool {
        !node.is_null() && (*node).color == Color::Red
    }
}

impl<'a, K, V> RBTreeMap<K, V> {
    /// Makes a new, empty `RBTreeMap`.
    pub fn new() -> RBTreeMap<K, V> {
        Self {
            size: 0,
            root: ptr::null_mut(),
        }
    }

    pub fn len(&self) -> usize {
        return self.size;
    }

    /// Clears the map, removing all elements from the map
    pub fn clear(&mut self) {
        *self = RBTreeMap::new();
    }

    /// Returns the value corresponding to the key
    pub fn get(&self, key: &K) -> Option<&V>
    where
        K: Ord,
    {
        unsafe {
            let node = self.search_node(key);
            if !node.is_null() {
                Some(&(*node).value)
            } else {
                None
            }
        }
    }

    /// Returns the key-value pair corresponding to the key
    pub fn get_key_value(&self, key: &K) -> Option<(&K, &V)>
    where
        K: Ord,
    {
        let node = self.search_node(key);
        unsafe {
            if !node.is_null() {
                return Some((&(*node).key, &(*node).value));
            } else {
                return None;
            }
        }
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
        let mut cur = self.root;
        let mut p = ptr::null_mut();
        let mut dir = Direction::Left;

        unsafe {
            while !cur.is_null() {
                p = cur;
                match key.cmp(&(*cur).key) {
                    Ordering::Less => {
                        cur = (*cur).left;
                        dir = Direction::Left;
                    }
                    Ordering::Greater => {
                        cur = (*cur).right;
                        dir = Direction::Right;
                    }
                    Ordering::Equal => {
                        break;
                    }
                }
            }
            if !p.is_null() {
                if p == cur {
                    // key already exists
                    let mut old_value = std::mem::replace(&mut (*cur).value, value);

                    Some(old_value) // old value is returned
                } else {
                    // new node inserting
                    let mut new_node = Node::new(key, value, Color::Red);
                    match dir {
                        Direction::Left => (*p).left = new_node,
                        Direction::Right => (*p).right = new_node,
                    }
                    (*new_node).parent = p;
                    self.fix_after_insertion(new_node);
                    Node::set_color(self.root, Color::Black);
                    self.size += 1;
                    None
                }
            } else {
                // empty tree case, set new root
                self.root = Node::new(key, value, Color::Black);
                self.size = 1;

                None
            }
        }
    }

    /// Removes a key from the map, returning the stored key and value if the key
    /// was previously in the map.
    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        K: Ord + Default,
        V: Default,
    {
        self.remove_entry(key).map(|e| e.1)
    }

    /// Removes a key from the map, returning the stored key and value if the key
    /// was previously in the map.
    pub fn remove_entry(&mut self, key: &K) -> Option<(K, V)>
    where
        K: Ord + Default,
        V: Default,
    {
        unsafe {
            let mut x = self.search_node(key);
            if x.is_null() {
                return None;
            }
            self.size -= 1;
            let result = Some((
                replace(&mut (*x).key, Default::default()),
                replace(&mut (*x).value, Default::default()),
            )); // can we move key and value out of Node without Default::default()?

            if !(*x).left.is_null() && !(*x).right.is_null() {
                let mut y = (*x).right;
                // find y, the successor of z
                while !(*y).left.is_null() {
                    y = (*y).left;
                }
                // copy data of y to x and then to remove the successor
                swap(&mut (*x).key, &mut (*y).key);
                swap(&mut (*x).value, &mut (*y).value);
                x = y;
            }

            if !(*x).left.is_null() || !(*x).right.is_null() {
                // x has only 1 child: replace and return
                // in this case, replacement must be red
                let mut replacement = (*x).left;
                if (*x).left.is_null() {
                    replacement = (*x).right;
                }

                let p = (*x).parent;

                (*replacement).parent = p;
                if !p.is_null() {
                    if x == Node::left_of(p) {
                        (*p).left = replacement;
                    } else {
                        (*p).right = replacement;
                    }
                } else {
                    self.root = replacement;
                }

                Node::set_color(replacement, Color::Black);
                Node::free_node(x);
            } else {
                // x is leaf: remove and fix
                if !Node::is_red(x) {
                    self.fix_after_deletion(x);
                }

                let p = (*x).parent;
                if !p.is_null() {
                    if x == Node::left_of(p) {
                        (*p).left = ptr::null_mut();
                    } else {
                        (*p).right = ptr::null_mut();
                    }
                } else {
                    self.root = ptr::null_mut();
                }
                Node::free_node(x);
            }
            Node::set_color(self.root, Color::Black);
            result
        }
    }

    pub fn iter(&'a self) -> Iter<'a, K, V> {
        let mut stack = Vec::new();
        let mut x = self.root;
        while !x.is_null() {
            stack.push(x);
            unsafe {
                x = (*x).left;
            }
        }

        Iter {
            stack,
            phantom: ptr::null_mut(),
        }
    }

    fn search_node(&self, key: &K) -> *mut Node<K, V>
    where
        K: Ord,
    {
        let mut cur = self.root;
        unsafe {
            while !cur.is_null() {
                match key.cmp(&(*cur).key) {
                    Ordering::Less => cur = (*cur).left,
                    Ordering::Greater => cur = (*cur).right,
                    Ordering::Equal => return cur,
                }
            }
        }
        ptr::null_mut()
    }

    // Assumes node and its right child are not null
    // Rotation here exchanges their colors.
    // When node.right is red, it will not violate rules of Red-Black tree.
    unsafe fn rotate_left(&mut self, node: *mut Node<K, V>) {
        if node.is_null() {
            return;
        }
        let x = node;
        let y = Node::right_of(x);
        assert!(!y.is_null());

        let p = (*x).parent;
        let ly = Node::left_of(y);

        (*x).right = ly;
        if !ly.is_null() {
            (*ly).parent = x;
        }
        (*y).left = x;
        (*y).parent = (*x).parent;
        (*x).parent = y;
        swap(&mut (*x).color, &mut (*y).color);

        if !p.is_null() {
            if (*p).left == x {
                (*p).left = y;
            } else {
                (*p).right = y;
            }
        } else {
            self.root = y;
        }
    }

    // Assumes node and its left child are not null
    // Rotation here exchanges their colors.
    // When node.left is red, it will not violate rules of Red-Black tree.
    unsafe fn rotate_right(&mut self, node: *mut Node<K, V>) {
        if node.is_null() {
            return;
        }
        let x = node;
        let y = Node::left_of(x);
        assert!(!y.is_null());

        let p = (*x).parent;
        let ry = Node::right_of(y);

        (*x).left = ry;
        if !ry.is_null() {
            (*ry).parent = x;
        }
        (*y).right = x;
        (*y).parent = (*x).parent;
        (*x).parent = y;
        swap(&mut (*x).color, &mut (*y).color);

        if !p.is_null() {
            if (*p).left == x {
                (*p).left = y;
            } else {
                (*p).right = y;
            }
        } else {
            self.root = y;
        }
    }

    unsafe fn fix_after_insertion(&mut self, node: *mut Node<K, V>) {
        if node.is_null() {
            return;
        }
        let mut x = node;
        Node::set_color(x, Color::Red);

        while !x.is_null() && self.root != x && (*Node::parent_of(x)).color == Color::Red {
            let p = Node::parent_of(x);
            let g = Node::parent_of(p);

            if p == Node::left_of(g) {
                //      g
                //     / \
                //    p   u
                //    |
                //    x
                // u and g may be null
                let mut u = Node::right_of(g);
                if Node::is_red(u) {
                    Node::set_color(p, Color::Black);
                    Node::set_color(u, Color::Black);
                    Node::set_color(g, Color::Red);
                    x = g;
                } else {
                    //      g              g
                    //     / \            / \
                    //    p   u          x   u
                    //     \            /
                    //      x    ==>   p
                    if x == Node::right_of(p) {
                        self.rotate_left(p);
                        x = p;
                    }
                    //      g             p
                    //     / \           / \
                    //    p   u         x   g
                    //   /                   \
                    //  x       ==>           u
                    self.rotate_right(g);
                }
            } else {
                // Symmetric case
                let mut u = Node::left_of(g);
                if Node::is_red(u) {
                    Node::set_color(p, Color::Black);
                    Node::set_color(u, Color::Black);
                    Node::set_color(g, Color::Red);
                    x = g;
                } else {
                    if x == Node::left_of(p) {
                        self.rotate_right(p);
                        x = p;
                    }
                    self.rotate_left(g);
                }
            }
        }
        Node::set_color(self.root, Color::Black);
    }

    unsafe fn fix_after_deletion(&mut self, node: *mut Node<K, V>) {
        let mut x = node;

        while x != self.root && !Node::is_red(x) {
            let p = Node::parent_of(x);
            if x == Node::left_of(p) {
                let mut y = Node::right_of(p);

                if Node::is_red(y) {
                    self.rotate_left(p);
                    y = Node::right_of(p);
                }

                if !Node::is_red(Node::left_of(y)) && !Node::is_red(Node::right_of(y)) {
                    // no red links to take, subtract 1 black depth from sibling,
                    // try to increase black depth of parent node in next iteration
                    Node::set_color(y, Color::Red);
                    x = p;
                } else {
                    // at least one of sibling's children is red, take this link
                    if Node::is_red(Node::left_of(y)) {
                        self.rotate_right(y);
                        y = Node::parent_of(y);
                    }

                    //          p              y
                    //         / \            / \
                    //        x   y          p  ry
                    //           / \        / \
                    //          ly ry  =>  x  ly
                    // dist   1  2  1      2  2  1
                    // dist: number of black links to root of this subtree
                    self.rotate_left(p);
                    Node::set_color(Node::right_of(y), Color::Black);
                    x = self.root;
                }
            } else {
                // symmetric case: x is right child of p
                let mut y = Node::left_of(p);
                if Node::is_red(y) {
                    self.rotate_right(p);
                    y = Node::left_of(p);
                }

                if !Node::is_red(Node::left_of(y)) && !Node::is_red(Node::right_of(y)) {
                    // no red links to take, subtract 1 black depth from sibling,
                    // try to increase black depth of parent node in next iteration
                    Node::set_color(y, Color::Red);
                    x = p;
                } else {
                    // at least one of sibling's children is red, take this link
                    if Node::is_red(Node::right_of(y)) {
                        self.rotate_left(y);
                        y = Node::parent_of(y);
                    }
                    self.rotate_right(p);
                    Node::set_color(Node::left_of(y), Color::Black);
                    x = self.root;
                }
            }
        }

        Node::set_color(x, Color::Black);
    }

    /// check if a tree rooted at node is a 2-3-4 tree,
    /// when it's 2-3-4 tree, returns the black height of root,
    /// when it's not, returns `None`
    fn is_234tree(node: *mut Node<K, V>) -> Option<usize> {
        if node.is_null() {
            return Some(0);
        }
        unsafe {
            let bh_left = Self::is_234tree((*node).left);
            let bh_right = Self::is_234tree((*node).right);
            if bh_left.is_none() || bh_right.is_none() {
                return None;
            } else if bh_left.unwrap() != bh_right.unwrap() {
                return None;
            }
            if Node::is_red(node) {
                return bh_left;
            } else {
                return Some(bh_left.unwrap() + 1);
            }
        }
    }
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            let node = self.stack.pop();
            if node.is_none() {
                return None;
            }
            let mut x = node.unwrap();
            if !(*x).right.is_null() {
                x = (*x).right;
                self.stack.push(x);
                while !(*x).left.is_null() {
                    x = (*x).left;
                    self.stack.push(x);
                }
            }
            node.map(|node| (&(*node).key, &(*node).value))
        }
    }
}

impl<K, V> Drop for RBTreeMap<K, V> {
    fn drop(&mut self) {
        // Uses a stack to record pointers of nodes to be freed.
        // Prevent invoking Node::drop() recursively.
        unsafe {
            let mut stack = Vec::new();
            if !self.root.is_null() {
                stack.push(self.root);
            }
            while let Some(ptr) = stack.pop() {
                if !(*ptr).left.is_null() {
                    stack.push((*ptr).left);
                }
                if !(*ptr).right.is_null() {
                    stack.push((*ptr).right);
                }
                Box::from_raw(ptr);
            }
        }
    }
}

impl<K, V> Drop for Node<K, V> {
    fn drop(&mut self) {}
}

#[cfg(test)]
mod tests {
    use super::RBTreeMap;

    #[quickcheck]
    fn is_234tree(v: Vec<i32>) -> bool {
        let mut map = RBTreeMap::new();

        for &x in v.iter() {
            map.insert(x, x);
            if RBTreeMap::is_234tree(map.root).is_none() {
                return false;
            }
        }
        for &x in v.iter() {
            map.remove(&x);
            if RBTreeMap::is_234tree(map.root).is_none() {
                return false;
            }
        }

        true
    }

    // #[test]
    // fn reproduce () {
    //     let mut map = RBTreeMap::new();
    //     let v = vec![1, -1, 2, -2, -3, 0];
    //     for &x in v.iter() {
    //         map.insert(x, x);
    //         assert!(RBTreeMap::is_234tree(map.root).is_some())
    //     }
    //     for &x in v.iter() {
    //         map.remove(&x);
    //         assert!(RBTreeMap::is_234tree(map.root).is_some())
    //     }
    // }
}
