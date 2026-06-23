use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;
use crate::widgets::textbox::{spawn_textbox, RuiTextBox};
use crate::widgets::slider::{spawn_slider, RuiSlider};

use bevy::asset::RenderAssetUsages;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::image::ImageSampler;

#[derive(Resource)]
pub struct ColorPickerImages {
    pub hue_slider: Handle<Image>,
    pub white_gradient: Handle<Image>,
    pub black_gradient: Handle<Image>,
}

fn srgb_to_linear(val: f32) -> f32 {
    if val <= 0.04045 {
        val / 12.92
    } else {
        ((val + 0.055) / 1.055).powf(2.4)
    }
}

pub fn setup_color_picker_images(mut images: ResMut<Assets<Image>>, mut commands: Commands) {
    // 1. Hue Slider (1x256)
    let mut hue_pixels = Vec::with_capacity(256 * 4);
    for y in 0..256 {
        let hue = (y as f32 / 255.0) * 360.0;
        let c = Color::from(Hsva { hue, saturation: 1.0, value: 1.0, alpha: 1.0 }).to_srgba();
        hue_pixels.push((c.red * 255.0) as u8);
        hue_pixels.push((c.green * 255.0) as u8);
        hue_pixels.push((c.blue * 255.0) as u8);
        hue_pixels.push(255);
    }
    let mut hue_image = Image::new(
        Extent3d { width: 1, height: 256, depth_or_array_layers: 1 },
        TextureDimension::D2,
        hue_pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    hue_image.sampler = ImageSampler::linear();

    // 2. White to Transparent (Horizontal) (256x1)
    let mut white_pixels = Vec::with_capacity(256 * 4);
    for x in 0..256 {
        let s = x as f32 / 255.0;
        let a_w = srgb_to_linear(1.0 - s);
        let alpha = (a_w * 255.0).clamp(0.0, 255.0) as u8;
        white_pixels.extend_from_slice(&[255, 255, 255, alpha]);
    }
    let mut white_image = Image::new(
        Extent3d { width: 256, height: 1, depth_or_array_layers: 1 },
        TextureDimension::D2,
        white_pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    white_image.sampler = ImageSampler::linear();

    // 3. Transparent to Black (Vertical) (1x256)
    let mut black_pixels = Vec::with_capacity(256 * 4);
    for y in 0..256 {
        let v = 1.0 - (y as f32 / 255.0); // y goes from 0 (top) to 255 (bottom)
        let a_b = 1.0 - srgb_to_linear(v);
        let alpha = (a_b * 255.0).clamp(0.0, 255.0) as u8;
        black_pixels.extend_from_slice(&[0, 0, 0, alpha]);
    }
    let mut black_image = Image::new(
        Extent3d { width: 1, height: 256, depth_or_array_layers: 1 },
        TextureDimension::D2,
        black_pixels,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    );
    black_image.sampler = ImageSampler::linear();

    commands.insert_resource(ColorPickerImages {
        hue_slider: images.add(hue_image),
        white_gradient: images.add(white_image),
        black_gradient: images.add(black_image),
    });
}

#[derive(Component)]
pub struct RuiColorPicker {
    pub color: Color,
    pub hue: f32,
    pub saturation: f32,
    pub value: f32,
    pub is_open: bool,
    pub popup_entity: Entity,
    pub r_slider: Entity,
    pub g_slider: Entity,
    pub b_slider: Entity,
    pub a_slider: Entity,
    pub r_text: Entity,
    pub g_text: Entity,
    pub b_text: Entity,
    pub a_text: Entity,
    pub preview_node: Entity,
    pub sv_area_entity: Entity,
    pub sv_base_image_entity: Entity,
    pub sv_cursor_entity: Entity,
    pub hue_area_entity: Entity,
    pub hue_cursor_entity: Entity,
    pub is_dragging_sv: bool,
    pub is_dragging_hue: bool,
}

#[derive(Component)]
pub struct RuiColorPickerHueBar;

#[derive(Component)]
pub struct RuiColorPickerSvWhite;

#[derive(Component)]
pub struct RuiColorPickerSvBlack;

pub fn apply_color_picker_images(
    images: Option<Res<ColorPickerImages>>,
    mut q_hue: Query<&mut ImageNode, Added<RuiColorPickerHueBar>>,
    mut q_white: Query<&mut ImageNode, (Added<RuiColorPickerSvWhite>, Without<RuiColorPickerHueBar>)>,
    mut q_black: Query<&mut ImageNode, (Added<RuiColorPickerSvBlack>, Without<RuiColorPickerHueBar>, Without<RuiColorPickerSvWhite>)>,
) {
    if let Some(imgs) = images {
        for mut img in &mut q_hue { img.image = imgs.hue_slider.clone(); }
        for mut img in &mut q_white { img.image = imgs.white_gradient.clone(); }
        for mut img in &mut q_black { img.image = imgs.black_gradient.clone(); }
    }
}

#[derive(Component)]
pub struct RuiColorPickerPopup {
    pub picker_entity: Entity,
}

#[derive(Component)]
pub enum ColorChannel { R, G, B, A }

pub fn spawn_color_picker<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    initial_color: Color,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
    let mut s = Node {
        display: Display::Flex,
        width: Val::Px(60.0),
        height: Val::Px(24.0),
        border: UiRect::all(Val::Px(1.0)),
        ..default()
    };
    modifier(&mut s);

    let mut cmds = parent_cmd.spawn((
        s,
        Button,
        crate::focus::Focusable,
        bevy::ui::FocusPolicy::Block,
        Pickable::default(),
        BorderColor::all(Color::srgb(0.3, 0.3, 0.3)),
        RelativeCursorPosition::default(),
    ));
    
    let picker_id = cmds.id();
    
    // Spawn preview node inside the button
    let mut preview_id = Entity::PLACEHOLDER;
    cmds.with_children(|p| {
        preview_id = p.spawn((
            Node { width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(initial_color) },
        )).id();
    });
    
    // Create the popup (hidden by default)
    let mut r_slider = Entity::PLACEHOLDER;
    let mut g_slider = Entity::PLACEHOLDER;
    let mut b_slider = Entity::PLACEHOLDER;
    let mut a_slider = Entity::PLACEHOLDER;
    let mut r_text = Entity::PLACEHOLDER;
    let mut g_text = Entity::PLACEHOLDER;
    let mut b_text = Entity::PLACEHOLDER;
    let mut a_text = Entity::PLACEHOLDER;
    let mut popup_id = Entity::PLACEHOLDER;
    
    let mut sv_area_entity = Entity::PLACEHOLDER;
    let mut sv_base_image_entity = Entity::PLACEHOLDER;
    let mut sv_cursor_entity = Entity::PLACEHOLDER;
    let mut hue_area_entity = Entity::PLACEHOLDER;
    let mut hue_cursor_entity = Entity::PLACEHOLDER;
    
    let hsva: Hsva = initial_color.into();
    let pure_hue = Color::from(Hsva { hue: hsva.hue, saturation: 1.0, value: 1.0, alpha: 1.0 });
    
    cmds.with_children(|parent| {
        popup_id = parent.spawn((
            Node { 
                display: Display::None, 
                position_type: PositionType::Absolute, 
                flex_direction: FlexDirection::Column, 
                width: Val::Px(240.0), 
                padding: UiRect::all(Val::Px(10.0)),
                top: Val::Px(30.0), 
                left: Val::Px(0.0),
                border: UiRect::all(Val::Px(1.0)), 
                ..default() 
            },
            ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::srgb(0.15, 0.15, 0.15)) }, 
            BorderColor::all(Color::srgb(0.3, 0.3, 0.3)), 
            ZIndex(500), 
            GlobalZIndex(100),
            bevy::ui::FocusPolicy::Block,
            RelativeCursorPosition::default(),
            RuiColorPickerPopup { picker_entity: picker_id },
            crate::theme::RuiThemeElement::Window,
        )).with_children(|popup| {
            
            // Top Section: SV Area and Hue Bar
            popup.spawn(Node {
                display: Display::Flex,
                flex_direction: FlexDirection::Row,
                width: Val::Percent(100.0),
                height: Val::Px(150.0),
                margin: UiRect::bottom(Val::Px(10.0)),
                ..default()
            }).with_children(|top_row| {
                // SV Area (150x150)
                sv_area_entity = top_row.spawn((
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(150.0),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::BLACK),
                    RelativeCursorPosition::default(),
                    Interaction::None,
                )).with_children(|sv_box| {
                    // Base color layer (Hue)
                    sv_base_image_entity = sv_box.spawn((
                        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(pure_hue) },
                    )).id();
                    // White horizontal gradient (Saturation)
                    sv_box.spawn((
                        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::default() },
                        RuiColorPickerSvWhite,
                    ));
                    // Black vertical gradient (Value)
                    sv_box.spawn((
                        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::default() },
                        RuiColorPickerSvBlack,
                    ));
                    // Cursor indicator
                    sv_cursor_entity = sv_box.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            left: Val::Percent(hsva.saturation * 100.0),
                            top: Val::Percent((1.0 - hsva.value) * 100.0),
                            width: Val::Px(8.0),
                            height: Val::Px(8.0),
                            margin: UiRect::axes(Val::Px(-4.0), Val::Px(-4.0)), // Center the cursor
                            border: UiRect::all(Val::Px(1.5)),
                            border_radius: BorderRadius::all(Val::Percent(50.0)),
                            ..default()
                        },
                        BorderColor::all(Color::WHITE),
                        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::BLACK) },
                    )).id();
                }).id();
                
                // Hue Bar (20x150)
                hue_area_entity = top_row.spawn((
                    Node {
                        width: Val::Px(24.0),
                        height: Val::Px(150.0),
                        margin: UiRect::left(Val::Px(10.0)),
                        border: UiRect::all(Val::Px(1.0)),
                        ..default()
                    },
                    BorderColor::all(Color::BLACK),
                    RelativeCursorPosition::default(),
                    Interaction::None,
                )).with_children(|hue_box| {
                    // Hue gradient image
                    hue_box.spawn((
                        Node { position_type: PositionType::Absolute, width: Val::Percent(100.0), height: Val::Percent(100.0), ..default() },
                        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::default() },
                        RuiColorPickerHueBar,
                    ));
                    // Hue Cursor
                    hue_cursor_entity = hue_box.spawn((
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Percent(hsva.hue / 360.0 * 100.0),
                            width: Val::Percent(100.0),
                            height: Val::Px(4.0),
                            margin: UiRect::top(Val::Px(-2.0)),
                            border: UiRect::all(Val::Px(1.0)),
                            ..default()
                        },
                        BorderColor::all(Color::BLACK),
                        ImageNode { visual_box: bevy::ui::VisualBox::BorderBox, image_mode: bevy::ui::widget::NodeImageMode::Stretch, ..ImageNode::solid_color(Color::WHITE) },
                    )).id();
                }).id();
            });
            
            // RGBA Inputs below
            let mut spawn_channel = |name: &str, channel: ColorChannel, val: f32| -> (Entity, Entity) {
                let mut slider_ent = Entity::PLACEHOLDER;
                let mut text_ent = Entity::PLACEHOLDER;
                popup.spawn(Node {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceBetween,
                    width: Val::Percent(100.0),
                    margin: UiRect::bottom(Val::Px(5.0)),
                    ..default()
                }).with_children(|row| {
                    row.spawn((Text::new(name), TextFont::default(), TextColor(Color::WHITE), Node { width: Val::Px(20.0), ..default() }));
                    slider_ent = spawn_slider(row, 0.0, 255.0, val, |s| {
                        s.width = Val::Px(100.0);
                        s.height = Val::Px(20.0);
                        s.margin = UiRect::axes(Val::Px(10.0), Val::Px(0.0));
                    }).insert(channel).id();
                    text_ent = spawn_textbox(row, &format!("{}", val.round() as u8), |s, _, _| {
                        s.width = Val::Px(50.0);
                        s.height = Val::Px(24.0);
                    }).id();
                });
                (slider_ent, text_ent)
            };
            
            let color_srgba = initial_color.to_srgba();
            let r_vals = spawn_channel("R", ColorChannel::R, color_srgba.red * 255.0);
            r_slider = r_vals.0; r_text = r_vals.1;
            let g_vals = spawn_channel("G", ColorChannel::G, color_srgba.green * 255.0);
            g_slider = g_vals.0; g_text = g_vals.1;
            let b_vals = spawn_channel("B", ColorChannel::B, color_srgba.blue * 255.0);
            b_slider = b_vals.0; b_text = b_vals.1;
            let a_vals = spawn_channel("A", ColorChannel::A, color_srgba.alpha * 255.0);
            a_slider = a_vals.0; a_text = a_vals.1;
        }).id();
    });
    
    cmds.insert(RuiColorPicker {
        color: initial_color,
        hue: hsva.hue,
        saturation: hsva.saturation,
        value: hsva.value,
        is_open: false,
        popup_entity: popup_id,
        r_slider, g_slider, b_slider, a_slider,
        r_text, g_text, b_text, a_text,
        preview_node: preview_id,
        sv_area_entity,
        sv_base_image_entity,
        sv_cursor_entity,
        hue_area_entity,
        hue_cursor_entity,
        is_dragging_sv: false,
        is_dragging_hue: false,
    });
    
    cmds
}

