use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use super::types::{
    HitType, PartDraw, PartHit, PartType, ScrollbarDraw, Skin, SkinError, SkinMeta, SkinPart,
    SkinWindow, TextAlign, TextInputDraw, TextValidation, VerticalAlign,
};

#[derive(Deserialize)]
struct SkinJson {
    skin: SkinMetaJson,
    window: SkinWindowJson,
    assets: HashMap<String, String>,
    #[serde(default)]
    parts: Vec<SkinPartJson>,
}

#[derive(Deserialize)]
struct SkinMetaJson {
    name: String,
    author: String,
    version: String,
}

#[derive(Deserialize)]
struct SkinWindowJson {
    width: u32,
    height: u32,
    #[serde(default)]
    resizable: bool,
}

#[derive(Deserialize)]
struct SkinPartJson {
    id: String,
    #[serde(rename = "type")]
    part_type: String,
    #[serde(default)]
    asset: Option<String>,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    #[serde(default)]
    z: i32,
    #[serde(default)]
    action: Option<String>,
    #[serde(default)]
    draw: Option<PartDrawJson>,
    #[serde(default)]
    text_input_draw: Option<TextInputDrawJson>,
    #[serde(default)]
    scrollbar: Option<ScrollbarDrawJson>,
    #[serde(default)]
    hit: Option<PartHitJson>,
    #[serde(default)]
    text_color: Option<String>,
    #[serde(default)]
    padding: Option<u32>,
    #[serde(default)]
    font_size: Option<f32>,
    #[serde(default)]
    max_length: Option<u32>,
    #[serde(default)]
    validation: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    text_align: Option<String>,
    #[serde(default)]
    vertical_align: Option<String>,
    #[serde(default)]
    binding: Option<String>,
    #[serde(default)]
    content_height: Option<u32>,
    #[serde(default)]
    child: Option<Box<SkinPartJson>>,
}

#[derive(Deserialize)]
struct PartDrawJson {
    normal: String,
    hover: String,
    pressed: String,
}

#[derive(Deserialize)]
struct TextInputDrawJson {
    normal: String,
    hover: String,
    focused: String,
    #[serde(default)]
    invalid: Option<String>,
}

#[derive(Deserialize)]
struct ScrollbarDrawJson {
    width: u32,
    track: String,
    thumb: String,
}

#[derive(Deserialize)]
struct PartHitJson {
    #[serde(rename = "type")]
    hit_type: String,
}

impl Skin {
    /// Load a skin from a JSON file path.
    pub fn load(path: &Path) -> Result<Self, SkinError> {
        let content = std::fs::read_to_string(path)?;
        let json: SkinJson = serde_json::from_str(&content)?;

        let base_path = path.parent().unwrap_or(Path::new("."));

        Ok(Skin {
            meta: SkinMeta {
                name: json.skin.name,
                author: json.skin.author,
                version: json.skin.version,
            },
            window: SkinWindow {
                width: json.window.width,
                height: json.window.height,
                resizable: json.window.resizable,
            },
            assets: json
                .assets
                .into_iter()
                .map(|(k, v)| (k, base_path.join(v)))
                .collect(),
            parts: json
                .parts
                .into_iter()
                .map(|p| Self::convert_part(p))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn convert_part(p: SkinPartJson) -> Result<SkinPart, SkinError> {
        let part_type = match p.part_type.as_str() {
            "image" => {
                let asset = p.asset.ok_or_else(|| {
                    SkinError::AssetNotFound(format!("Image part '{}' missing 'asset' field", p.id))
                })?;
                PartType::Image { asset }
            }
            "button" => PartType::Button,
            "text_input" => PartType::TextInput,
            "static_text" => PartType::StaticText,
            "vscroll_container" => PartType::VScrollContainer,
            other => return Err(SkinError::InvalidPartType(other.to_string())),
        };

        let draw = p.draw.map(|d| PartDraw {
            normal: d.normal,
            hover: d.hover,
            pressed: d.pressed,
        });

        let text_input_draw = p.text_input_draw.map(|d| TextInputDraw {
            normal: d.normal,
            hover: d.hover,
            focused: d.focused,
            invalid: d.invalid,
        });

        let scrollbar = p.scrollbar.map(|s| ScrollbarDraw {
            width: s.width,
            track: s.track,
            thumb: s.thumb,
        });

        let hit = p.hit.map(|h| PartHit {
            hit_type: match h.hit_type.as_str() {
                "rect" | _ => HitType::Rect,
            },
        });

        // Parse text_color from hex string like "0x000000"
        let text_color = p.text_color.and_then(|s| {
            let s = s.trim_start_matches("0x").trim_start_matches("0X");
            u32::from_str_radix(s, 16).ok()
        });

        // Parse validation mode
        let validation = p.validation.map(|s| match s.as_str() {
            "numeric" => TextValidation::Numeric,
            "alpha" => TextValidation::Alpha,
            "alphanumeric" => TextValidation::Alphanumeric,
            "any" => TextValidation::Any,
            pattern => TextValidation::Pattern(pattern.to_string()),
        });

        // Parse text alignment
        let text_align = p.text_align.map(|s| match s.as_str() {
            "left" => TextAlign::Left,
            "center" => TextAlign::Center,
            "right" => TextAlign::Right,
            _ => TextAlign::Left,
        });

        // Parse vertical alignment
        let vertical_align = p.vertical_align.map(|s| match s.as_str() {
            "top" => VerticalAlign::Top,
            "center" => VerticalAlign::Center,
            "bottom" => VerticalAlign::Bottom,
            _ => VerticalAlign::Center,
        });

        // Parse child recursively
        let child = match p.child {
            Some(child_json) => Some(Box::new(Self::convert_part(*child_json)?)),
            None => None,
        };

        Ok(SkinPart {
            id: p.id,
            part_type,
            x: p.x,
            y: p.y,
            width: p.width,
            height: p.height,
            z: p.z,
            draw,
            text_input_draw,
            scrollbar,
            hit,
            action: p.action,
            text_color,
            padding: p.padding,
            font_size: p.font_size,
            max_length: p.max_length,
            validation,
            content: p.content,
            text_align,
            vertical_align,
            binding: p.binding,
            content_height: p.content_height,
            child,
        })
    }
}
