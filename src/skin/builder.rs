use crate::core::{Rect, UiTree, Widget};
use crate::widgets::Container;

use super::assets::LoadedSkin;
use super::types::{PartType, SkinError, SkinPart, SkinWindow};
use super::widgets::{SkinButton, SkinImage};

/// Builds a UiTree from a loaded skin.
pub struct SkinBuilder;

impl SkinBuilder {
    /// Build a UiTree from a loaded skin.
    /// Returns the tree and window configuration.
    pub fn build(skin: &LoadedSkin) -> Result<(UiTree, SkinWindow), SkinError> {
        let mut tree = UiTree::new();
        let window = &skin.skin.window;

        // Create a transparent root container with window dimensions
        let root_container = Container::transparent(window.width, window.height);
        let root_id = tree.add(root_container, None);
        tree.set_bounds(root_id, Rect::new(0, 0, window.width, window.height));

        // Sort parts by z-order
        let mut parts: Vec<_> = skin.skin.parts.iter().collect();
        parts.sort_by_key(|p| p.z);

        // Create widgets and add to tree as children of root
        for part in parts {
            let widget = Self::create_widget(part, skin)?;
            let bounds = Rect::new(part.x, part.y, part.width, part.height);

            let node_id = tree.add_boxed(widget, Some(root_id));
            tree.set_bounds(node_id, bounds);
        }

        Ok((tree, skin.skin.window.clone()))
    }

    fn create_widget(part: &SkinPart, skin: &LoadedSkin) -> Result<Box<dyn Widget>, SkinError> {
        match &part.part_type {
            PartType::Image { asset } => {
                let image = skin
                    .get_image(asset)
                    .ok_or_else(|| SkinError::AssetNotFound(asset.clone()))?;
                Ok(Box::new(SkinImage::new(image.clone())))
            }
            PartType::Button => {
                let draw = part
                    .draw
                    .as_ref()
                    .ok_or_else(|| SkinError::MissingDrawSection(part.id.clone()))?;

                let normal = skin
                    .get_image(&draw.normal)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.normal.clone()))?;
                let hover = skin
                    .get_image(&draw.hover)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.hover.clone()))?;
                let pressed = skin
                    .get_image(&draw.pressed)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.pressed.clone()))?;

                Ok(Box::new(SkinButton::new(
                    normal.clone(),
                    hover.clone(),
                    pressed.clone(),
                    part.action.clone(),
                )))
            }
        }
    }
}