pub fn handle_color_picker_clicks(
    q_interactions: Query<(Entity, &Interaction), (Changed<Interaction>, With<RuiColorPicker>)>,
    mut q_pickers: Query<&mut RuiColorPicker>,
    mut q_popup: Query<&mut Node>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    for (entity, interaction) in &q_interactions {
        if *interaction == Interaction::Pressed {
            if let Ok(mut picker) = q_pickers.get_mut(entity) {
                picker.is_open = !picker.is_open;
                if picker.is_open {
                    active_scope.push_window(picker.popup_entity);
                } else {
                    active_scope.remove_window(picker.popup_entity);
                }
                if let Ok(mut node) = q_popup.get_mut(picker.popup_entity) {
                    node.display = if picker.is_open { Display::Flex } else { Display::None };
                }
            }
        }
    }
}

pub fn close_color_picker_on_outside_click(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_pickers: Query<(Entity, &mut RuiColorPicker)>,
    q_cursor: Query<&RelativeCursorPosition>,
    mut active_scope: ResMut<crate::focus::RuiActiveScope>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        for (picker_ent, mut picker) in &mut q_pickers {
            if !picker.is_open { continue; }
            
            let mut over_popup = false;
            if let Ok(relative) = q_cursor.get(picker.popup_entity) {
                if relative.cursor_over() {
                    over_popup = true;
                }
            }
            
            let mut over_button = false;
            if let Ok(relative) = q_cursor.get(picker_ent) {
                if relative.cursor_over() {
                    over_button = true;
                }
            }
            
            if !over_popup && !over_button {
                picker.is_open = false;
                active_scope.remove_window(picker.popup_entity);
            }
        }
    }
}

