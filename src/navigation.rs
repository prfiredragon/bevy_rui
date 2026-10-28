//! Directional UI navigation for keyboard and gamepads.
//!
//! Built on top of Bevy 0.20's `AutoDirectionalNavigator` and `InputFocus`.
//! Enable navigation at runtime by setting [`RuiNavigationConfig::enabled`] to `true`.

use bevy::{input_focus::{InputFocus, FocusCause}, prelude::*};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigator;
use bevy::math::CompassOctant;

/// Configurable keyboard and gamepad input bindings for UI directional navigation.
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct RuiNavigationBindings {
    /// Keys that move focus upward (defaults to `ArrowUp`, `KeyW`).
    pub up_keys: Vec<KeyCode>,
    /// Keys that move focus downward (defaults to `ArrowDown`, `KeyS`).
    pub down_keys: Vec<KeyCode>,
    /// Keys that move focus left (defaults to `ArrowLeft`, `KeyA`).
    pub left_keys: Vec<KeyCode>,
    /// Keys that move focus right (defaults to `ArrowRight`, `KeyD`).
    pub right_keys: Vec<KeyCode>,
    /// Keys that activate/click the focused widget (defaults to `Enter`, `Space`).
    pub accept_keys: Vec<KeyCode>,
    /// Keys that cancel or close the current menu/scope (defaults to `Escape`, `Backspace`).
    pub cancel_keys: Vec<KeyCode>,

    /// Gamepad buttons that move focus upward (defaults to `DPadUp`).
    pub up_buttons: Vec<GamepadButton>,
    /// Gamepad buttons that move focus downward (defaults to `DPadDown`).
    pub down_buttons: Vec<GamepadButton>,
    /// Gamepad buttons that move focus left (defaults to `DPadLeft`).
    pub left_buttons: Vec<GamepadButton>,
    /// Gamepad buttons that move focus right (defaults to `DPadRight`).
    pub right_buttons: Vec<GamepadButton>,
    /// Gamepad buttons that activate/click the focused widget (defaults to `South`).
    pub accept_buttons: Vec<GamepadButton>,
    /// Gamepad buttons that cancel or close the current scope (defaults to `East`).
    pub cancel_buttons: Vec<GamepadButton>,
}

/// Resource controlling whether directional keyboard/gamepad UI navigation is active.
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct RuiNavigationConfig {
    /// Whether directional navigation is enabled (`false` by default).
    pub enabled: bool,
}

impl Default for RuiNavigationConfig {
    fn default() -> Self {
        Self { enabled: false }
    }
}

impl Default for RuiNavigationBindings {
    fn default() -> Self {
        Self {
            up_keys: vec![KeyCode::ArrowUp, KeyCode::KeyW],
            down_keys: vec![KeyCode::ArrowDown, KeyCode::KeyS],
            left_keys: vec![KeyCode::ArrowLeft, KeyCode::KeyA],
            right_keys: vec![KeyCode::ArrowRight, KeyCode::KeyD],
            accept_keys: vec![KeyCode::Enter, KeyCode::Space],
            cancel_keys: vec![KeyCode::Escape, KeyCode::Backspace],

            up_buttons: vec![GamepadButton::DPadUp],
            down_buttons: vec![GamepadButton::DPadDown],
            left_buttons: vec![GamepadButton::DPadLeft],
            right_buttons: vec![GamepadButton::DPadRight],
            accept_buttons: vec![GamepadButton::South],
            cancel_buttons: vec![GamepadButton::East],
        }
    }
}

/// Message emitted when the user presses an accept key/button on a focused UI entity.
#[derive(Message, Clone, Debug)]
pub struct RuiClickEvent {
    /// The focused UI entity being activated.
    pub entity: Entity,
}

/// Temporary timer component inserted on an entity to briefly simulate [`bevy::ui::Pressed`] visuals.
#[derive(Component)]
pub struct RuiSimulateClick(pub Timer);

impl Default for RuiSimulateClick {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

/// Plugin registering [`RuiNavigationBindings`], [`RuiNavigationConfig`], [`RuiClickEvent`], and navigation systems.
pub struct RuiNavigationPlugin;

impl Plugin for RuiNavigationPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<bevy::input_focus::InputDispatchPlugin>() {
            app.add_plugins(bevy::input_focus::InputDispatchPlugin);
        }
        if !app.is_plugin_added::<bevy::input_focus::directional_navigation::DirectionalNavigationPlugin>() {
            app.add_plugins(bevy::input_focus::directional_navigation::DirectionalNavigationPlugin);
        }

        app.init_resource::<RuiNavigationBindings>()
            .init_resource::<RuiNavigationConfig>()
            .register_type::<RuiNavigationBindings>()
            .add_message::<RuiClickEvent>()
            .add_systems(Update, (
                navigate_ui_system,
                trigger_ui_click_system,
            ))
            .add_systems(PreUpdate, simulate_click_visuals_system.after(bevy::ui::UiSystems::Focus));
    }
}

