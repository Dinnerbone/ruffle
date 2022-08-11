//! Iterator types for XML trees

use crate::xml::XmlNode;
use ruffle_types::backend::Backend;

/// Iterator that yields direct children of an XML node.
pub struct ChildIter<'gc, B: Backend> {
    base: XmlNode<'gc, B>,
    index: usize,
    back_index: usize,
}

impl<'gc, B: Backend> ChildIter<'gc, B> {
    /// Construct a new `ChildIter` that lists the children of an XML node.
    pub fn for_node(base: XmlNode<'gc, B>) -> Self {
        Self {
            base,
            index: 0,
            back_index: base.children_len(),
        }
    }
}

impl<'gc, B: Backend> Iterator for ChildIter<'gc, B> {
    type Item = XmlNode<'gc, B>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.back_index {
            let item = self.base.get_child_by_index(self.index);
            self.index += 1;

            return item;
        }

        None
    }
}

impl<'gc, B: Backend> DoubleEndedIterator for ChildIter<'gc, B> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.index < self.back_index {
            self.back_index -= 1;
            let item = self.base.get_child_by_index(self.back_index);

            return item;
        }

        None
    }
}

/// Iterator that yields the ancestors of an XML node.
pub struct AnscIter<'gc, B: Backend> {
    next: Option<XmlNode<'gc, B>>,
}

impl<'gc, B: Backend> AnscIter<'gc, B> {
    /// Construct a new `AnscIter` that lists the parents of an XML node (including itself).
    pub fn for_node(node: XmlNode<'gc, B>) -> Self {
        Self { next: Some(node) }
    }
}

impl<'gc, B: Backend> Iterator for AnscIter<'gc, B> {
    type Item = XmlNode<'gc, B>;

    fn next(&mut self) -> Option<Self::Item> {
        let parent = self.next;

        if let Some(parent) = parent {
            self.next = parent.parent();
        }

        parent
    }
}
