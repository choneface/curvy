# Crix Skin Widgets Reference

This document describes all available widgets and their JSON schemas for use in `skin.json` files.

## Skin File Structure

```json
{
  "skin": {
    "name": "App Name",
    "author": "Author Name",
    "version": "1.0"
  },
  "window": {
    "width": 800,
    "height": 600,
    "resizable": false
  },
  "assets": {
    "asset_key": "path/to/image.png"
  },
  "parts": [
    { /* widget definitions */ }
  ]
}
```

## Common Fields

All widgets share these common fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `id` | string | Yes | Unique identifier for the widget |
| `type` | string | Yes | Widget type (see below) |
| `x` | integer | Yes | X position in pixels |
| `y` | integer | Yes | Y position in pixels |
| `width` | integer | Yes | Width in pixels |
| `height` | integer | Yes | Height in pixels |
| `z` | integer | No | Z-order for layering (default: 0, higher = on top) |

---

## Widget Types

### 1. Image (`image`)

Displays a static image.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `asset` | string | Yes | Key referencing an asset in the `assets` section |

#### Example

```json
{
  "id": "background",
  "type": "image",
  "asset": "background_image",
  "x": 0,
  "y": 0,
  "width": 800,
  "height": 600,
  "z": 0
}
```

---

### 2. Button (`button`)

A clickable button with normal, hover, and pressed states.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `action` | string | No | Action name to trigger when clicked |
| `draw` | object | Yes | Drawing configuration (see below) |
| `hit` | object | No | Hit testing configuration |

**`draw` object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `normal` | string | Yes | Asset key for normal state |
| `hover` | string | Yes | Asset key for hover state |
| `pressed` | string | Yes | Asset key for pressed state |

**`hit` object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `type` | string | Yes | Hit region type (`"rect"`) |

#### Example

```json
{
  "id": "submit_button",
  "type": "button",
  "x": 100,
  "y": 200,
  "width": 120,
  "height": 40,
  "z": 10,
  "action": "submit_form",
  "draw": {
    "normal": "btn_normal",
    "hover": "btn_hover",
    "pressed": "btn_pressed"
  },
  "hit": {
    "type": "rect"
  }
}
```

---

### 3. Text Input (`text_input`)

An editable text input field with validation support.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `text_input_draw` | object | Yes | Drawing configuration (see below) |
| `text_color` | string | No | Text color as hex (e.g., `"0x000000"`) |
| `padding` | integer | No | Internal padding in pixels |
| `font_size` | float | No | Font size in pixels |
| `max_length` | integer | No | Maximum character count |
| `validation` | string | No | Validation mode (see below) |
| `binding` | string | No | Store key for two-way binding |
| `action` | string | No | Action triggered on text change |
| `hit` | object | No | Hit testing configuration |

**`text_input_draw` object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `normal` | string | Yes | Asset key for normal state |
| `hover` | string | Yes | Asset key for hover state |
| `focused` | string | Yes | Asset key for focused state |
| `invalid` | string | No | Asset key for invalid state |

**`validation` values:**

| Value | Description |
|-------|-------------|
| `"any"` | Any printable ASCII characters (default) |
| `"numeric"` | Digits only (0-9) |
| `"alpha"` | Letters only (a-z, A-Z) |
| `"alphanumeric"` | Letters and digits |
| `"<chars>"` | Custom allowed character set (e.g., `"0123456789."`) |

#### Example

```json
{
  "id": "email_input",
  "type": "text_input",
  "x": 50,
  "y": 100,
  "width": 300,
  "height": 40,
  "z": 10,
  "font_size": 18.0,
  "max_length": 50,
  "text_color": "0x333333",
  "padding": 8,
  "binding": "user.email",
  "text_input_draw": {
    "normal": "input_normal",
    "hover": "input_hover",
    "focused": "input_focused",
    "invalid": "input_invalid"
  },
  "hit": {
    "type": "rect"
  }
}
```

---

### 4. Static Text (`static_text`)

