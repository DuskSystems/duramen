use alloc::vec::Vec;
use core::ops::Range;

use crate::NodeIndex;

/// Internal node storage.
#[derive(Debug, Clone)]
pub struct NodeData<T: Copy> {
    pub(crate) kind: T,
    pub(crate) start: u32,
    pub(crate) end: u32,
    pub(crate) first: NodeIndex,
    pub(crate) next: NodeIndex,
}

/// Concrete syntax tree.
#[derive(Debug, Clone)]
pub struct Tree<T: Copy> {
    pub(crate) nodes: Vec<NodeData<T>>,
    pub(crate) root: NodeIndex,
}

impl<T: Copy> Tree<T> {
    /// Returns the number of nodes in the tree.
    #[must_use]
    pub const fn len(&self) -> u32 {
        self.nodes.len() as u32
    }

    /// Returns `true` if the tree contains no nodes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Returns the root node, if present.
    #[must_use]
    pub fn root(&self) -> Option<Node<'_, T>> {
        Some(Node {
            tree: &self.nodes,
            index: self.root.get()?,
        })
    }

    /// Returns an iterator over the root-level nodes.
    #[must_use]
    pub fn children(&self) -> Children<'_, T> {
        Children {
            tree: &self.nodes,
            current: self.root.get(),
        }
    }
}

/// Reference to a node in the tree.
#[derive(Debug, Clone, Copy)]
pub struct Node<'a, T: Copy> {
    tree: &'a [NodeData<T>],
    index: usize,
}

impl<'a, T: Copy> Node<'a, T> {
    /// Returns the syntax kind of this node.
    #[must_use]
    #[inline(always)]
    pub fn kind(&self) -> T {
        self.tree[self.index].kind
    }

    /// Returns the byte range of this node.
    #[must_use]
    #[inline(always)]
    pub fn range(&self) -> Range<u32> {
        let data = &self.tree[self.index];
        data.start..data.end
    }

    /// Returns the source text for this node.
    #[must_use]
    #[inline(always)]
    pub fn text<'s>(&self, source: &'s str) -> &'s str {
        let data = &self.tree[self.index];
        &source[data.start as usize..data.end as usize]
    }

    /// Returns an iterator over this node's children.
    #[must_use]
    #[inline(always)]
    pub fn children(&self) -> Children<'a, T> {
        Children {
            tree: self.tree,
            current: self.tree[self.index].first.get(),
        }
    }
}

/// Iterator over sibling nodes.
#[derive(Debug, Clone)]
pub struct Children<'a, T: Copy> {
    tree: &'a [NodeData<T>],
    current: Option<usize>,
}

impl<'a, T: Copy> Iterator for Children<'a, T> {
    type Item = Node<'a, T>;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current?;
        let data = &self.tree[index];
        self.current = data.next.get();

        Some(Node {
            tree: self.tree,
            index,
        })
    }
}
