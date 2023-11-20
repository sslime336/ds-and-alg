use std::{cell::RefCell, fmt::Display, ops::Add};

use crate::route::RouteBuilder;

#[derive(Debug, Clone)]
pub struct BinarySortTree<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Display + Clone,
{
    root: NodePtr<T>,
}

type NodePtr<T> = Box<RefCell<Node<T>>>;

#[derive(Debug)]
pub enum NodeOpError {
    NodeAlreadyExisted,
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone,
{
    val: T,
    left: Option<NodePtr<T>>,
    right: Option<NodePtr<T>>,
}

impl<T: PartialEq + Eq + PartialOrd + Ord + Display + Clone> Node<T> {
    pub const fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn as_node_ptr(self) -> NodePtr<T> {
        Box::new(RefCell::new(self))
    }

    #[allow(unused)]
    #[deprecated(note = "using trait `Clone` is just fine")]
    pub fn deep_clone(&self) -> NodePtr<T> {
        // let res = Box::new(RefCell::new(self.clone()));
        // Self::deep_clone_helper(&res)
        panic!("should using `.clone` rather than `.deep_clone`")
    }

    #[allow(unused)]
    fn deep_clone_helper(node: &NodePtr<T>) -> NodePtr<T> {
        let new_node = Node::new(node.borrow().val.clone()).as_node_ptr();
        if let Some(left_node) = node.borrow().left.as_ref() {
            (*new_node)
                .borrow_mut()
                .add_left(Self::deep_clone_helper(left_node))
                .unwrap();
        }
        if let Some(right_node) = node.borrow().left.as_ref() {
            (*new_node)
                .borrow_mut()
                .add_right(Self::deep_clone_helper(right_node))
                .unwrap();
        }
        new_node
    }

    pub fn add_left(&mut self, node: NodePtr<T>) -> Result<(), NodeOpError> {
        if let None = self.left {
            self.left = Some(node);
            Ok(())
        } else {
            Err(NodeOpError::NodeAlreadyExisted)
        }
    }

    pub fn add_right(&mut self, node: NodePtr<T>) -> Result<(), NodeOpError> {
        if let None = self.right {
            self.right = Some(node);
            Ok(())
        } else {
            Err(NodeOpError::NodeAlreadyExisted)
        }
    }
}

impl<T> BinarySortTree<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Display + Clone,
{
    pub fn new(root_val: T) -> Self {
        Self {
            root: Node::new(root_val).as_node_ptr(),
        }
    }

    pub fn insert(&mut self, val: T) {
        Self::insert_helper(&mut self.root, val);
    }

    fn insert_helper(node: &mut NodePtr<T>, val: T) {
        let mut node_mut = (**node).borrow_mut();

        if val > node_mut.val {
            if let Some(node) = node_mut.right.as_mut() {
                Self::insert_helper(node, val)
            } else {
                node_mut.add_right(Node::new(val).as_node_ptr()).unwrap();
            }
        }
        // assert that *node_val != *val
        else {
            if let Some(node) = node_mut.left.as_mut() {
                Self::insert_helper(node, val)
            } else {
                node_mut.add_left(Node::new(val).as_node_ptr()).unwrap();
            }
        }
    }

    pub fn remove(&mut self, target: &T) -> Result<(), ()> {
        if !self.contains(target) {
            return Err(());
        }

        Ok(())
    }

    pub fn contains(&self, target: &T) -> bool {
        Self::search_tree(&self.root, target)
    }

    fn search_tree(node: &NodePtr<T>, target: &T) -> bool {
        let cur_val = node.borrow().val.clone();
        let target_val = target.clone();
        if cur_val == target_val {
            return true;
        }
        if target_val < cur_val {
            if let Some(left_node) = node.borrow().left.as_ref() {
                Self::search_tree(left_node, target)
            } else {
                false
            }
        } else {
            if let Some(right_node) = node.borrow().right.as_ref() {
                Self::search_tree(right_node, target)
            } else {
                false
            }
        }
    }

