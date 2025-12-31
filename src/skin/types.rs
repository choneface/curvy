use std::collections::HashMap;
use std::path::PathBuf;

/// Skin metadata from [skin] section.
#[derive(Debug, Clone)]
pub struct SkinMeta {
    pub name: String,
    pub author: String,
    pub version: String,
}

/// Window configuration from [window] section.
#[derive(Debug, Clone)]
pub struct SkinWindow {
    pub width: u32,
    pub height: u32,
    pub resizable: bool,
}

/// Drawing configuration for stateful widgets (buttons).
#[derive(Debug, Clone)]
pub struct PartDraw {
    pub normal: String,
    pub hover: String,
    pub pressed: String,
}

/// Drawing configuration for text inputs.
#[derive(Debug, Clone)]
pub struct TextInputDraw {
    pub normal: String,
    pub hover: String,
    pub focused: String,
    pub invalid: Option<String>,
}

/// Scrollbar configuration for scroll containers.
#[derive(Debug, Clone)]
pub struct ScrollbarDraw {
    pub width: u32,
    pub track: String,
    pub thumb: String,
}

/// Directory picker drawing configuration.
#[derive(Debug, Clone)]
pub struct DirectoryPickerDraw {
    pub normal: String,
    pub hover: String,
    pub button_normal: String,
    pub button_hover: String,
}

/// File picker drawing configuration.
#[derive(Debug, Clone)]
pub struct FilePickerDraw {
    /// Picker bar normal state.
    pub picker_normal: String,
    /// Picker bar hover state.
    pub picker_hover: String,
    /// Picker button normal state.
    pub picker_btn_normal: String,
    /// Picker button hover state.
    pub picker_btn_hover: String,
    /// Scrollbar track image.
    pub track: String,
    /// Scrollbar thumb image.
    pub thumb: String,
    /// List item normal state.
    pub item_normal: String,
    /// List item hover state.
    pub item_hover: String,
    /// List item selected state.
    pub item_selected: String,
}

/// Hit testing configuration.
#[derive(Debug, Clone)]
pub struct PartHit {
    pub hit_type: HitType,
}

/// Hit region type.
#[derive(Debug, Clone)]
pub enum HitType {
    Rect,
}

/// Horizontal text alignment.
#[derive(Debug, Clone, Copy, Default)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// Vertical text alignment.
#[derive(Debug, Clone, Copy, Default)]
pub enum VerticalAlign {
    Top,
    #[default]
    Center,
    Bottom,
}

/// Part type discriminator.
#[derive(Debug, Clone)]
pub enum PartType {
    Image { asset: String },
    Button,
    TextInput,
    StaticText,
    VScrollContainer,
    DirectoryPicker,
    FilePicker,
}

/// Validation mode for text input.
#[derive(Debug, Clone)]
pub enum TextValidation {
    /// Any printable ASCII characters (default)
    Any,
    /// Digits only (0-9)
    Numeric,
    /// Letters only (a-z, A-Z)
    Alpha,
    /// Letters and digits
    Alphanumeric,
    /// Custom regex pattern
    Pattern(String),
}

/// A skin part definition from [[parts]] in TOML.
#[derive(Debug, Clone)]
pub struct SkinPart {
    pub id: String,
    pub part_type: PartType,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub z: i32,
    pub draw: Option<PartDraw>,
    pub text_input_draw: Option<TextInputDraw>,
    pub directory_picker_draw: Option<DirectoryPickerDraw>,
    pub file_picker_draw: Option<FilePickerDraw>,
    pub scrollbar: Option<ScrollbarDraw>,
    pub hit: Option<PartHit>,
    pub action: Option<String>,
    pub text_color: Option<u32>,
    pub padding: Option<u32>,
    /// Font size in pixels (uses global font size if not specified)
    pub font_size: Option<f32>,
    /// Maximum number of characters allowed
    pub max_length: Option<u32>,
    /// Character validation mode
    pub validation: Option<TextValidation>,
    /// Static text content
    pub content: Option<String>,
    /// Horizontal text alignment
    pub text_align: Option<TextAlign>,
    /// Vertical text alignment
    pub vertical_align: Option<VerticalAlign>,
    /// Store binding key for reading/writing values
    pub binding: Option<String>,
    /// Content height for scroll containers
    pub content_height: Option<u32>,
    /// Child widget for containers
    pub child: Option<Box<SkinPart>>,
    /// Filter string for file pickers (e.g., ".crix")
    pub filter: Option<String>,
    /// Action to trigger on file selection
    pub on_select: Option<String>,
}

/// The root skin structure parsed from skin.toml.
#[derive(Debug, Clone)]
pub struct Skin {
    pub meta: SkinMeta,
    pub window: SkinWindow,
    pub assets: HashMap<String, PathBuf>,
    pub parts: Vec<SkinPart>,
}

/// Errors that can occur when loading a skin.
#[derive(Debug)]
pub enum SkinError {
    Io(std::io::Error),
    Json(serde_json::Error),
    AssetNotFound(String),
    MissingDrawSection(String),
    InvalidPartType(String),
    Image(image::ImageError),
}

impl std::fmt::Display for SkinError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SkinError::Io(e) => write!(f, "IO error: {}", e),
            SkinError::Json(e) => write!(f, "JSON parse error: {}", e),
            SkinError::AssetNotFound(key) => write!(f, "Asset not found: {}", key),
            SkinError::MissingDrawSection(id) => write!(f, "Missing 'draw' for button: {}", id),
            SkinError::InvalidPartType(t) => write!(f, "Invalid part type: {}", t),
            SkinError::Image(e) => write!(f, "Image error: {}", e),
        }
    }
}

impl std::error::Error for SkinError {}

impl From<std::io::Error> for SkinError {
    fn from(e: std::io::Error) -> Self {
        SkinError::Io(e)
    }
}

impl From<serde_json::Error> for SkinError {
    fn from(e: serde_json::Error) -> Self {
        SkinError::Json(e)
    }
}

impl From<image::ImageError> for SkinError {
    fn from(e: image::ImageError) -> Self {
        SkinError::Image(e)
    }
}
