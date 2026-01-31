use alloc::vec::Vec;

use crate::NodeIndex;
use crate::tree::{NodeData, Tree};

/// Builder for the CST tree.
#[derive(Debug)]
pub struct Builder<T: Copy> {
    nodes: Vec<NodeData<T>>,
    parents: Vec<usize>,
    sibling: NodeIndex,
    root: NodeIndex,
    cursor: u32,
}

impl<T: Copy> Builder<T> {
    /// Creates a new builder with a capacity hint.
    #[must_use]
    pub fn new(capacity: u32) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity as usize),
            parents: Vec::with_capacity(8),
            sibling: NodeIndex::NONE,
            root: NodeIndex::NONE,
            cursor: 0,
        }
    }

    /// Opens a new branch.
    /// Call [`close`](Self::close) to commit the node.
    #[inline(always)]
    pub fn open(&mut self, kind: T) {
        let index = self.nodes.len();
        let node = NodeData {
            kind,
            start: self.cursor,
            end: self.cursor,
            first: NodeIndex::NONE,
            next: NodeIndex::NONE,
        };

        self.attach(index);
        self.nodes.push(node);
        self.parents.push(index);
        self.sibling = NodeIndex::NONE;
    }

    /// Closes the current branch.
    #[inline(always)]
    pub fn close(&mut self) {
        let Some(index) = self.parents.pop() else {
            debug_assert!(false, "Close without matching open");
            return;
        };

        self.nodes[index].end = self.cursor;
        self.sibling = NodeIndex::new(index);
    }

    /// Adds a new token node.
    #[inline(always)]
    pub fn token(&mut self, kind: T, len: u32) {
        let index = self.nodes.len();
        let start = self.cursor;
        let end = start + len;
        self.cursor = end;

        let node = NodeData {
            kind,
            start,
            end,
            first: NodeIndex::NONE,
            next: NodeIndex::NONE,
        };

        self.attach(index);
        self.nodes.push(node);
        self.sibling = NodeIndex::new(index);
    }

    /// Returns a checkpoint for use with [`wrap`](Self::wrap).
    #[must_use]
    #[inline(always)]
    pub const fn checkpoint(&self) -> NodeIndex {
        self.sibling
    }

    /// Wraps nodes since the checkpoint in a new parent.
    pub fn wrap(&mut self, checkpoint: NodeIndex, kind: T) {
        let first = match checkpoint.get() {
            Some(previous) => self.nodes[previous].next,
            None => match self.parent() {
                Some(parent) => self.nodes[parent].first,
                None => self.root,
            },
        };

        let Some(first) = first.get() else {
            self.open(kind);
            self.close();
            return;
        };

        let start = self.nodes[first].start;
        let end = self.cursor;

        let wrapper = self.nodes.len();
        let node = NodeData {
            kind,
            start,
            end,
            first: NodeIndex::new(first),
            next: NodeIndex::NONE,
        };

        self.nodes.push(node);

        let wrapper_index = NodeIndex::new(wrapper);

        if let Some(previous) = checkpoint.get() {
            self.nodes[previous].next = wrapper_index;
        }

        if let Some(parent) = self.parent() {
            let parent = &mut self.nodes[parent];
            if parent.first == NodeIndex::new(first) {
                parent.first = wrapper_index;
            }
        } else if self.root == NodeIndex::new(first) {
            self.root = wrapper_index;
        }

        self.sibling = wrapper_index;
    }

    /// Constructs the tree.
    #[must_use]
    pub fn build(self) -> Tree<T> {
        Tree {
            nodes: self.nodes,
            root: self.root,
        }
    }

    #[inline(always)]
    fn parent(&self) -> Option<usize> {
        self.parents.last().copied()
    }

    #[inline(always)]
    fn attach(&mut self, index: usize) {
        let node_index = NodeIndex::new(index);

        if let Some(sibling) = self.sibling.get() {
            self.nodes[sibling].next = node_index;
        }

        if let Some(parent) = self.parent() {
            let parent = &mut self.nodes[parent];
            if parent.first.is_none() {
                parent.first = node_index;
            }
        } else if self.root.is_none() {
            self.root = node_index;
        }
    }
}