// Separate system to hide popups
pub fn update_color_picker_popups(
    q_pickers: Query<&RuiColorPicker, Changed<RuiColorPicker>>,
    mut q_popup: Query<&mut Node>,
) {
    for picker in &q_pickers {
        if let Ok(mut node) = q_popup.get_mut(picker.popup_entity) {
            node.display = if picker.is_open { Display::Flex } else { Display::None };
        }
    }
}

pub fn handle_color_picker_2d_interaction(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut q_pickers: Query<&mut RuiColorPicker>,
    q_cursor: Query<&RelativeCursorPosition>,
) {
    let left_pressed = mouse_input.pressed(MouseButton::Left);
    let left_just_pressed = mouse_input.just_pressed(MouseButton::Left);

    for mut picker in &mut q_pickers {
        if !picker.is_open { continue; }

        if !left_pressed {
            picker.is_dragging_sv = false;
            picker.is_dragging_hue = false;
        }

        if let Ok(relative) = q_cursor.get(picker.sv_area_entity) {
            if left_just_pressed && relative.cursor_over() {
                picker.is_dragging_sv = true;
            }
            if picker.is_dragging_sv {
                if let Some(pos) = relative.normalized {
                    let nx = pos.x + 0.5;
                    let ny = pos.y + 0.5;
                    picker.saturation = nx.clamp(0.0, 1.0);
                    picker.value = (1.0 - ny).clamp(0.0, 1.0);
                }
            }
        }

        if let Ok(relative) = q_cursor.get(picker.hue_area_entity) {
            if left_just_pressed && relative.cursor_over() {
                picker.is_dragging_hue = true;
            }
            if picker.is_dragging_hue {
                if let Some(pos) = relative.normalized {
                    let ny = pos.y + 0.5;
                    picker.hue = (ny * 360.0).clamp(0.0, 360.0);
                }
            }
        }
    }
}

