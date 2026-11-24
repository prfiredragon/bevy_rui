[![Rust](https://github.com/prfiredragon/bevy_rui/actions/workflows/rust.yml/badge.svg)](https://github.com/prfiredragon/bevy_rui/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/bevy_rui.svg)](https://crates.io/crates/bevy_rui)
[![Docs](https://docs.rs/bevy/badge.svg)](https://docs.rs/bevy/latest/bevy_rui/)

# bevy_rui

Retained UI (RUI) widget library and layout system for the **Bevy Engine**, designed for both **desktop applications / editor tools** and **9-slice textured game UIs**.

These are the included demos. For the game demos you need the [Kenney UI Assets Pack](https://kenney-assets.itch.io/ui-pack).

* Theming and textures (Ninepatch / 9-slice) are fully supported for all game-oriented widgets.
* Built-in keyboard and gamepad directional navigation (`D-Pad` + `Left Stick`).
* If you want to support the project: [ko-fi goal](https://ko-fi.com/robertxy/goal?g=1) or [ko-fi member](http://ko-fi.com/robertxy/tiers)

## Bevy Compatibility

| `bevy_rui` | `bevy` |
| :--- | :--- |
| **`0.1.8`** | **`0.20.1`** |
| `0.1.7` | `0.20.0` |
| `0.1.6` | `0.19.0` |

## Widgets

* **Accordion Menu** (`example_accordion` / `example_game_accordion`)
* **Button** (`example_button` / `example_game_button`)
* **Canvas Layer** (for multi-camera Game HUDs — `example_canvas_layer` / `example_game_canvas_layer`)
* **Checkbox** (`example_checkbox` / `example_game_checkbox`)
* **Clipboard** (`example_clipboard` / `example_game_clipboard`)
* **Color Picker** (`example_color_picker` / `example_game_color_picker`)
* **Context Menu** (`example_context_menu` / `example_game_context_menu`)
* **Debug Colors** (`example_debug_colors` / `example_game_debug_colors`)
* **Dock (Splittable Panels)** (`example_docks` / `example_game_docks`)
* **Dropdown** (`example_dropdown` / `example_game_dropdown`)
* **Label** (`example_label` / `example_game_label`)
* **Layout (`vbox` / `hbox`)** (`example_layout` / `example_game_layout`)
* **Menu & Submenu** (`example_menu` / `example_game_menu`)
* **Open/Save File Dialog** (`example_file_dialog` / `example_game_file_dialog`)
* **Progress Bar** (`example_progress_bar` / `example_game_progress_bar`)
* **Resizer (Splitter Bar)** (`example_resizer` / `example_game_resizer`)
* **ScrollView** (`example_scrollview` / `example_game_scrollview`)
* **Slider** (`example_slider` / `example_game_slider`)
* **Tabs** (`example_tabs` / `example_game_tabs`)
* **Textbox** (`example_textbox` / `example_game_textbox`)
* **Tooltip** (`example_tooltip` / `example_game_tooltip`)
* **Viewport (Camera-in-UI)** (`example_viewport` / `example_game_viewport`)
* **Windows (Draggable & Z-Ordered)** (`example_windows` / `example_game_windows`)

---

## Widget Gallery (Game 9-Slice Theme)

Each widget includes both a standard application example (`cargo run --example example_<widget>`) and a 9-slice textured game UI example (`cargo run --example example_game_<widget>`).

### Accordion
* **App Example:** [`examples/example_accordion.rs`](examples/example_accordion.rs) — `cargo run --example example_accordion`
* **Game Example:** [`examples/example_game_accordion.rs`](examples/example_game_accordion.rs) — `cargo run --example example_game_accordion`

![Accordion Game Example](screenshots/example_game_accordion.png)

### Button
* **App Example:** [`examples/example_button.rs`](examples/example_button.rs) — `cargo run --example example_button`
* **Game Example:** [`examples/example_game_button.rs`](examples/example_game_button.rs) — `cargo run --example example_game_button`

![Button Game Example](screenshots/example_game_button.png)

### Canvas Layer (Game HUD Overlay)
* **App Example:** [`examples/example_canvas_layer.rs`](examples/example_canvas_layer.rs) — `cargo run --example example_canvas_layer`
* **Game Example:** [`examples/example_game_canvas_layer.rs`](examples/example_game_canvas_layer.rs) — `cargo run --example example_game_canvas_layer`

![Canvas Layer Game Example](screenshots/example_game_canvas_layer.png)

### Checkbox
* **App Example:** [`examples/example_checkbox.rs`](examples/example_checkbox.rs) — `cargo run --example example_checkbox`
* **Game Example:** [`examples/example_game_checkbox.rs`](examples/example_game_checkbox.rs) — `cargo run --example example_game_checkbox`

![Checkbox Game Example](screenshots/example_game_checkbox.png)

### Clipboard
* **App Example:** [`examples/example_clipboard.rs`](examples/example_clipboard.rs) — `cargo run --example example_clipboard`
* **Game Example:** [`examples/example_game_clipboard.rs`](examples/example_game_clipboard.rs) — `cargo run --example example_game_clipboard`

![Clipboard Game Example](screenshots/example_game_clipboard.png)

### Color Picker
* **App Example:** [`examples/example_color_picker.rs`](examples/example_color_picker.rs) — `cargo run --example example_color_picker`
* **Game Example:** [`examples/example_game_color_picker.rs`](examples/example_game_color_picker.rs) — `cargo run --example example_game_color_picker`

![Color Picker Game Example](screenshots/example_game_color_picker.png)

### Context Menu
* **App Example:** [`examples/example_context_menu.rs`](examples/example_context_menu.rs) — `cargo run --example example_context_menu`
* **Game Example:** [`examples/example_game_context_menu.rs`](examples/example_game_context_menu.rs) — `cargo run --example example_game_context_menu`

![Context Menu Game Example](screenshots/example_game_context_menu.png)

### Docks (Splittable Panels)
* **App Example:** [`examples/example_docks.rs`](examples/example_docks.rs) — `cargo run --example example_docks`
* **Game Example:** [`examples/example_game_docks.rs`](examples/example_game_docks.rs) — `cargo run --example example_game_docks`

![Docks Game Example](screenshots/example_game_docks.png)

### Dropdown
* **App Example:** [`examples/example_dropdown.rs`](examples/example_dropdown.rs) — `cargo run --example example_dropdown`
* **Game Example:** [`examples/example_game_dropdown.rs`](examples/example_game_dropdown.rs) — `cargo run --example example_game_dropdown`

![Dropdown Game Example](screenshots/example_game_dropdown.png)

### File Dialog (Open / Save)
* **App Example:** [`examples/example_file_dialog.rs`](examples/example_file_dialog.rs) — `cargo run --example example_file_dialog`
* **Game Example:** [`examples/example_game_file_dialog.rs`](examples/example_game_file_dialog.rs) — `cargo run --example example_game_file_dialog`

![File Dialog Game Example](screenshots/example_game_file_dialog.png)

### Label
* **App Example:** [`examples/example_label.rs`](examples/example_label.rs) — `cargo run --example example_label`
* **Game Example:** [`examples/example_game_label.rs`](examples/example_game_label.rs) — `cargo run --example example_game_label`

![Label Game Example](screenshots/example_game_label.png)

### Layout (`vbox` & `hbox`)
* **App Example:** [`examples/example_layout.rs`](examples/example_layout.rs) — `cargo run --example example_layout`
* **Game Example:** [`examples/example_game_layout.rs`](examples/example_game_layout.rs) — `cargo run --example example_game_layout`

![Layout Game Example](screenshots/example_game_layout.png)

### Menu & Submenu
* **App Example:** [`examples/example_menu.rs`](examples/example_menu.rs) — `cargo run --example example_menu`
* **Game Example:** [`examples/example_game_menu.rs`](examples/example_game_menu.rs) — `cargo run --example example_game_menu`

![Menu Game Example](screenshots/example_game_menu.png)

### Progress Bar
* **App Example:** [`examples/example_progress_bar.rs`](examples/example_progress_bar.rs) — `cargo run --example example_progress_bar`
* **Game Example:** [`examples/example_game_progress_bar.rs`](examples/example_game_progress_bar.rs) — `cargo run --example example_game_progress_bar`

![Progress Bar Game Example](screenshots/example_game_progress_bar.png)

### Resizer (Splitter Bar)
* **App Example:** [`examples/example_resizer.rs`](examples/example_resizer.rs) — `cargo run --example example_resizer`
* **Game Example:** [`examples/example_game_resizer.rs`](examples/example_game_resizer.rs) — `cargo run --example example_game_resizer`

![Resizer Game Example](screenshots/example_game_resizer.png)

### ScrollView
* **App Example:** [`examples/example_scrollview.rs`](examples/example_scrollview.rs) — `cargo run --example example_scrollview`
* **Game Example:** [`examples/example_game_scrollview.rs`](examples/example_game_scrollview.rs) — `cargo run --example example_game_scrollview`

![ScrollView Game Example](screenshots/example_game_scrollview.png)

### Slider
* **App Example:** [`examples/example_slider.rs`](examples/example_slider.rs) — `cargo run --example example_slider`
* **Game Example:** [`examples/example_game_slider.rs`](examples/example_game_slider.rs) — `cargo run --example example_game_slider`

![Slider Game Example](screenshots/example_game_slider.png)

### Tabs
* **App Example:** [`examples/example_tabs.rs`](examples/example_tabs.rs) — `cargo run --example example_tabs`
* **Game Example:** [`examples/example_game_tabs.rs`](examples/example_game_tabs.rs) — `cargo run --example example_game_tabs`

![Tabs Game Example](screenshots/example_game_tabs.png)

### Textbox
* **App Example:** [`examples/example_textbox.rs`](examples/example_textbox.rs) — `cargo run --example example_textbox`
* **Game Example:** [`examples/example_game_textbox.rs`](examples/example_game_textbox.rs) — `cargo run --example example_game_textbox`

![Textbox Game Example](screenshots/example_game_textbox.png)

### Tooltip
* **App Example:** [`examples/example_tooltip.rs`](examples/example_tooltip.rs) — `cargo run --example example_tooltip`
* **Game Example:** [`examples/example_game_tooltip.rs`](examples/example_game_tooltip.rs) — `cargo run --example example_game_tooltip`

![Tooltip Game Example](screenshots/example_game_tooltip.png)

### Viewport (Camera-in-UI)
* **App Example:** [`examples/example_viewport.rs`](examples/example_viewport.rs) — `cargo run --example example_viewport`
* **Game Example:** [`examples/example_game_viewport.rs`](examples/example_game_viewport.rs) — `cargo run --example example_game_viewport`

![Viewport Game Example](screenshots/example_game_viewport.png)

### Windows
* **App Example:** [`examples/example_windows.rs`](examples/example_windows.rs) — `cargo run --example example_windows`
* **Game Example:** [`examples/example_game_windows.rs`](examples/example_game_windows.rs) — `cargo run --example example_game_windows`

![Windows Game Example](screenshots/example_game_windows.png)

---

## Full Showcase Demos

[dock demo](https://github.com/prfiredragon/bevy_rui/blob/main/examples/layout_dock.rs)

![](assets/20260619_221219_demo_editor.png)


[Layout dock save](https://github.com/prfiredragon/bevy_rui/blob/main/examples/layout_dock_save.rs)

![](assets/20260621_093832_20240718_09h22m19s_grim.png)


[layout demo](https://github.com/prfiredragon/bevy_rui/blob/main/examples/layout_test.rs)

![](assets/20260619_221331_demo_layout.png)


[game menu demo](https://github.com/prfiredragon/bevy_rui/blob/main/examples/game_test.rs)

![](assets/20260619_221343_menu_game.png)
