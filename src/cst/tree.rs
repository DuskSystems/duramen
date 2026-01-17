use alloc::vec::Vec;
use core::ops::Range;

use super::index::NodeId;

#[derive(Debug, Clone, Copy)]
pub struct NodeData<T: Copy> {
    pub kind: T,
    pub start: u32,
    pub end: u32,
    pub parent: NodeId,
    pub previous: NodeId,
    pub next: NodeId,
    pub first: NodeId,
    pub(crate) last: NodeId,
}

#[derive(Debug, Clone)]
pub struct Tree<T: Copy> {
    pub nodes: Vec<NodeData<T>>,
    pub first: NodeId,
}

impl<T: Copy> Tree<T> {
    #[inline]
    #[must_use]
    pub fn first(&self) -> Option<Node<'_, T>> {
        if self.first == NodeId::NONE {
            None
        } else {
            Some(Node {
                tree: &self.nodes,
                index: self.first,
            })
        }
    }

    #[inline]
    #[must_use]
    pub fn walk(&self) -> Walk<'_, T> {
        Walk {
            tree: &self.nodes,
            current: self.first,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node<'a, T: Copy> {
    pub(crate) tree: &'a [NodeData<T>],
    pub(crate) index: NodeId,
}

impl<'a, T: Copy> Node<'a, T> {
    #[inline]
    #[must_use]
    pub fn value(&self) -> T {
        self.data().kind
    }

    #[inline]
    #[must_use]
    pub fn range(&self) -> Range<usize> {
        let data = self.data();
        data.start as usize..data.end as usize
    }

    #[inline]
    #[must_use]
    pub fn children(&self) -> Children<'a, T> {
        Children {
            tree: self.tree,
            current: self.data().first,
        }
    }

    #[inline]
    fn data(&self) -> &'a NodeData<T> {
        &self.tree[self.index.index()]
    }
}

#[derive(Debug, Clone)]
pub struct Children<'a, T: Copy> {
    tree: &'a [NodeData<T>],
    current: NodeId,
}

impl<'a, T: Copy> Iterator for Children<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == NodeId::NONE {
            return None;
        }

        let index = self.current;
        self.current = self.tree[index.index()].next;

        Some(Node {
            tree: self.tree,
            index,
        })
    }
}

#[derive(Debug, Clone)]
pub struct Walk<'a, T: Copy> {
    tree: &'a [NodeData<T>],
    current: NodeId,
}

impl<'a, T: Copy> Iterator for Walk<'a, T> {
    type Item = Node<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == NodeId::NONE {
            return None;
        }

        let index = self.current;
        let node = &self.tree[index.index()];

        if node.first == NodeId::NONE {
            let mut cursor = index;

            loop {
                let current = &self.tree[cursor.index()];

                if current.next != NodeId::NONE {
                    self.current = current.next;
                    break;
                }

                if current.parent == NodeId::NONE {
                    self.current = NodeId::NONE;
                    break;
                }

                cursor = current.parent;
            }
        } else {
            self.current = node.first;
        }

        Some(Node {
            tree: self.tree,
            index,
        })
    }
}
