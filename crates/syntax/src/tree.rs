use alloc::string::String;
use alloc::vec::Vec;
use core::ops::Range;

use crate::syntax::Syntax;

/// Internal node storage.
#[derive(Clone, Debug)]
pub struct NodeData {
    pub(crate) kind: Syntax,
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) parent: Option<usize>,
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
            parent: None,
            first: None,
            next: None,
        }
    }
}

/// Concrete syntax tree.
#[derive(Clone, Debug)]
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
#[derive(Clone, Copy, Debug)]
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

    /// Finds the first child with the given syntax kind.
    #[must_use]
    pub fn child(&self, kind: Syntax) -> Option<Self> {
        self.children().find(|child| child.kind() == kind)
    }

    /// Returns children after the first occurrence of a child with the given kind.
    ///
    /// Skips all children up to and including the marker, then yields the rest.
    #[must_use]
    pub fn after(&self, kind: Syntax) -> Children<'a> {
        let mut iter = self.children();

        loop {
            match iter.next() {
                Some(child) if child.kind() == kind => break,
                Some(_) => {}
                None => break,
            }
        }

        iter
    }

    /// Returns the next sibling node.
    #[must_use]
    pub fn next(&self) -> Option<Self> {
        Some(Node {
            tree: self.tree,
            index: self.tree[self.index].next?,
        })
    }

    /// Returns the previous sibling node.
    #[must_use]
    pub fn previous(&self) -> Option<Self> {
        let parent_idx = self.tree[self.index].parent?;
        let parent = &self.tree[parent_idx];

        let mut prev = None;
        let mut current = parent.first;

        while let Some(idx) = current {
            if idx == self.index {
                return prev.map(|index| Node {
                    tree: self.tree,
                    index,
                });
            }

            prev = Some(idx);
            current = self.tree[idx].next;
        }

        None
    }

    /// Returns the parent node, if any.
    #[must_use]
    pub fn parent(&self) -> Option<Self> {
        let data = &self.tree[self.index];

        Some(Node {
            tree: self.tree,
            index: data.parent?,
        })
    }

    /// Returns an iterator over ancestor nodes from parent to root.
    #[must_use]
    pub fn ancestors(&self) -> Ancestors<'a> {
        Ancestors {
            tree: self.tree,
            current: self.tree[self.index].parent,
        }
    }

    /// Returns the first token (leftmost leaf) in this subtree.
    #[must_use]
    pub fn first(&self) -> Self {
        let mut node = *self;

        while let Some(child) = self.tree[node.index].first {
            node.index = child;
        }

        node
    }

    /// Returns the last token (rightmost leaf) in this subtree.
    #[must_use]
    pub fn last(&self) -> Self {
        let mut node = *self;

        while let Some(mut child) = self.tree[node.index].first {
            while let Some(sibling) = self.tree[child].next {
                child = sibling;
            }

            node.index = child;
        }

        node
    }

    /// Returns a preorder iterator over all nodes in this subtree.
    #[must_use]
    pub const fn descendants(&self) -> Descendants<'a> {
        Descendants {
            tree: self.tree,
            current: Some(self.index),
            root: self.index,
        }
    }

    /// Returns a preorder walk yielding enter and leave events.
    #[must_use]
    pub const fn preorder(&self) -> Preorder<'a> {
        Preorder {
            tree: self.tree,
            current: Some(self.index),
            root: self.index,
            entering: true,
        }
    }

    /// Returns `true` if this node or any descendant is a [`Syntax::Error`].
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.descendants().any(|node| node.kind().is_error())
    }

    /// Returns the deepest descendant whose range fully contains `range`.
    #[must_use]
    pub fn covering(&self, range: Range<usize>) -> Self {
        let mut node = *self;

        'descend: loop {
            for child in node.children() {
                let current = child.range();
                if current.start <= range.start && range.end <= current.end {
                    node = child;
                    continue 'descend;
                }
            }

            break;
        }

        node
    }
}

/// Iterator over child nodes.
#[derive(Clone, Debug)]
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

/// Iterator over ancestor nodes from parent to root.
#[derive(Clone, Debug)]
pub struct Ancestors<'a> {
    tree: &'a [NodeData],
    current: Option<usize>,
}

impl<'a> Iterator for Ancestors<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current?;
        let data = &self.tree[index];
        self.current = data.parent;

        Some(Node {
            tree: self.tree,
            index,
        })
    }
}

/// Preorder iterator over all nodes in a subtree.
#[derive(Clone, Debug)]
pub struct Descendants<'a> {
    tree: &'a [NodeData],
    current: Option<usize>,
    root: usize,
}

impl<'a> Iterator for Descendants<'a> {
    type Item = Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current?;
        let data = &self.tree[index];

        if let Some(first) = data.first {
            self.current = Some(first);
        } else {
            let mut start = index;

            self.current = loop {
                let current = &self.tree[start];
                if let Some(next) = current.next {
                    break Some(next);
                }

                if start == self.root {
                    break None;
                }

                start = current.parent?;
            };
        }

        Some(Node {
            tree: self.tree,
            index,
        })
    }
}

/// Event emitted during a preorder walk.
#[derive(Clone, Copy, Debug)]
pub enum WalkEvent<'a> {
    Enter(Node<'a>),
    Leave(Node<'a>),
}

/// Preorder walk yielding enter and leave events for each node.
#[derive(Clone, Debug)]
pub struct Preorder<'a> {
    tree: &'a [NodeData],
    current: Option<usize>,
    root: usize,
    entering: bool,
}

impl<'a> Iterator for Preorder<'a> {
    type Item = WalkEvent<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.current?;
        let data = &self.tree[index];
        let node = Node {
            tree: self.tree,
            index,
        };

        if self.entering {
            if data.first.is_some() {
                self.current = data.first;
                self.entering = true;
            } else {
                self.entering = false;
            }

            Some(WalkEvent::Enter(node))
        } else {
            if index == self.root {
                self.current = None;
            } else if let Some(next) = data.next {
                self.current = Some(next);
                self.entering = true;
            } else {
                self.current = data.parent;
                self.entering = false;
            }

            Some(WalkEvent::Leave(node))
        }
    }
}