    pub fn get_in_tree_address(&self, target: &T) -> Option<usize> {
        Self::in_tree_address_helper(&self.root, target)
    }

    fn in_tree_address_helper(node: &NodePtr<T>, target: &T) -> Option<usize> {
        let cur_val = node.borrow().val.clone();
        let target_val = target.clone();
        if cur_val == target_val {
            return Some(node.as_ptr() as usize);
        }
        if target_val < cur_val {
            if let Some(left_node) = node.borrow().left.as_ref() {
                Self::in_tree_address_helper(left_node, target)
            } else {
                None
            }
        } else {
            if let Some(right_node) = node.borrow().right.as_ref() {
                Self::in_tree_address_helper(right_node, target)
            } else {
                None
            }
        }
    }

    /// Returns tree's height.
    pub fn height(&self) -> usize {
        Self::get_height(&self.root) + 1
    }

    fn get_height(node: &NodePtr<T>) -> usize {
        let left_height = if let Some(left_node) = &node.borrow().left {
            Self::get_height(left_node) + 1
        } else {
            0
        };
        let right_height = if let Some(right_node) = &node.borrow().right {
            Self::get_height(right_node) + 1
        } else {
            0
        };
        usize::max(left_height, right_height)
    }

    /// Get all leaf nodes.
    pub fn leaves(&self) -> Vec<T> {
        let mut leaves = vec![];
        Self::get_leaves(&self.root, &mut leaves);
        leaves
    }

    fn get_leaves(node: &NodePtr<T>, leaves: &mut Vec<T>) {
        let len = leaves.len();
        if let Some(left_node) = node.borrow().left.as_ref() {
            Self::get_leaves(left_node, leaves);
        }
        if let Some(right_node) = node.borrow().right.as_ref() {
            Self::get_leaves(right_node, leaves);
        }
        // No child.
        if len == leaves.len() {
            leaves.push(node.borrow().val.clone())
        }
    }

    /// Inorder traversal.
    pub fn inorder_traversal(&self) -> String {
        let mut route_builder = RouteBuilder::with_arrow();

        Self::traversal_helper(&self.root, &mut route_builder);

        route_builder.build()
    }

    fn traversal_helper(node: &NodePtr<T>, route_builder: &mut RouteBuilder) {
        if let Some(node) = &node.borrow().left {
            Self::traversal_helper(node, route_builder);
        }
        route_builder.push(format!("{}", node.borrow().val));
        if let Some(node) = &node.borrow().right {
            Self::traversal_helper(node, route_builder);
        }
    }
}

#[test]
fn test_b_sort_tree() {
    let mut tree = BinarySortTree::new(10);
    tree.insert(2);
    tree.insert(5);
    tree.insert(3);
    tree.insert(6);
    tree.insert(4);

    let route = tree.inorder_traversal();
    let leaves = tree.leaves();
    let height = tree.height();

    println!("route: {}\nleaves: {:?}", route, leaves);

    assert_eq!("2 -> 3 -> 4 -> 5 -> 6 -> 10", route.as_str());
    assert_eq!(vec![4, 6], tree.leaves());
    assert!(height == 5, "height: {}", height);
}

#[test]
fn test_search() {
    let mut tree = BinarySortTree::new(10);
    tree.insert(2);
    tree.insert(5);
    tree.insert(3);
    tree.insert(6);
    tree.insert(4);

    let route = tree.inorder_traversal();
    println!("route: {}", route);

    let res = tree.contains(&1);
    println!("res(1): {}", res);

    let res = tree.contains(&10);
    println!("res(10): {}", res);

    let res = tree.contains(&2);
    println!("res(2): {}", res);

    let res = tree.contains(&3);
    println!("res(3): {}", res);

    let res = tree.contains(&6);
    println!("res(6): {}", res);

    let res = tree.contains(&55);
    println!("res(55): {}", res);
    println!("0x{:x}", tree.get_in_tree_address(&5).unwrap());
}
