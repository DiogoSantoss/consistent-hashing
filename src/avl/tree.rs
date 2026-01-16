use super::node::Node;

pub struct Tree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

// Ord is a trait bound. A constraint that tells Rust what we can do with that generic type.
impl<T: Ord + std::fmt::Debug> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }

    pub fn predecessor(&self, key: T) -> Option<&T> {

        let mut curr = self.root.as_ref();
        let mut last_node: Option<&Box<Node<T>>> = None;

        loop {
            match curr {
                None => {
                    match last_node {
                        None => return None,
                        Some(node) => {
                            if node.key < key {
                                return Some(&node.key);
                            } else {
                                return None
                            }
                        }
                    }
                },
                Some(node) => {
                    println!("Current key: {:?}",node.key);
                    if node.key > key {
                        curr = node.left.as_ref();
                    } else if node.key < key {
                        last_node = Some(node);
                        curr = node.right.as_ref();
                    } else {
                        return Some(&node.key);
                    }
                }
            }
        }
    }

    // pub fn successor(&self) {}

    pub fn insert(&mut self, key: T) {
        self.root = Self::insert_recursive(self.root.take(), key)
    }

    fn insert_recursive(node: Option<Box<Node<T>>>, key: T) -> Option<Box<Node<T>>> {
        match node {
            None => Some(Box::new(Node::new(key))),
            Some(mut curr) => {
                if curr.key > key {
                    curr.left = Self::insert_recursive(curr.left.take(), key)
                } else if curr.key < key {
                    curr.right = Self::insert_recursive(curr.right.take(), key)
                } else {
                    // Ignore duplicate keys
                }

                // Update node's height
                curr.update_heigh();

                // Check balance factor
                // Since tree was previously balanced, the worst case is absolute 2
                // -2 -> right-heavy
                // 2 -> left-heavy
                // -1/0/1 -> balanced
                let balance_factor = curr.balance_facator();

                if balance_factor == -2 {
                    // Right-heavy tree

                    let mut right_child = curr
                        .right
                        .take()
                        .expect("Right-heavy tree should have right child");

                    let right_right_height = right_child.right.as_ref().map_or(-1, |n| n.height);
                    let right_left_height = right_child.left.as_ref().map_or(-1, |n| n.height);

                    if right_right_height > right_left_height {
                        // Right-right heavy (left rotation)
                        /*
                            Before:          After:
                               3               2
                              / \             / \
                             A   2           3   1
                                / \         / \ / \
                               B   1        A B C D
                                  / \
                                 C   D
                        */

                        // move ownership of right_child.left to curr
                        curr.right = right_child.left.take();
                        curr.update_heigh();

                        // move ownership of curr to right_child
                        right_child.left = Some(curr);
                        right_child.update_heigh();

                        // return new "root"
                        return Some(right_child);
                    } else {
                        // Right-left heavy (double rotation: right on child, then left)
                        /*
                            Before:          After:
                               1               2
                              / \             / \
                             A   3           1   3
                                / \         / \ / \
                               2   D        A B C D
                              / \
                             B   C
                        */

                        let mut right_left_child = right_child.left.take().expect(
                            "Right-left heavy tree should have right child with left child",
                        );

                        curr.left = right_left_child.left.take();
                        curr.update_heigh();

                        right_child.left = right_left_child.right.take();
                        right_child.update_heigh();

                        right_left_child.left = Some(curr);
                        right_left_child.right = Some(right_child);
                        right_left_child.update_heigh();

                        return Some(right_left_child);
                    }
                } else if balance_factor == 2 {
                    // Left-heavy tree

                    let mut left_child = curr
                        .left
                        .take()
                        .expect("Left-heavy tree should have left child");

                    let left_left_height = left_child.left.as_ref().map_or(-1, |n| n.height);
                    let left_right_height = left_child.right.as_ref().map_or(-1, |n| n.height);

                    if left_left_height > left_right_height {
                        // Left-left heavy (right rotation)
                        /*
                            Before:        After:
                               3             2
                              / \           / \
                             2   D         1   3
                            / \           / \ / \
                           1   C          A B C D
                          / \
                         A   B
                        */

                        // move ownership of left_child.right to curr
                        curr.left = left_child.right.take();
                        curr.update_heigh();

                        // move ownership of curr to left_child
                        left_child.right = Some(curr);
                        left_child.update_heigh();

                        // retun new "root"
                        return Some(left_child);
                    } else {
                        // Left-right heavy (double rotation: left on child, then right)
                        /*
                            Before:          After:
                               3               2
                              / \             / \
                             1   D           1   3
                            / \             / \ / \
                           A   2            A B C D
                              / \
                             B   C
                        */

                        let mut left_right_child = left_child.right.take().expect(
                            "Left-right heavy tree should have left child with right child",
                        );

                        curr.right = left_right_child.right.take();
                        curr.update_heigh();

                        left_child.right = left_right_child.left.take();
                        left_child.update_heigh();

                        left_right_child.left = Some(left_child);
                        left_right_child.right = Some(curr);
                        left_right_child.update_heigh();

                        return Some(left_right_child);
                    }
                } else {
                    // Balanced
                }

                return Some(curr);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_tree_insert_node_key() {
        let mut tree = Tree::<String>::new();

        tree.insert("b".to_string());
        assert_eq!(tree.root.as_ref().unwrap().key, "b");

        tree.insert("a".to_string());
        tree.insert("c".to_string());
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.left.as_ref().unwrap().key, "a");
        assert_eq!(root.right.as_ref().unwrap().key, "c");
    }

    #[test]
    fn test_tree_insert_node_height() {
        let mut tree = Tree::<String>::new();

        tree.insert("b".to_string());
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.height, 0);

        tree.insert("a".to_string());
        tree.insert("c".to_string());
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.height, 1);
        assert_eq!(root.left.as_ref().unwrap().height, 0);
        assert_eq!(root.right.as_ref().unwrap().height, 0);

        tree.insert("d".to_string());
        let root = tree.root.as_ref().unwrap();
        assert_eq!(root.height, 2);
        assert_eq!(root.left.as_ref().unwrap().height, 0);

        let right = root.right.as_ref().unwrap();
        assert_eq!(right.height, 1);
        assert_eq!(right.right.as_ref().unwrap().height, 0);
    }

    #[test]
    fn test_tree_right_right_rotate_after_simple_insert() {
        let mut tree = Tree::<u8>::new();

        tree.insert(1);
        tree.insert(2);
        tree.insert(3);

        let root = tree.root.as_ref().unwrap();
        let left_child = root.left.as_ref().unwrap();
        let right_child = root.right.as_ref().unwrap();

        assert_eq!(root.key, 2);
        assert_eq!(root.height, 1);

        assert_eq!(left_child.key, 1);
        assert_eq!(left_child.height, 0);

        assert_eq!(right_child.key, 3);
        assert_eq!(right_child.height, 0);
    }

    #[test]
    fn test_tree_right_left_rotate_after_simple_insert() {
        let mut tree = Tree::<u8>::new();

        tree.insert(1);
        tree.insert(3);
        tree.insert(2);

        let root = tree.root.as_ref().unwrap();
        let left_child = root.left.as_ref().unwrap();
        let right_child = root.right.as_ref().unwrap();

        assert_eq!(root.key, 2);
        assert_eq!(root.height, 1);

        assert_eq!(left_child.key, 1);
        assert_eq!(left_child.height, 0);

        assert_eq!(right_child.key, 3);
        assert_eq!(right_child.height, 0);
    }

    #[test]
    fn test_tree_left_left_rotate_after_simple_insert() {
        let mut tree = Tree::<u8>::new();

        tree.insert(3);
        tree.insert(2);
        tree.insert(1);

        let root = tree.root.as_ref().unwrap();
        let left_child = root.left.as_ref().unwrap();
        let right_child = root.right.as_ref().unwrap();

        assert_eq!(root.key, 2);
        assert_eq!(root.height, 1);

        assert_eq!(left_child.key, 1);
        assert_eq!(left_child.height, 0);

        assert_eq!(right_child.key, 3);
        assert_eq!(right_child.height, 0);
    }

    #[test]
    fn test_tree_left_right_rotate_after_simple_insert() {
        let mut tree = Tree::<u8>::new();

        tree.insert(3);
        tree.insert(1);
        tree.insert(2);

        let root = tree.root.as_ref().unwrap();
        let left_child = root.left.as_ref().unwrap();
        let right_child = root.right.as_ref().unwrap();

        assert_eq!(root.key, 2);
        assert_eq!(root.height, 1);

        assert_eq!(left_child.key, 1);
        assert_eq!(left_child.height, 0);

        assert_eq!(right_child.key, 3);
        assert_eq!(right_child.height, 0);
    }

    #[test]
    fn test_predecessor() {
        let mut tree = Tree::<u8>::new();

        tree.insert(3);
        tree.insert(10);
        tree.insert(15);
        tree.insert(20);
        tree.insert(30);
        tree.insert(50);

        assert_eq!(tree.predecessor(2).is_none(), true);
        assert_eq!(*tree.predecessor(4).unwrap(),3);
        assert_eq!(*tree.predecessor(11).unwrap(),10);
        assert_eq!(*tree.predecessor(19).unwrap(),15);
        assert_eq!(*tree.predecessor(27).unwrap(),20);
        assert_eq!(*tree.predecessor(33).unwrap(),30);
        assert_eq!(*tree.predecessor(255).unwrap(),50);
    }
}