pub fn sync_color_picker(
    mut q_pickers: Query<&mut RuiColorPicker>,
    mut q_sliders: Query<&mut RuiSlider>,
    mut q_textboxes: Query<&mut RuiTextBox>,
    mut q_images: Query<&mut ImageNode>,
    mut q_nodes: Query<&mut Node>,
) {
    for mut picker in &mut q_pickers {
        if !picker.is_open { continue; }

        let mut color_srgba = picker.color.to_srgba();
        let mut color_changed_from_rgba = false;

        let mut check_channel = |slider_ent: Entity, text_ent: Entity, val: &mut f32| {
            if let Ok(slider) = q_sliders.get(slider_ent) {
                let slider_val = slider.value / 255.0;
                if (slider_val - *val).abs() > 0.001 {
                    *val = slider_val;
                    color_changed_from_rgba = true;
                    if let Ok(mut text) = q_textboxes.get_mut(text_ent) {
                        text.text = format!("{}", slider.value.round() as u8);
                        text.cursor_index = text.text.chars().count();
                    }
                }
            }
            
            if let Ok(text) = q_textboxes.get(text_ent) {
                if let Ok(parsed) = text.text.parse::<f32>() {
                    let parsed = parsed.clamp(0.0, 255.0) / 255.0;
                    if (parsed - *val).abs() > 0.001 {
                        *val = parsed;
                        color_changed_from_rgba = true;
                        if let Ok(mut slider) = q_sliders.get_mut(slider_ent) {
                            slider.value = parsed * 255.0;
                        }
                    }
                }
            }
        };

        if !picker.is_dragging_sv && !picker.is_dragging_hue {
            check_channel(picker.r_slider, picker.r_text, &mut color_srgba.red);
            check_channel(picker.g_slider, picker.g_text, &mut color_srgba.green);
            check_channel(picker.b_slider, picker.b_text, &mut color_srgba.blue);
            check_channel(picker.a_slider, picker.a_text, &mut color_srgba.alpha);
        }

        let mut update_visuals = false;

        if color_changed_from_rgba {
            picker.color = color_srgba.into();
            let hsva: Hsva = picker.color.into();
            // Don't override hue if saturation is 0 (grayscale), to avoid hue jumping to 0 when sliding to gray
            if hsva.saturation > 0.001 {
                picker.hue = hsva.hue;
            }
            picker.saturation = hsva.saturation;
            picker.value = hsva.value;
            update_visuals = true;
        }

        if picker.is_dragging_sv || picker.is_dragging_hue {
            picker.color = Color::from(Hsva {
                hue: picker.hue,
                saturation: picker.saturation,
                value: picker.value,
                alpha: color_srgba.alpha,
            });
            color_srgba = picker.color.to_srgba();
            update_visuals = true;

            let mut set_channel = |slider_ent, text_ent, val: f32| {
                if let Ok(mut slider) = q_sliders.get_mut(slider_ent) { slider.value = val * 255.0; }
                if let Ok(mut text) = q_textboxes.get_mut(text_ent) {
                    text.text = format!("{}", (val * 255.0).round() as u8);
                    text.cursor_index = text.text.chars().count();
                }
            };
            set_channel(picker.r_slider, picker.r_text, color_srgba.red);
            set_channel(picker.g_slider, picker.g_text, color_srgba.green);
            set_channel(picker.b_slider, picker.b_text, color_srgba.blue);
            set_channel(picker.a_slider, picker.a_text, color_srgba.alpha);
        }

        if update_visuals {
            if let Ok(mut image) = q_images.get_mut(picker.preview_node) {
                image.color = picker.color;
            }
            if let Ok(mut image) = q_images.get_mut(picker.sv_base_image_entity) {
                image.color = Color::from(Hsva { hue: picker.hue, saturation: 1.0, value: 1.0, alpha: 1.0 });
            }
            if let Ok(mut node) = q_nodes.get_mut(picker.sv_cursor_entity) {
                node.left = Val::Percent(picker.saturation * 100.0);
                node.top = Val::Percent((1.0 - picker.value) * 100.0);
            }
            if let Ok(mut node) = q_nodes.get_mut(picker.hue_cursor_entity) {
                node.top = Val::Percent(picker.hue / 360.0 * 100.0);
            }
        }
    }
}
