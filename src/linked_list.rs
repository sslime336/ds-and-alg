//! Single linked list.

use std::fmt::Debug;

#[derive(Clone)]
pub struct SingleLinkedList<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    head: Option<Box<ListNode<T>>>,
    len: usize,
}

impl<T> Debug for SingleLinkedList<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SingleLinkedList")
            .field("head", &self.head)
            .field("len", &self.len)
            .finish()
    }
}

impl<T> SingleLinkedList<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    pub const fn new() -> Self {
        Self { head: None, len: 0 }
    }

    pub fn push(&mut self, val: T) {
        let mut new_node = ListNode::new(val);
        new_node.next = self.head.take();
        self.head.replace(Box::new(new_node));
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(mut head) = self.head.take() {
            let next = head.next.take();
            self.head = next;
            self.len -= 1;
            Some(head.val.clone())
        } else {
            None
        }
    }
}

#[derive(Clone)]
struct ListNode<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone,
{
    next: Option<Box<ListNode<T>>>,
    val: T,
}

impl<T> Debug for ListNode<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone + Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ListNode")
            .field("next", &self.next)
            .field("val", &self.val)
            .finish()
    }
}

impl<T> ListNode<T>
where
    T: PartialEq + Eq + PartialOrd + Ord + Clone,
{
    pub const fn new(val: T) -> Self {
        Self { next: None, val }
    }
}

#[test]
fn test_single_linked_list() {
    let mut list = SingleLinkedList::new();
    list.push(1);
    list.push(2);
    list.push(3);
    // dbg!(list);

    while let Some(value) = list.pop() {
        println!("value: {value}");
    }
}
