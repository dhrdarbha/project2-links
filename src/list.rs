#[allow(unused_imports)]
use std::{
    fmt::{self, Display, Formatter},
    mem,
};

#[derive(Debug, Default)]
pub enum ListNode<T> {
    #[default]
    Nil,
    Cons(T, Box<ListNode<T>>),
}

impl<T> ListNode<T> {
    /// Deletes a node from the list (given by starter)
    pub fn delete(&mut self) {
        let as_owned: ListNode<T> = mem::take(self);
        match as_owned {
            ListNode::Nil => {}
            ListNode::Cons(_, next) => {
                *self = *next;
            }
        }
    }

    /// Creates a new empty list
    pub fn new() -> Self {
        ListNode::Nil
    }

    /// Inserts a new list node with value `value` after `self` and returns a reference to the new node
    ///
    /// Uses `mem::take` to temporarily move ownership of the current node, then rebuild it.
    pub fn insert(&mut self, value: T) -> &mut Self {
        // Take ownership of the current node
        let mut owned = mem::take(self);
        match &mut owned {
            ListNode::Nil => {
                // If list is empty, just create one node
                owned = ListNode::Cons(value, Box::new(ListNode::Nil));
            }
            ListNode::Cons(_, next) => {
                // Traverse to the end of the list
                let mut cur = next.as_mut();
                loop {
                    match cur {
                        ListNode::Nil => {
                            *cur = ListNode::Cons(value, Box::new(ListNode::Nil));
                            break;
                        }
                        ListNode::Cons(_, ref mut next_node) => {
                            cur = next_node.as_mut();
                        }
                    }
                }
            }
        }
        // Write back to self
        *self = owned;
        self
    }

    /// Reverses the list in place (without cloning)
    pub fn reverse(&mut self) {
        // Take ownership of current list
        let mut cur = mem::take(self);
        let mut prev = ListNode::Nil;
        // Iteratively reverse using ownership
        loop {
            match cur {
                ListNode::Nil => {
                    *self = prev;
                    return;
                }
                ListNode::Cons(v, next) => {
                    let next_unboxed = *next;
                    let new_prev = ListNode::Cons(v, Box::new(prev));
                    prev = new_prev;
                    cur = next_unboxed;
                }
            }
        }
    }
}

/* ---------- Trait Implementations ---------- */

impl<T: PartialEq> PartialEq for ListNode<T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ListNode::Nil, ListNode::Nil) => true,
            (ListNode::Cons(a, an), ListNode::Cons(b, bn)) => a == b && an == bn,
            _ => false,
        }
    }
}

impl<T: Eq> Eq for ListNode<T> {}

impl<T: Display> Display for ListNode<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut cur = self;
        loop {
            match cur {
                ListNode::Nil => {
                    write!(f, "Nil")?;
                    break;
                }
                ListNode::Cons(v, next) => {
                    write!(f, "{} -> ", v)?;
                    cur = next;
                }
            }
        }
        Ok(())
    }
}

/* ---------- From Implementations ---------- */

impl<T> From<Vec<T>> for ListNode<T> {
    /// Builds a list preserving order: vec![1,2,3] -> 1 -> 2 -> 3 -> Nil
    fn from(v: Vec<T>) -> Self {
        let mut list = ListNode::new();
        let mut node = &mut list;
        for x in v {
            node = node.insert(x);
        }
        list
    }
}

impl<T> From<ListNode<T>> for Vec<T> {
    /// Consumes the list into a Vec in the same order.
    fn from(list: ListNode<T>) -> Self {
        let mut result = Vec::new();
        let mut cur = list;
        loop {
            match cur {
                ListNode::Nil => break,
                ListNode::Cons(v, next) => {
                    result.push(v);
                    cur = *next;
                }
            }
        }
        result
    }
}
