use std::collections::VecDeque;

use serde::{Deserialize, Serialize};

/// ```json
/// {
///     "value": "A",
///     "left": {
///         "value": "B",
///         "left": { "value": "D", "left": null, "right": null },
///         "right": { "value": "E", "left": null, "right": null }
///     },
///     "right": {
///         "value": "C", "left": { "value": "F", "left": null, "right": null },
///         "right": { "value": "G", "left": null, "right": null }
///     }
/// }
/// ```
/// [Source](https://hackernoon.com/how-to-insert-binary-tree-in-rust)
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BinaryTree<T> {
    pub value: T,
    pub left: Option<Box<BinaryTree<T>>>,
    pub right: Option<Box<BinaryTree<T>>>,
}

/// Uses Builder Lite pattern.
/// [See also](https://endler.dev/2017/boxes-and-trees/)
impl<T> BinaryTree<T>
where
    T: Copy,
{
    /// Create a balanced Binary tree from an array reference with breadth first traversal insertion
    /// algorithm.
    ///
    /// `BinaryTree::from_vec` takes a reference of an array just by borrowing.
    ///
    /// # Note
    ///
    /// * `Self::insert_breadth_first` takes ownership of the new value, to keep the program memory
    ///   saved,
    /// * Since the compiler doesn't allow us to "move" the array elements into this method .
    /// * Because the array can be referenced in other parts of the program.
    /// * So we pass in a copy (duplicate) of an array element.
    ///
    /// # Panics
    ///
    /// Panics if unwraps an enpty `vec` argument while splitting the first `root` from the `rest`.
    pub fn from_vec(vec: &[T]) -> Self {
        let (root, rest): (&T, &[T]) = vec.split_first().unwrap();
        let mut tree = BinaryTree::new(*root);
        rest.iter().for_each(|value: &T| tree.insert_breadth_first(*value));
        tree
    }

    /// Insert a tree node in the next available branch with breadth first traversal.
    ///
    /// Breadth First traversal insertion using Queue with a `VecDequeue`.
    ///
    /// # Examples
    ///
    /// ```
    /// use dialogue::*;
    ///
    /// fn main() {
    ///     let mut tree = BinaryTree::new(1);
    ///     assert_eq!(tree.value, 1);
    ///
    ///     let expect = BinaryTree::new(1);
    ///     assert_eq!(tree, expect);
    ///
    ///     tree.insert_breadth_first(2);
    ///     tree.insert_breadth_first(3);
    ///     tree.insert_breadth_first(4);
    ///     tree.insert_breadth_first(5);
    ///
    ///     let expect = BinaryTree::new(1)
    ///         .with_left(
    ///             BinaryTree::new(2).with_left(BinaryTree::new(4)).with_right(BinaryTree::new(5)),
    ///         )
    ///         .with_right(BinaryTree::new(3));
    ///     assert_eq!(tree, expect);
    /// }
    /// ```
    ///
    /// # Algorithm
    ///
    /// The algorithm forces the loop to visit sibling nodes first,
    /// from left to right, before visiting the next layer of child nodes.
    /// In each iteration check if either the left or right child is absent.
    /// If found, assign it as the next available spot for the new node.
    ///
    /// # Visual
    ///
    /// A simple binary tree example
    /// ```mermaid
    /// R(1)
    /// R-->L1(2)
    /// R-->R1(3)
    /// L1-->L2(4)
    /// L1-->R2(5)
    /// ```
    /// # Note
    ///
    /// * Pushing prepends an element to the deque.
    /// * Popping removes the last element from the deque, and return "it" if it is Some, or return
    ///   `None` if it is empty.
    /// * Pop the queue in the loop with mutable reference to `left` and `right` to insert new node.
    /// * During insertion, the `left` and `right` are `dereferenced` with `*` dereference operator,
    ///   to allow new node assignment.
    ///
    /// # Panics
    ///
    /// Panics if .
    pub fn insert_breadth_first(&mut self, new_value: T) {
        let mut queue: VecDeque<&mut BinaryTree<T>> = VecDeque::new();
        queue.push_front(self);

        loop {
            let BinaryTree { ref mut left, ref mut right, .. } = queue.pop_back().unwrap();

            match left {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *left = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }

            match right {
                Some(node) => {
                    queue.push_front(node);
                }
                None => {
                    *right = Some(Box::new(BinaryTree::new(new_value)));
                    return;
                }
            }
        }
    }

    /// Creates a new Binary Tree node.
    pub fn new(value: T) -> Self {
        Self { value, left: None, right: None }
    }

    /// Creates the left child of previous root node.
    pub fn with_left(mut self, node: BinaryTree<T>) -> Self {
        self.left = Some(Box::new(node));
        self
    }

    /// Creates the right child of previous root node.
    pub fn with_right(mut self, node: BinaryTree<T>) -> Self {
        self.right = Some(Box::new(node));
        self
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::{assert_eq, assert_ne};

    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Relationship<T> {
        pub value: T,
        pub left: Option<T>,
        pub right: Option<T>,
    }

    #[test]
    fn it_works_eq() {
        let (a, b) = (3, 1 + 2);
        assert_eq!(a, b);
        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }
    #[test]
    fn it_works_ne() {
        let (a, b) = (3, 2);
        assert_ne!(a, b);
        assert_ne!(a, b, "we are testing that the values are not equal");
    }

    #[test]
    fn it_create_new_tree() {
        let tree = BinaryTree::new(1);
        assert_eq!(tree.value, 1);
    }
    #[test]
    fn it_insert_left_node() {
        let expect = Relationship { value: 1, left: Some(2), right: None };
        let tree = BinaryTree::new(expect.value).with_left(BinaryTree::new(expect.left.unwrap()));

        assert_eq!(tree.value, expect.value);

        if let Some(node) = tree.left {
            assert_eq!(node.value, expect.left.unwrap());
        }

        assert_eq!(expect.right, None);
        assert_eq!(tree.right, None);
    }
    #[test]
    fn it_insert_right_node() {
        let expect = Relationship { value: 1, left: None, right: Some(3) };
        let tree = BinaryTree::new(expect.value).with_right(BinaryTree::new(expect.right.unwrap()));

        assert_eq!(tree.value, expect.value);

        if let Some(node) = tree.right {
            assert_eq!(node.value, expect.right.unwrap());
        }

        assert_eq!(expect.left, None);
        assert_eq!(tree.left, None);
    }

    #[test]
    fn it_insert_breadth_first() {
        let mut tree = BinaryTree::new(1);
        let expect = BinaryTree::new(1);
        assert_eq!(tree.value, 1);
        assert_eq!(tree, expect);

        tree.insert_breadth_first(2);
        tree.insert_breadth_first(3);
        tree.insert_breadth_first(4);
        tree.insert_breadth_first(5);

        let expect = BinaryTree::new(1)
            .with_left(
                BinaryTree::new(2).with_left(BinaryTree::new(4)).with_right(BinaryTree::new(5)),
            )
            .with_right(BinaryTree::new(3));
        assert_eq!(tree, expect);

        tree.insert_breadth_first(6);
        let expect = BinaryTree::new(1)
            .with_left(
                BinaryTree::new(2).with_left(BinaryTree::new(4)).with_right(BinaryTree::new(5)),
            )
            .with_right(BinaryTree::new(3).with_left(BinaryTree::new(6)));
        assert_eq!(tree, expect);
    }

    /// `BinaryTree::from_vec` takes a reference of an array just by borrowing.
    #[test]
    fn it_create_new_tree_from_vec() {
        let tree = BinaryTree::from_vec(&[1, 2, 3, 4, 5, 6]);
        let expect = BinaryTree::new(1)
            .with_left(
                BinaryTree::new(2).with_left(BinaryTree::new(4)).with_right(BinaryTree::new(5)),
            )
            .with_right(BinaryTree::new(3).with_left(BinaryTree::new(6)));
        assert_eq!(tree, expect);
    }
}
