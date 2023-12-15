use std::{
    collections::VecDeque,
    fmt::{Debug, Display},
};

use crate::{adjacency_list::AdjacencyList, route::RouteBuilder};

#[derive(Debug, Clone)]
pub struct BinarySortTree<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Display + Clone + Debug,
{
    root: Option<Box<Node<T>>>,
}

#[derive(Debug, Clone)]
pub struct Node<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    val: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Display + Clone + Debug,
{
    pub const fn new(val: T) -> Self {
        Self {
            val,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, val: T) {
        self.left = Some(Box::new(Node::new(val)));
    }

    pub fn set_right(&mut self, val: T) {
        self.right = Some(Box::new(Node::new(val)));
    }
}

#[derive(Debug)]
pub enum BinarySortTreeError {
    NodeNotFound,
    ValueAlreadyExisted,
}

impl<T> BinarySortTree<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Display + Clone + Debug,
{
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn insert(&mut self, val: T) -> Result<(), BinarySortTreeError> {
        if let Some(root) = &mut self.root {
            Self::insert_helper(root, val)
        } else {
            self.root = Some(Box::new(Node::new(val)));
            Ok(())
        }
    }

    fn insert_helper(node: &mut Box<Node<T>>, val: T) -> Result<(), BinarySortTreeError> {
        if val == node.val {
            return Err(BinarySortTreeError::ValueAlreadyExisted);
        }

        if val > node.val {
            if let Some(right) = &mut node.right {
                return Self::insert_helper(right, val);
            } else {
                node.set_right(val);
            }
        } else {
            if let Some(left) = &mut node.left {
                return Self::insert_helper(left, val);
            } else {
                node.set_left(val);
            }
        }
        Ok(())
    }

    /// Returns tree's height.
    pub fn height(&self) -> usize {
        if let Some(root) = &self.root {
            Self::get_height(root)
        } else {
            0
        }
    }

    fn get_height(node: &Box<Node<T>>) -> usize {
        let mut left_height = 0;
        let mut right_height = 0;
        if let Some(left_node) = &node.left {
            left_height += Self::get_height(left_node);
        }
        if let Some(right_node) = &node.right {
            right_height += Self::get_height(right_node);
        }
        usize::max(left_height + 1, right_height + 1)
    }

    /// Get all leaf nodes.
    pub fn leaves(&self) -> Option<Vec<T>> {
        if let Some(root) = &self.root {
            let mut leaves = vec![];
            Self::get_leaves(root, &mut leaves);
            Some(leaves)
        } else {
            None
        }
    }

    fn get_leaves(node: &Box<Node<T>>, leaves: &mut Vec<T>) {
        if let Some(left_node) = &node.left {
            Self::get_leaves(left_node, leaves);
        }
        if let Some(right_node) = &node.right {
            Self::get_leaves(right_node, leaves);
        }
        if node.left.is_none() && node.right.is_none() {
            leaves.push(node.val.clone())
        }
    }

    /// Inorder traversal.
    pub fn inorder_traversal(&self) -> String {
        if let Some(root) = &self.root {
            let mut route_builder = RouteBuilder::with_arrow();
            Self::inorder_traversal_helper(root, &mut route_builder);
            route_builder.build()
        } else {
            String::new()
        }
    }

    fn inorder_traversal_helper(node: &Box<Node<T>>, route_builder: &mut RouteBuilder) {
        if let Some(node) = &node.left {
            Self::inorder_traversal_helper(node, route_builder);
        }
        route_builder.push(format!("{}", node.val.clone()));
        if let Some(node) = &node.right {
            Self::inorder_traversal_helper(node, route_builder);
        }
    }

    pub fn contains(&self, value: &T) -> bool {
        if let Some(root) = &self.root {
            Self::contains_helper(root, &value)
        } else {
            false
        }
    }

