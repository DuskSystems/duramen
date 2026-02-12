use alloc::vec::Vec;

use crate::syntax::Syntax;
use crate::tree::{NodeData, Tree};

/// Branch for walking nodes.
#[derive(Debug)]
#[repr(transparent)]
pub struct Branch(usize);

/// Checkpoint for wrapping nodes.
#[derive(Debug)]
#[repr(transparent)]
pub struct Checkpoint(Option<usize>);

/// Builder for the CST tree.
#[derive(Debug)]
pub struct Builder {
    nodes: Vec<NodeData>,
    parents: Vec<usize>,
    sibling: Option<usize>,
    root: Option<usize>,
    cursor: usize,
}

impl Builder {
    /// Creates a new builder.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            parents: Vec::new(),
            sibling: None,
            root: None,
            cursor: 0,
        }
    }

    /// Returns the current parent node index.
    fn parent(&self) -> Option<usize> {
        self.parents.last().copied()
    }

    /// Opens a new branch.
    pub fn open(&mut self, kind: Syntax) -> Branch {
        let index = self.nodes.len();
        let node = NodeData {
            kind,
            start: self.cursor,
            end: self.cursor,
            parent: self.parent(),
            first: None,
            next: None,
        };

        self.attach(index);
        self.nodes.push(node);
        self.parents.push(index);
        self.sibling = None;

        Branch(index)
    }

    /// Closes the branch.
    pub fn close(&mut self, branch: &Branch) {
        self.parents.pop();
        let node = &mut self.nodes[branch.0];
        node.end = self.cursor;
        self.sibling = Some(branch.0);
    }

    /// Opens a checkpoint.
    #[must_use]
    pub const fn checkpoint(&self) -> Checkpoint {
        Checkpoint(self.sibling)
    }

    /// Commits nodes since the checkpoint into a new parent.
    pub fn commit(&mut self, checkpoint: &Checkpoint, kind: Syntax) {
        let first = match checkpoint.0 {
            Some(previous) => {
                let node = &self.nodes[previous];
                node.next
            }
            None => match self.parent() {
                Some(parent_idx) => {
                    let parent = &self.nodes[parent_idx];
                    parent.first
                }
                None => self.root,
            },
        };

        let Some(first) = first else {
            let branch = self.open(kind);
            self.close(&branch);
            return;
        };

        let first_node = &self.nodes[first];
        let start = first_node.start;
        let end = self.cursor;

        let wrapper = self.nodes.len();
        let node = NodeData {
            kind,
            start,
            end,
            parent: self.parent(),
            first: Some(first),
            next: None,
        };

        self.nodes.push(node);

        let mut child = Some(first);
        while let Some(child_idx) = child {
            self.nodes[child_idx].parent = Some(wrapper);
            child = self.nodes[child_idx].next;
        }

        if let Some(previous) = checkpoint.0 {
            let prev_node = &mut self.nodes[previous];
            prev_node.next = Some(wrapper);
        }

        if let Some(parent_idx) = self.parent() {
            let parent = &mut self.nodes[parent_idx];
            if parent.first == Some(first) {
                parent.first = Some(wrapper);
            }
        }

        if self.root == Some(first) {
            self.root = Some(wrapper);
        }

        self.sibling = Some(wrapper);
    }

    /// Adds a new token node.
    pub fn token(&mut self, kind: Syntax, len: usize) {
        let index = self.nodes.len();
        let start = self.cursor;
        let end = start + len;
        self.cursor = end;

        let node = NodeData {
            kind,
            start,
            end,
            parent: self.parent(),
            first: None,
            next: None,
        };

        self.attach(index);
        self.nodes.push(node);
        self.sibling = Some(index);
    }

    fn attach(&mut self, index: usize) {
        if let Some(sibling_idx) = self.sibling {
            let sibling = &mut self.nodes[sibling_idx];
            sibling.next = Some(index);
        }

        if let Some(parent_idx) = self.parent() {
            let parent = &mut self.nodes[parent_idx];
            if parent.first.is_none() {
                parent.first = Some(index);
            }
        }

        if self.root.is_none() {
            self.root = Some(index);
        }
    }

    /// Constructs the tree.
    #[must_use]
    pub fn build(self, source: &str) -> Tree<'_> {
        Tree {
            source,
            nodes: self.nodes,
            root: self.root,
        }
    }
}
