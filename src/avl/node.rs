#[derive(Debug)]
pub(super) struct Node<T: Ord> {
    pub(super) height: i16, // height =  max(left.height, right.height) + 1
    pub(super) left: Option<Box<Node<T>>>, // Must be Box<Node>, otherwise we can't compute the struct size
    pub(super) right: Option<Box<Node<T>>>,
    pub(super) key: T,
}

impl<T: Ord> Node<T> {
    pub fn new(key: T) -> Self {
        Node {
            height: 0,
            left: None,
            right: None,
            key,
        }
    }

    pub fn update_heigh(&mut self) {
        let left_height = self.left.as_ref().map_or(-1, |n| n.height);
        let right_height = self.right.as_ref().map_or(-1, |n| n.height);
        self.height = std::cmp::max(left_height, right_height) + 1;
    }

    pub fn balance_facator(&self) -> i16 {
        let left_height = self.left.as_ref().map_or(-1, |n| n.height);
        let right_height = self.right.as_ref().map_or(-1, |n| n.height);
        return left_height - right_height
    }
}

// cfg -> configuration
// This is a conditional compilation.
// This peace of code will only be compiled when running "cargo test"
#[cfg(test)]
mod tests {
    use super::*;

    // Mark the function as a test. Rust will run it with "cargo test"
    #[test]
    fn test_node_creation() {
        let node = Node {
            height: 1,
            left: None,
            right: None,
            key: 67,
        };

        assert_eq!(node.key, 67);
    }

    #[test]
    fn test_node_with_children() {
        let left_child = Box::new(Node {
            height: 1,
            left: None,
            right: None,
            key: "left",
        });
        let right_child = Box::new(Node {
            height: 1,
            left: None,
            right: None,
            key: "right",
        });

        let parent = Node {
            height: 2,
            left: Some(left_child),
            right: Some(right_child),
            key: "parent",
        };

        // Notice the "as_ref".
        //
        // If we didn't use this function we would transfer ownership and it would be
        // impossible to later on use it again.
        //
        // By doing parent.left.unwrap() we are moving the Box<Node> out of parent.left
        // to a temporary unnamed variable that exists only for this expression which is
        // freed/destroyed at the end of the statement.
        // Similar to doing:
        // let child = parent.left.unwrap()
        // where child is now the new owner of left
        //
        // By using as_ref we make a reference instead of moving so that
        // parent.left is still the owner.
        assert_eq!(parent.left.as_ref().unwrap().key, "left");
        assert_eq!(parent.right.as_ref().unwrap().key, "right");
        assert_eq!(
            parent.height,
            std::cmp::max(
                parent.left.as_ref().unwrap().height,
                parent.right.as_ref().unwrap().height
            ) + 1
        );
    }
}
