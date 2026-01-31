use alloc::vec::Vec;

use crate::tree::{NodeData, Tree};

/// Builder for the CST tree.
#[derive(Debug)]
pub struct Builder<T: Copy> {
    nodes: Vec<NodeData<T>>,
    parents: Vec<usize>,
    sibling: Option<usize>,
    root: Option<usize>,
    cursor: usize,
}

impl<T: Copy> Builder<T> {
    /// Creates a new builder with a capacity hint.
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(capacity),
            parents: Vec::with_capacity(8),
            sibling: None,
            root: None,
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
            first: None,
            next: None,
        };

        self.attach(index);
        self.nodes.push(node);
        self.parents.push(index);
        self.sibling = None;
    }

    /// Closes the current branch.
    #[inline(always)]
    pub fn close(&mut self) {
        let Some(index) = self.parents.pop() else {
            debug_assert!(false, "Close without matching open");
            return;
        };

        self.nodes[index].end = self.cursor;
        self.sibling = Some(index);
    }

    /// Adds a new token node.
    #[inline(always)]
    pub fn token(&mut self, kind: T, len: usize) {
        let index = self.nodes.len();
        let start = self.cursor;
        let end = start + len;
        self.cursor = end;

        let node = NodeData {
            kind,
            start,
            end,
            first: None,
            next: None,
        };

        self.attach(index);
        self.nodes.push(node);
        self.sibling = Some(index);
    }

    /// Returns a checkpoint for use with [`wrap`](Self::wrap):
    /// - `None` wraps all nodes in scope,
    /// - `Some` wraps nodes after that point.
    #[must_use]
    #[inline(always)]
    pub const fn checkpoint(&self) -> Option<usize> {
        self.sibling
    }

    /// Wraps nodes since the checkpoint in a new parent.
    pub fn wrap(&mut self, checkpoint: Option<usize>, kind: T) {
        let first = match checkpoint {
            Some(previous) => self.nodes[previous].next,
            None => match self.parent() {
                Some(parent) => self.nodes[parent].first,
                None => self.root,
            },
        };

        let Some(first) = first else {
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
            first: Some(first),
            next: None,
        };

        self.nodes.push(node);

        if let Some(previous) = checkpoint {
            self.nodes[previous].next = Some(wrapper);
        }

        if let Some(parent) = self.parent() {
            let parent = &mut self.nodes[parent];
            if parent.first == Some(first) {
                parent.first = Some(wrapper);
            }
        } else if self.root == Some(first) {
            self.root = Some(wrapper);
        }

        self.sibling = Some(wrapper);
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
        if let Some(sibling) = self.sibling {
            self.nodes[sibling].next = Some(index);
        }

        if let Some(parent) = self.parent() {
            let parent = &mut self.nodes[parent];
            if parent.first.is_none() {
                parent.first = Some(index);
            }
        } else if self.root.is_none() {
            self.root = Some(index);
        }
    }
}
