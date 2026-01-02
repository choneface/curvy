use crate::core::{Rect, UiTree, Widget};
use crate::widgets::Container;

use super::assets::LoadedSkin;
use super::types::{PartType, SkinError, SkinPart, SkinWindow};
use super::widgets::{Checkbox, DirectoryPicker, FilePicker, SkinButton, SkinImage, SkinVScroll, StaticText, TextInput};

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
            PartType::TextInput => {
                let draw = part
                    .text_input_draw
                    .as_ref()
                    .ok_or_else(|| SkinError::MissingDrawSection(part.id.clone()))?;

                let normal = skin
                    .get_image(&draw.normal)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.normal.clone()))?;
                let hover = skin
                    .get_image(&draw.hover)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.hover.clone()))?;
                let focused = skin
                    .get_image(&draw.focused)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.focused.clone()))?;
                let invalid = draw.invalid.as_ref().and_then(|key| skin.get_image(key).cloned());

                let mut text_input = TextInput::new(
                    normal.clone(),
                    hover.clone(),
                    focused.clone(),
                    invalid,
                );

                if let Some(action) = &part.action {
                    text_input = text_input.with_on_change(action.clone());
                }
                if let Some(color) = part.text_color {
                    text_input = text_input.with_text_color(color);
                }
                if let Some(padding) = part.padding {
                    text_input = text_input.with_padding(padding);
                }
                if let Some(size) = part.font_size {
                    text_input = text_input.with_font_size(size);
                }
                if let Some(max) = part.max_length {
                    text_input = text_input.with_max_length(max);
                }
                if let Some(validation) = &part.validation {
                    text_input = text_input.with_validation(validation.clone());
                }
                if let Some(binding) = &part.binding {
                    text_input = text_input.with_binding(binding.clone());
                }

                Ok(Box::new(text_input))
            }
            PartType::StaticText => {
                let content = part.content.clone().unwrap_or_default();
                let mut static_text = StaticText::new(content);

                if let Some(size) = part.font_size {
                    static_text = static_text.with_font_size(size);
                }
                if let Some(color) = part.text_color {
                    static_text = static_text.with_text_color(color);
                }
                if let Some(align) = part.text_align {
                    static_text = static_text.with_text_align(align);
                }
                if let Some(valign) = part.vertical_align {
                    static_text = static_text.with_vertical_align(valign);
                }
                if let Some(padding) = part.padding {
                    static_text = static_text.with_padding(padding);
                }
                if let Some(binding) = &part.binding {
                    static_text = static_text.with_binding(binding.clone());
                }

                Ok(Box::new(static_text))
            }
            PartType::VScrollContainer => {
                let scrollbar = part
                    .scrollbar
                    .as_ref()
                    .ok_or_else(|| SkinError::MissingDrawSection(format!("{} (scrollbar)", part.id)))?;

                let track = skin
                    .get_image(&scrollbar.track)
                    .ok_or_else(|| SkinError::AssetNotFound(scrollbar.track.clone()))?;
                let thumb = skin
                    .get_image(&scrollbar.thumb)
                    .ok_or_else(|| SkinError::AssetNotFound(scrollbar.thumb.clone()))?;

                let mut scroll = SkinVScroll::new(
                    part.width,
                    part.height,
                    track.clone(),
                    thumb.clone(),
                );

                // Set content height if specified
                if let Some(content_height) = part.content_height {
                    scroll = scroll.with_content_height(content_height);
                }

                // Build child widget if present
                if let Some(ref child_part) = part.child {
                    let child_widget = Self::create_widget(child_part, skin)?;
                    scroll = scroll.with_child(child_widget);
                }

                Ok(Box::new(scroll))
            }
            PartType::DirectoryPicker => {
                let draw = part
                    .directory_picker_draw
                    .as_ref()
                    .ok_or_else(|| SkinError::MissingDrawSection(part.id.clone()))?;

                let normal = skin
                    .get_image(&draw.normal)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.normal.clone()))?;
                let hover = skin
                    .get_image(&draw.hover)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.hover.clone()))?;
                let button_normal = skin
                    .get_image(&draw.button_normal)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.button_normal.clone()))?;
                let button_hover = skin
                    .get_image(&draw.button_hover)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.button_hover.clone()))?;

                let mut picker = DirectoryPicker::new(
                    normal.clone(),
                    hover.clone(),
                    button_normal.clone(),
                    button_hover.clone(),
                );

                if let Some(color) = part.text_color {
                    picker = picker.with_text_color(color);
                }
                if let Some(padding) = part.padding {
                    picker = picker.with_padding(padding);
                }
                if let Some(size) = part.font_size {
                    picker = picker.with_font_size(size);
                }
                if let Some(binding) = &part.binding {
                    picker = picker.with_binding(binding.clone());
                }

                Ok(Box::new(picker))
            }
            PartType::FilePicker => {
                let draw = part
                    .file_picker_draw
                    .as_ref()
                    .ok_or_else(|| SkinError::MissingDrawSection(part.id.clone()))?;

                let picker_normal = skin
                    .get_image(&draw.picker_normal)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.picker_normal.clone()))?;
                let picker_hover = skin
                    .get_image(&draw.picker_hover)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.picker_hover.clone()))?;
                let picker_btn_normal = skin
                    .get_image(&draw.picker_btn_normal)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.picker_btn_normal.clone()))?;
                let picker_btn_hover = skin
                    .get_image(&draw.picker_btn_hover)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.picker_btn_hover.clone()))?;
                let track = skin
                    .get_image(&draw.track)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.track.clone()))?;
                let thumb = skin
                    .get_image(&draw.thumb)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.thumb.clone()))?;
                let item_normal = skin
                    .get_image(&draw.item_normal)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.item_normal.clone()))?;
                let item_hover = skin
                    .get_image(&draw.item_hover)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.item_hover.clone()))?;
                let item_selected = skin
                    .get_image(&draw.item_selected)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.item_selected.clone()))?;

                let mut picker = FilePicker::new(
                    part.width,
                    part.height,
                    picker_normal.clone(),
                    picker_hover.clone(),
                    picker_btn_normal.clone(),
                    picker_btn_hover.clone(),
                    track.clone(),
                    thumb.clone(),
                    item_normal.clone(),
                    item_hover.clone(),
                    item_selected.clone(),
                );

                if let Some(ref filter) = part.filter {
                    picker = picker.with_filter(filter.clone());
                }
                if let Some(color) = part.text_color {
                    picker = picker.with_text_color(color);
                }
                if let Some(padding) = part.padding {
                    picker = picker.with_padding(padding);
                }
                if let Some(binding) = &part.binding {
                    picker = picker.with_binding(binding.clone());
                }
                if let Some(on_select) = &part.on_select {
                    picker = picker.with_on_select(on_select.clone());
                }

                Ok(Box::new(picker))
            }
            PartType::Checkbox => {
                let draw = part
                    .checkbox_draw
                    .as_ref()
                    .ok_or_else(|| SkinError::MissingDrawSection(part.id.clone()))?;

                let unchecked = skin
                    .get_image(&draw.unchecked)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.unchecked.clone()))?;
                let checked = skin
                    .get_image(&draw.checked)
                    .ok_or_else(|| SkinError::AssetNotFound(draw.checked.clone()))?;

                let mut checkbox = Checkbox::new(unchecked.clone(), checked.clone());

                if let Some(ref label) = part.label {
                    checkbox = checkbox.with_label(label.clone());
                }
                if let Some(color) = part.text_color {
                    checkbox = checkbox.with_text_color(color);
                }
                if let Some(size) = part.font_size {
                    checkbox = checkbox.with_font_size(size);
                }
                if let Some(padding) = part.padding {
                    checkbox = checkbox.with_padding(padding);
                }
                if let Some(binding) = &part.binding {
                    checkbox = checkbox.with_binding(binding.clone());
                }
                if let Some(action) = &part.action {
                    checkbox = checkbox.with_action(action.clone());
                }

                Ok(Box::new(checkbox))
            }
        }
    }
}
