use std::{fmt::Debug, ops::Deref};

use crate::{linked_list::SingleLinkedList, route::RouteBuilder};

pub struct AdjacencyList<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    heads: Vec<(T, SingleLinkedList<T>)>,
}

impl<T> Deref for AdjacencyList<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    type Target = Vec<(T, SingleLinkedList<T>)>;

    fn deref(&self) -> &Self::Target {
        &self.heads
    }
}

impl<T> Debug for AdjacencyList<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.heads.clone().into_iter().for_each(|(head, mut list)| {
            let mut route_builder = RouteBuilder::with_arrow();
            while let Some(val) = list.pop() {
                route_builder.push(format!("{:?}", val));
            }
            if route_builder.len() > 0 {
                writeln!(f, "{:?}: {}", head.clone(), route_builder.build()).unwrap();
            } else {
                writeln!(f, "{:?}: None", head.clone(),).unwrap();
            }
        });
        Ok(())
    }
}

#[derive(Debug)]
pub enum AdjacencyListOpError {
    HeadAlreadyExists,
    RequiredHeadNotExists,
}

impl<T> AdjacencyList<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    pub fn new() -> Self {
        Self { heads: Vec::new() }
    }

    pub fn add_head(&mut self, head_val: T) -> Result<(), AdjacencyListOpError> {
        if self.heads.iter().any(|(head, _)| *head == head_val) {
            Err(AdjacencyListOpError::HeadAlreadyExists)
        } else {
            self.heads.push((head_val, SingleLinkedList::new()));
            Ok(())
        }
    }

    pub fn append_to_head(&mut self, head_val: T, val: T) -> Result<(), AdjacencyListOpError> {
        if let Some((_, list)) = self.heads.iter_mut().find(|(head, _)| *head == head_val) {
            list.push(val);
            Ok(())
        } else {
            Err(AdjacencyListOpError::RequiredHeadNotExists)
        }
    }
}

#[test]
fn test_adjacency_list_print() {
    let mut adj_list = AdjacencyList::new();
    adj_list.add_head(1).unwrap();
    println!("{:?}", adj_list);

    adj_list.add_head(2).unwrap();
    println!("{:?}", adj_list);

    adj_list.append_to_head(1, 2).unwrap();
    adj_list.append_to_head(1, 3).unwrap();
    adj_list.append_to_head(1, 4).unwrap();
    adj_list.append_to_head(1, 5).unwrap();
    println!("{:?}", adj_list);
}
