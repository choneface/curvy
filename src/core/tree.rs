use crate::core::{Node, NodeId, Rect, Widget, WidgetState};
use crate::graphics::Canvas;

/// The UI tree that owns all nodes in an arena.
pub struct UiTree {
    nodes: Vec<Option<Node>>,
    free_list: Vec<usize>,
    root: Option<NodeId>,
    hovered: Option<NodeId>,
    pressed: Option<NodeId>,
    focused: Option<NodeId>,
    captured: Option<NodeId>,
}

impl UiTree {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free_list: Vec::new(),
            root: None,
            hovered: None,
            pressed: None,
            focused: None,
            captured: None,
        }
    }

    /// Add a widget to the tree, optionally as a child of another node.
    /// If parent is None and there's no root, this becomes the root.
    pub fn add(&mut self, widget: impl Widget + 'static, parent: Option<NodeId>) -> NodeId {
        self.add_boxed(Box::new(widget), parent)
    }

    /// Add a boxed widget to the tree.
    pub fn add_boxed(&mut self, widget: Box<dyn Widget>, parent: Option<NodeId>) -> NodeId {
        let node = Node::new(widget);
        let id = self.allocate_slot(node);

        if let Some(parent_id) = parent {
            if let Some(parent_node) = self.nodes.get_mut(parent_id.index()).and_then(|n| n.as_mut()) {
                parent_node.children.push(id);
            }
            if let Some(node) = self.nodes.get_mut(id.index()).and_then(|n| n.as_mut()) {
                node.parent = Some(parent_id);
            }
        } else if self.root.is_none() {
            self.root = Some(id);
        }

        id
    }

    /// Remove a node and all its children from the tree.
    pub fn remove(&mut self, id: NodeId) {
        // First collect children to remove
        let children: Vec<NodeId> = self
            .get(id)
            .map(|n| n.children.clone())
            .unwrap_or_default();

        // Recursively remove children
        for child_id in children {
            self.remove(child_id);
        }

        // Remove from parent's children list
        if let Some(node) = self.get(id) {
            if let Some(parent_id) = node.parent {
                if let Some(parent) = self.get_mut(parent_id) {
                    parent.children.retain(|&c| c != id);
                }
            }
        }

        // Clear state references
        if self.root == Some(id) {
            self.root = None;
        }
        if self.hovered == Some(id) {
            self.hovered = None;
        }
        if self.pressed == Some(id) {
            self.pressed = None;
        }
        if self.focused == Some(id) {
            self.focused = None;
        }
        if self.captured == Some(id) {
            self.captured = None;
        }

        // Free the slot
        if let Some(slot) = self.nodes.get_mut(id.index()) {
            *slot = None;
            self.free_list.push(id.index());
        }
    }

    fn allocate_slot(&mut self, node: Node) -> NodeId {
        if let Some(index) = self.free_list.pop() {
            self.nodes[index] = Some(node);
            NodeId(index)
        } else {
            let index = self.nodes.len();
            self.nodes.push(Some(node));
            NodeId(index)
        }
    }

    pub fn get(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id.index()).and_then(|n| n.as_ref())
    }

    pub fn get_mut(&mut self, id: NodeId) -> Option<&mut Node> {
        self.nodes.get_mut(id.index()).and_then(|n| n.as_mut())
    }

    pub fn root(&self) -> Option<NodeId> {
        self.root
    }

    pub fn set_root(&mut self, id: Option<NodeId>) {
        self.root = id;
    }

    /// Set the bounds for a node.
    pub fn set_bounds(&mut self, id: NodeId, bounds: Rect) {
        if let Some(node) = self.get_mut(id) {
            node.bounds = bounds;
        }
    }

    // State accessors

    pub fn hovered(&self) -> Option<NodeId> {
        self.hovered
    }

    pub fn set_hovered(&mut self, id: Option<NodeId>) {
        self.hovered = id;
    }

    pub fn pressed(&self) -> Option<NodeId> {
        self.pressed
    }

    pub fn set_pressed(&mut self, id: Option<NodeId>) {
        self.pressed = id;
    }

    pub fn focused(&self) -> Option<NodeId> {
        self.focused
    }

    pub fn set_focused(&mut self, id: Option<NodeId>) {
        self.focused = id;
    }

    pub fn captured(&self) -> Option<NodeId> {
        self.captured
    }

    pub fn set_captured(&mut self, id: Option<NodeId>) {
        self.captured = id;
    }

    /// Hit test: find the topmost (deepest) node at the given position.
    /// Children are tested before parents (front-to-back).
    pub fn hit_test(&self, x: i32, y: i32) -> Option<NodeId> {
        self.root.and_then(|root| self.hit_test_node(root, x, y))
    }

    fn hit_test_node(&self, id: NodeId, x: i32, y: i32) -> Option<NodeId> {
        let node = self.get(id)?;

        if !node.bounds.contains(x, y) {
            return None;
        }

        // Check children in reverse order (last child is on top)
        for &child_id in node.children.iter().rev() {
            if let Some(hit) = self.hit_test_node(child_id, x, y) {
                return Some(hit);
            }
        }

        // No child hit, this node is the target
        Some(id)
    }

    /// Draw the entire tree to the canvas.
    pub fn draw(&self, canvas: &mut Canvas) {
        if let Some(root) = self.root {
            self.draw_node(root, canvas);
        }
    }

    fn draw_node(&self, id: NodeId, canvas: &mut Canvas) {
        let Some(node) = self.get(id) else {
            return;
        };

        let state = WidgetState {
            hovered: self.hovered == Some(id),
            pressed: self.pressed == Some(id),
            focused: self.focused == Some(id),
        };

        let bounds = node.bounds;
        node.widget.draw(canvas, &bounds, state);

        // Draw children
        let children: Vec<NodeId> = node.children.clone();
        for child_id in children {
            self.draw_node(child_id, canvas);
        }
    }
}

impl Default for UiTree {
    fn default() -> Self {
        Self::new()
    }
}

// Implement View so UiTree can be rendered by the existing Renderer
impl crate::core::View for UiTree {
    fn size(&self) -> (u32, u32) {
        // Size is determined by the root node's bounds
        self.root
            .and_then(|id| self.get(id))
            .map(|node| (node.bounds.width, node.bounds.height))
            .unwrap_or((0, 0))
    }

    fn draw(&self, canvas: &mut Canvas) {
        UiTree::draw(self, canvas);
    }
}
