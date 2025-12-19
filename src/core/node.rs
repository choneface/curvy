use crate::core::{Rect, Widget};

/// A handle to a node in the UI tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct NodeId(pub(crate) usize);

impl NodeId {
    pub(crate) fn index(&self) -> usize {
        self.0
    }
}

/// A node in the UI tree.
pub struct Node {
    pub(crate) widget: Box<dyn Widget>,
    pub(crate) children: Vec<NodeId>,
    pub(crate) parent: Option<NodeId>,
    pub(crate) bounds: Rect,
}

impl Node {
    pub(crate) fn new(widget: Box<dyn Widget>) -> Self {
        Self {
            widget,
            children: Vec::new(),
            parent: None,
            bounds: Rect::default(),
        }
    }

    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }

    pub fn children(&self) -> &[NodeId] {
        &self.children
    }

    pub fn parent(&self) -> Option<NodeId> {
        self.parent
    }

    pub fn widget(&self) -> &dyn Widget {
        &*self.widget
    }

    pub fn widget_mut(&mut self) -> &mut dyn Widget {
        &mut *self.widget
    }
}