    fn contains_helper(node: &Box<Node<T>>, value: &T) -> bool {
        if node.val == *value {
            return true;
        }
        if let Some(left_node) = &node.left {
            if Self::contains_helper(left_node, value) {
                return true;
            }
        }
        if let Some(right_node) = &node.right {
            if Self::contains_helper(right_node, value) {
                return true;
            }
        }
        false
    }

    pub fn get_addr(&self, value: &T) -> Option<usize> {
        if let Some(root) = &self.root {
            Self::get_addr_helper(root, value)
        } else {
            None
        }
    }

    fn get_addr_helper(node: &Box<Node<T>>, value: &T) -> Option<usize> {
        if node.val == *value {
            return Some(node.as_ref() as *const _ as usize);
        }
        if *value < node.val && node.left.is_some() {
            Self::get_addr_helper(node.left.as_ref().unwrap(), value)
        } else if *value > node.val && node.right.is_some() {
            Self::get_addr_helper(node.right.as_ref().unwrap(), value)
        } else {
            None
        }
    }

    pub fn remove(&mut self, target: &T) -> Result<(), BinarySortTreeError> {
        Self::remove_helper(&mut self.root, target)
    }

    fn remove_helper(
        node: &mut Option<Box<Node<T>>>,
        target: &T,
    ) -> Result<(), BinarySortTreeError> {
        if node.is_none() {
            return Err(BinarySortTreeError::NodeNotFound);
        }
        let node_val = &node.as_ref().unwrap().val;
        if *target < *node_val {
            Self::remove_helper(&mut node.as_mut().unwrap().left, target)
        } else if *target > *node_val {
            Self::remove_helper(&mut node.as_mut().unwrap().right, target)
        } else {
            let node_mut = node.as_mut().unwrap();
            if node_mut.left.is_some() && node_mut.right.is_none() {
                let left_node = node_mut.left.take();
                *node = left_node;
            } else if node_mut.right.is_some() && node_mut.left.is_none() {
                let right_node = node_mut.right.take();
                *node = right_node;
            } else if node_mut.right.is_none() && node_mut.left.is_none() {
                node.take();
            } else {
                node_mut.val = Self::get_max_node_val(&mut node_mut.left);
            }
            Ok(())
        }
    }

    fn get_max_node_val(node: &mut Option<Box<Node<T>>>) -> T {
        let node_mut = node.as_mut().unwrap();
        if node_mut.right.is_some() {
            Self::get_max_node_val(&mut node_mut.right)
        } else {
            let ret = node_mut.val.clone();
            let left_node = node_mut.left.take();
            *node = left_node;
            ret
        }
    }

    pub fn adjacency_list(&self) -> Option<AdjacencyList<T>> {
        if self.root.is_none() {
            return None;
        }
        let mut dequeue = VecDeque::new();
        dequeue.push_back(self.root.as_ref().unwrap());

        let mut adj_list = AdjacencyList::new();
        while dequeue.len() > 0 {
            let cur_node = dequeue.pop_front().unwrap();
            let cur_node_val = &cur_node.val;
            adj_list.add_head(cur_node_val.clone()).unwrap();
            if let Some(left_node) = &cur_node.left {
                dequeue.push_back(left_node);
                adj_list
                    .append_to_head(cur_node_val.clone(), left_node.val.clone())
                    .unwrap();
            }
            if let Some(right_node) = &cur_node.right {
                dequeue.push_back(right_node);
                adj_list
                    .append_to_head(cur_node_val.clone(), right_node.val.clone())
                    .unwrap();
            }
        }
        Some(adj_list)
    }

    pub fn topology_sort(&self) -> String {
        let mut route_builder = RouteBuilder::with_arrow();
        let adj_list = self.adjacency_list().unwrap();
        adj_list.clone().into_iter().for_each(|(head, mut list)| {
            if !route_builder.contains(&format!("{}", head)) {
                route_builder.push(format!("{}", head));
            }
            while let Some(item) = list.pop() {
                route_builder.push(format!("{}", item));
            }
        });
        route_builder.build()
    }
}
