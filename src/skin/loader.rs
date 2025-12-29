use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use super::types::{
    HitType, PartDraw, PartHit, PartType, Skin, SkinError, SkinMeta, SkinPart, SkinWindow,
};

#[derive(Deserialize)]
struct SkinToml {
    skin: SkinMetaToml,
    window: SkinWindowToml,
    assets: HashMap<String, String>,
    #[serde(default)]
    parts: Vec<SkinPartToml>,
}

#[derive(Deserialize)]
struct SkinMetaToml {
    name: String,
    author: String,
    version: String,
}

#[derive(Deserialize)]
struct SkinWindowToml {
    width: u32,
    height: u32,
    #[serde(default)]
    resizable: bool,
}

#[derive(Deserialize)]
struct SkinPartToml {
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
    draw: Option<PartDrawToml>,
    #[serde(default)]
    hit: Option<PartHitToml>,
}

#[derive(Deserialize)]
struct PartDrawToml {
    normal: String,
    hover: String,
    pressed: String,
}

#[derive(Deserialize)]
struct PartHitToml {
    #[serde(rename = "type")]
    hit_type: String,
}

impl Skin {
    /// Load a skin from a TOML file path.
    pub fn load(path: &Path) -> Result<Self, SkinError> {
        let content = std::fs::read_to_string(path)?;
        let toml: SkinToml = toml::from_str(&content)?;

        let base_path = path.parent().unwrap_or(Path::new("."));

        Ok(Skin {
            meta: SkinMeta {
                name: toml.skin.name,
                author: toml.skin.author,
                version: toml.skin.version,
            },
            window: SkinWindow {
                width: toml.window.width,
                height: toml.window.height,
                resizable: toml.window.resizable,
            },
            assets: toml
                .assets
                .into_iter()
                .map(|(k, v)| (k, base_path.join(v)))
                .collect(),
            parts: toml
                .parts
                .into_iter()
                .map(|p| Self::convert_part(p))
                .collect::<Result<Vec<_>, _>>()?,
        })
    }

    fn convert_part(p: SkinPartToml) -> Result<SkinPart, SkinError> {
        let part_type = match p.part_type.as_str() {
            "image" => {
                let asset = p.asset.ok_or_else(|| {
                    SkinError::AssetNotFound(format!("Image part '{}' missing 'asset' field", p.id))
                })?;
                PartType::Image { asset }
            }
            "button" => PartType::Button,
            other => return Err(SkinError::InvalidPartType(other.to_string())),
        };

        let draw = p.draw.map(|d| PartDraw {
            normal: d.normal,
            hover: d.hover,
            pressed: d.pressed,
        });

        let hit = p.hit.map(|h| PartHit {
            hit_type: match h.hit_type.as_str() {
                "rect" | _ => HitType::Rect,
            },
        });

        Ok(SkinPart {
            id: p.id,
            part_type,
            x: p.x,
            y: p.y,
            width: p.width,
            height: p.height,
            z: p.z,
            draw,
            hit,
            action: p.action,
        })
    }
}
