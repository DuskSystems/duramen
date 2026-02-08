use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::Syntax;

/// Internal node storage.
#[derive(Debug, Clone)]
pub struct NodeData {
    pub(crate) kind: Syntax,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) first: Option<usize>,
    pub(crate) next: Option<usize>,
}

impl NodeData {
    /// Creates a new node.
    #[must_use]
    pub const fn new(kind: Syntax, start: usize, end: usize) -> Self {
        Self {
            kind,
            start,
            end,
            first: None,
            next: None,
        }
    }
}

/// Concrete syntax tree.
#[derive(Debug, Clone)]
pub struct Tree {
    pub(crate) nodes: Vec<NodeData>,
    pub(crate) root: Option<usize>,
}

impl Tree {
    /// Creates a new tree.
    #[must_use]
    pub const fn new(nodes: Vec<NodeData>, root: Option<usize>) -> Self {
        Self { nodes, root }
    }

    /// Returns the root node, if present.
    #[must_use]
    pub fn root(&self) -> Option<Node<'_>> {
        Some(Node {
            tree: &self.nodes,
            index: self.root?,
        })
    }

    /// Returns an iterator over the root-level nodes.
    #[must_use]
    pub fn children(&self) -> Children<'_> {
        Children {
            tree: &self.nodes,
            current: self.root,
        }
    }

    /// Returns the number of nodes in the tree.
    #[must_use]
    pub const fn len(&self) -> usize {
        self.nodes.len()
    }

    /// Returns `true` if the tree contains no nodes.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    /// Reconstructs the source text from the CST.
    #[must_use]
    pub fn print(&self, source: &str) -> String {
        let mut output = String::new();

        for node in self.children() {
            if let Some(text) = source.get(node.range()) {
                output.push_str(text);
            }
        }

        output
    }
}

/// Reference to a node in the tree.
#[derive(Debug, Clone, Copy)]
pub struct Node<'a> {
    tree: &'a [NodeData],
    index: usize,
}

impl<'a> Node<'a> {
    /// Returns the syntax kind of this node.
    #[must_use]
    pub const fn kind(&self) -> Syntax {
        let node = &self.tree[self.index];
        node.kind
    }

    /// Returns the byte range of this node.
    #[must_use]
    pub const fn range(&self) -> Range<usize> {
        let data = &self.tree[self.index];
        data.start..data.end
    }

    /// Returns an iterator over this node's children.
    #[must_use]
    pub const fn children(&self) -> Children<'a> {
        let node = &self.tree[self.index];
        Children {
            tree: self.tree,
            current: node.first,
        }
    }
}

/// Iterator over child nodes.
#[derive(Debug, Clone)]
pub struct Children<'a> {
    tree: &'a [NodeData],
    current: Option<usize>,
}

impl<'a> Iterator for Children<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current?;
        let data = &self.tree[index];
        self.current = data.next;

        Some(Node {
            tree: self.tree,
            index,
        })
    }
}