Displays static or dynamically-bound text.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `content` | string | No | Initial text content |
| `text_color` | string | No | Text color as hex (e.g., `"0xFFFFFF"`) |
| `font_size` | float | No | Font size in pixels |
| `padding` | integer | No | Internal padding in pixels |
| `text_align` | string | No | Horizontal alignment: `"left"`, `"center"`, `"right"` |
| `vertical_align` | string | No | Vertical alignment: `"top"`, `"center"`, `"bottom"` |
| `binding` | string | No | Store key to read display value from |

#### Example

```json
{
  "id": "title_label",
  "type": "static_text",
  "x": 0,
  "y": 20,
  "width": 800,
  "height": 60,
  "z": 10,
  "content": "Welcome",
  "font_size": 48.0,
  "text_color": "0xFFFFFF",
  "text_align": "center",
  "vertical_align": "center"
}
```

**Dynamic binding example:**

```json
{
  "id": "result_display",
  "type": "static_text",
  "x": 50,
  "y": 300,
  "width": 200,
  "height": 40,
  "z": 10,
  "content": "---",
  "font_size": 24.0,
  "text_color": "0x00FF00",
  "binding": "outputs.result"
}
```

---

### 5. Checkbox (`checkbox`)

A toggleable checkbox with optional label text.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `checkbox_draw` | object | Yes | Drawing configuration (see below) |
| `label` | string | No | Label text displayed next to checkbox |
| `text_color` | string | No | Label text color as hex |
| `font_size` | float | No | Label font size in pixels |
| `padding` | integer | No | Space between checkbox and label |
| `binding` | string | No | Store key for boolean state |
| `action` | string | No | Action triggered when toggled |

**`checkbox_draw` object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `unchecked` | string | Yes | Asset key for unchecked state |
| `checked` | string | Yes | Asset key for checked state |

#### Example

```json
{
  "id": "dark_mode_toggle",
  "type": "checkbox",
  "x": 600,
  "y": 20,
  "width": 150,
  "height": 24,
  "z": 10,
  "label": "Dark Mode",
  "text_color": "0xFFFFFF",
  "font_size": 16.0,
  "padding": 8,
  "binding": "settings.dark_mode",
  "action": "apply_theme",
  "checkbox_draw": {
    "unchecked": "checkbox_off",
    "checked": "checkbox_on"
  }
}
```

---

### 6. Vertical Scroll Container (`vscroll_container`)

A scrollable container for content taller than the viewport.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `scrollbar` | object | Yes | Scrollbar configuration (see below) |
| `content_height` | integer | No | Total scrollable content height |
| `child` | object | No | Nested child widget definition |

**`scrollbar` object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `width` | integer | Yes | Scrollbar width in pixels |
| `track` | string | Yes | Asset key for scrollbar track |
| `thumb` | string | Yes | Asset key for scrollbar thumb |

#### Example

```json
{
  "id": "content_scroll",
  "type": "vscroll_container",
  "x": 20,
  "y": 100,
  "width": 400,
  "height": 300,
  "z": 10,
  "content_height": 800,
  "scrollbar": {
    "width": 16,
    "track": "scroll_track",
    "thumb": "scroll_thumb"
  },
  "child": {
    "id": "scroll_content",
    "type": "static_text",
    "x": 0,
    "y": 0,
    "width": 380,
    "height": 800,
    "content": "Long scrollable content..."
  }
}
```

---

### 7. Directory Picker (`directory_picker`)

A widget for selecting directories with a browse button.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `directory_picker_draw` | object | Yes | Drawing configuration (see below) |
| `text_color` | string | No | Path text color as hex |
| `font_size` | float | No | Font size in pixels |
| `padding` | integer | No | Internal padding in pixels |
| `binding` | string | No | Store key for selected path |

**`directory_picker_draw` object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `normal` | string | Yes | Asset key for text area normal state |
| `hover` | string | Yes | Asset key for text area hover state |
| `button_normal` | string | Yes | Asset key for browse button normal state |
| `button_hover` | string | Yes | Asset key for browse button hover state |

#### Example

