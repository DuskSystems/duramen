use alloc::vec::Vec;

use super::index::NodeId;
use super::tree::{NodeData, Tree};

#[derive(Debug, Clone, Copy)]
pub struct Checkpoint {
    index: u32,
    parent: NodeId,
}

#[derive(Debug)]
pub struct Builder<T: Copy> {
    nodes: Vec<NodeData<T>>,
    parent: NodeId,
    sibling: NodeId,
    cursor: u32,
    first: NodeId,
    last: NodeId,
}

impl<T: Copy> Builder<T> {
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            nodes: Vec::new(),
            parent: NodeId::NONE,
            sibling: NodeId::NONE,
            cursor: 0,
            first: NodeId::NONE,
            last: NodeId::NONE,
        }
    }

    pub fn open(&mut self, kind: T) {
        let index = self.alloc_index();
        let node = NodeData {
            kind,
            start: self.cursor,
            end: self.cursor,
            parent: self.parent,
            previous: self.sibling,
            next: NodeId::NONE,
            first: NodeId::NONE,
            last: NodeId::NONE,
        };

        self.link_node(index);
        self.nodes.push(node);
        self.parent = index;
        self.sibling = NodeId::NONE;
    }

    pub fn close(&mut self) {
        assert!(self.parent != NodeId::NONE, "close without matching open");

        let node = &mut self.nodes[self.parent.index()];
        node.end = self.cursor;

        self.sibling = self.parent;
        self.parent = node.parent;
    }

    #[expect(clippy::cast_possible_truncation, reason = "Cedar sources are < 4GB")]
    pub fn token(&mut self, kind: T, len: usize) {
        let index = self.alloc_index();
        let start = self.cursor;

        let end = start + len as u32;
        self.cursor = end;

        let node = NodeData {
            kind,
            start,
            end,
            parent: self.parent,
            previous: self.sibling,
            next: NodeId::NONE,
            first: NodeId::NONE,
            last: NodeId::NONE,
        };

        self.link_node(index);
        self.nodes.push(node);
        self.sibling = index;
    }

    fn link_node(&mut self, index: NodeId) {
        if self.sibling != NodeId::NONE {
            self.nodes[self.sibling.index()].next = index;
        }

        if self.parent == NodeId::NONE {
            if self.first == NodeId::NONE {
                self.first = index;
            }
            self.last = index;
        } else {
            let parent = &mut self.nodes[self.parent.index()];
            if parent.first == NodeId::NONE {
                parent.first = index;
            }
            parent.last = index;
        }
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_possible_truncation, reason = "Cedar sources are < 4GB")]
    pub const fn checkpoint(&self) -> Checkpoint {
        Checkpoint {
            index: self.nodes.len() as u32,
            parent: self.parent,
        }
    }

    pub fn close_at(&mut self, checkpoint: Checkpoint, kind: T) {
        assert!(
            checkpoint.parent == self.parent,
            "checkpoint from different nesting level"
        );

        let checkpoint_index = checkpoint.index;

        if checkpoint_index as usize >= self.nodes.len() {
            self.open(kind);
            self.close();
            return;
        }

        let mut first = NodeId::new(checkpoint_index);

        while self.nodes[first.index()].parent != checkpoint.parent {
            let parent = self.nodes[first.index()].parent;
            assert!(parent != NodeId::NONE, "node has no parent but should");
            first = parent;
        }

        let wrapper_index = self.alloc_index();

        let start = self.nodes[first.index()].start;
        let end = self.cursor;
        let previous = self.nodes[first.index()].previous;

        let wrapper = NodeData {
            kind,
            start,
            end,
            parent: checkpoint.parent,
            previous,
            next: NodeId::NONE,
            first,
            last: self.sibling,
        };

        self.nodes.push(wrapper);

        let mut current = first;
        while current != NodeId::NONE {
            let node = &mut self.nodes[current.index()];
            node.parent = wrapper_index;
            current = node.next;
        }

        self.nodes[first.index()].previous = NodeId::NONE;

        if previous != NodeId::NONE {
            self.nodes[previous.index()].next = wrapper_index;
        }

        if checkpoint.parent == NodeId::NONE {
            if self.first == first {
                self.first = wrapper_index;
            }

            if self.last == self.sibling {
                self.last = wrapper_index;
            }
        } else {
            let parent = &mut self.nodes[checkpoint.parent.index()];

            if parent.first == first {
                parent.first = wrapper_index;
            }

            if parent.last == self.sibling {
                parent.last = wrapper_index;
            }
        }

        self.sibling = wrapper_index;
    }

    #[must_use]
    pub fn build(self) -> Tree<T> {
        assert!(self.parent == NodeId::NONE, "unclosed nodes in tree");

        Tree {
            nodes: self.nodes,
            first: self.first,
        }
    }

    #[expect(clippy::cast_possible_truncation, reason = "Cedar sources are < 4GB")]
    const fn alloc_index(&self) -> NodeId {
        NodeId::new(self.nodes.len() as u32)
    }
}