/// Returns `true` if any of the specified [`KeyCode`]s were just pressed.
pub fn check_keys(input: &ButtonInput<KeyCode>, keys: &[KeyCode]) -> bool {
    keys.iter().any(|k| input.just_pressed(*k))
}

/// Returns `true` if any of the specified [`GamepadButton`]s were just pressed on any connected gamepad.
pub fn check_buttons(buttons: &[GamepadButton], gamepads: &Query<&Gamepad>) -> bool {
    buttons.iter().any(|b| {
        gamepads.iter().any(|g| g.just_pressed(*b))
    })
}

/// Computes the 2D directional navigation vector from keyboard and gamepad bindings.
fn get_navigation_vector(b: &RuiNavigationBindings, k: &ButtonInput<KeyCode>, g: &Query<&Gamepad>) -> Vec2 {
    let mut v = Vec2::ZERO;
    if check_keys(k, &b.up_keys) || check_buttons(&b.up_buttons, g) { v.y += 1.0; }
    if check_keys(k, &b.down_keys) || check_buttons(&b.down_buttons, g) { v.y -= 1.0; }
    if check_keys(k, &b.left_keys) || check_buttons(&b.left_buttons, g) { v.x -= 1.0; }
    if check_keys(k, &b.right_keys) || check_buttons(&b.right_buttons, g) { v.x += 1.0; }
    v
}

/// System that moves [`InputFocus`] between navigable UI widgets using [`AutoDirectionalNavigator`].
/// Horizontal navigation is suppressed when a [`crate::widgets::slider::RuiSlider`] is focused so left/right adjusts its value.
pub fn navigate_ui_system(
    bindings: Res<RuiNavigationBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    mut navigator: AutoDirectionalNavigator,
    navigable: Query<Entity, With<bevy::ui::auto_directional_navigation::AutoDirectionalNavigation>>,
    time: Res<Time>,
    mut last_nav_time: Local<f32>,
    config: Res<RuiNavigationConfig>,
    sliders: Query<(), With<crate::widgets::slider::RuiSlider>>,
) {
    if !config.enabled { return; }
    let now = time.elapsed_secs();
    if now - *last_nav_time < 0.15 { return; }

    let mut move_dir = get_navigation_vector(&bindings, &keys, &gamepads);

    if let Some(focus_entity) = navigator.input_focus() {
        if sliders.get(focus_entity).is_ok() { move_dir.x = 0.0; }
    }
    if move_dir.length_squared() < 0.1 { return; }

    if let Some(direction) = Dir2::from_xy(move_dir.x, move_dir.y).ok().map(CompassOctant::from) {
        if navigator.input_focus().is_none() {
            if let Some(entity) = navigable.iter().next() {
                navigator.manual_directional_navigation.focus.set(entity, FocusCause::Navigated);
                *last_nav_time = now;
            }
        } else if navigator.navigate(direction).is_ok() {
            *last_nav_time = now;
        }
    }
}

/// System that emits [`RuiClickEvent`] when an accept key or gamepad button is pressed on a focused widget.
pub fn trigger_ui_click_system(
    bindings: Res<RuiNavigationBindings>,
    keys: Res<ButtonInput<KeyCode>>,
    gamepads: Query<&Gamepad>,
    input_focus: Res<InputFocus>,
    mut click_events: MessageWriter<RuiClickEvent>,
    config: Res<RuiNavigationConfig>,
) {
    if !config.enabled { return; }
    let Some(focused_entity) = input_focus.get() else { return; };
    if check_keys(&keys, &bindings.accept_keys) || check_buttons(&bindings.accept_buttons, &gamepads) {
        click_events.write(RuiClickEvent { entity: focused_entity });
    }
}

/// System that applies a brief [`bevy::ui::Pressed`] state on entities receiving [`RuiClickEvent`].
pub fn simulate_click_visuals_system(
    mut commands: Commands,
    mut click_events: MessageReader<RuiClickEvent>,
    q_simulate_tasks: Query<(Entity, &RuiSimulateClick)>,
    time: Res<Time>,
) {
    for ev in click_events.read() {
        if let Ok(mut e_cmds) = commands.get_entity(ev.entity) {
            e_cmds.try_insert((bevy::ui::Pressed, RuiSimulateClick::default()));
        }
    }
    for (entity, task) in &q_simulate_tasks {
        let mut updated_task = RuiSimulateClick(task.0.clone());
        updated_task.0.tick(time.delta());
        if updated_task.0.just_finished() {
            if let Ok(mut e_cmds) = commands.get_entity(entity) {
                e_cmds.remove::<bevy::ui::Pressed>();
                e_cmds.try_insert(bevy::picking::hover::Hovered(true));
                e_cmds.remove::<RuiSimulateClick>();
            }
        } else if let Ok(mut e_cmds) = commands.get_entity(entity) {
            e_cmds.insert(updated_task);
        }
    }
}