```json
{
  "id": "output_folder",
  "type": "directory_picker",
  "x": 50,
  "y": 200,
  "width": 400,
  "height": 40,
  "z": 10,
  "text_color": "0x000000",
  "font_size": 14.0,
  "padding": 8,
  "binding": "settings.output_path",
  "directory_picker_draw": {
    "normal": "picker_normal",
    "hover": "picker_hover",
    "button_normal": "browse_btn_normal",
    "button_hover": "browse_btn_hover"
  }
}
```

---

### 8. File Picker (`file_picker`)

A full file browser with directory bar, scrollable file list, and selection support.

#### Schema

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `file_picker_draw` | object | Yes | Drawing configuration (see below) |
| `filter` | string | No | File extension filter (e.g., `".crix"`, `".txt"`) |
| `text_color` | string | No | Text color as hex |
| `padding` | integer | No | Internal padding in pixels |
| `binding` | string | No | Store key for selected file path |
| `on_select` | string | No | Action triggered when a file is selected |

**`file_picker_draw` object:**

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `picker_normal` | string | Yes | Asset key for path bar normal state |
| `picker_hover` | string | Yes | Asset key for path bar hover state |
| `picker_btn_normal` | string | Yes | Asset key for browse button normal state |
| `picker_btn_hover` | string | Yes | Asset key for browse button hover state |
| `track` | string | Yes | Asset key for scrollbar track |
| `thumb` | string | Yes | Asset key for scrollbar thumb |
| `item_normal` | string | Yes | Asset key for list item normal state |
| `item_hover` | string | Yes | Asset key for list item hover state |
| `item_selected` | string | Yes | Asset key for list item selected state |

#### Example

```json
{
  "id": "app_browser",
  "type": "file_picker",
  "x": 20,
  "y": 50,
  "width": 500,
  "height": 400,
  "z": 10,
  "filter": ".crix",
  "text_color": "0xDDDDDD",
  "padding": 8,
  "binding": "selected_app",
  "on_select": "load_app_info",
  "file_picker_draw": {
    "picker_normal": "picker_bg",
    "picker_hover": "picker_bg_hover",
    "picker_btn_normal": "browse_normal",
    "picker_btn_hover": "browse_hover",
    "track": "scroll_track",
    "thumb": "scroll_thumb",
    "item_normal": "item_bg",
    "item_hover": "item_bg_hover",
    "item_selected": "item_bg_selected"
  }
}
```

---

## Store Bindings

Widgets can be bound to the store for reactive data flow:

- **Text Input**: Two-way binding - user input syncs to store, store changes update display
- **Static Text**: One-way binding - reads from store to update display
- **Checkbox**: Two-way binding - toggle state syncs as boolean to store
- **Directory/File Picker**: One-way binding - selected path syncs to store

### Accessing Bindings in Lua

```lua
-- Read a value
local value = app.get("inputs.username")

-- Write a value
app.set("outputs.result", "42")

-- Read checkbox state (boolean)
local is_checked = app.get("settings.dark_mode")
```

---

## Actions

Actions connect UI events to Lua scripts defined in `app.toml`:

```toml
[actions]
calculate = "scripts/calculate.lua"
update_labels = "scripts/update_labels.lua"
```

Widgets that support actions:
- **Button**: `action` - triggered on click
- **Checkbox**: `action` - triggered on toggle
- **Text Input**: `action` - triggered on text change
- **File Picker**: `on_select` - triggered when a file is selected

### Built-in Actions

Some actions are handled internally by the runtime:

| Action | Description |
|--------|-------------|
| `launch_child_app` | Launches a .crix bundle in a new process |
| `load_app_info` | Loads app.toml metadata into store |
| `launch_selected_app` | Launches the app at `selected_app_path` store key |

---

## Color Format

Colors are specified as hex strings with `0x` prefix:

```json
"text_color": "0xFFFFFF"   // White
"text_color": "0x000000"   // Black
"text_color": "0xFF0000"   // Red
"text_color": "0x00FF00"   // Green
"text_color": "0x0000FF"   // Blue
"text_color": "0xDDDDDD"   // Light gray
```
