use bevy::{input_focus::{InputFocus, FocusCause}, prelude::*};
use bevy::ui::auto_directional_navigation::AutoDirectionalNavigator;
use bevy::math::CompassOctant;

#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct RuiNavigationBindings {
    pub up_keys: Vec<KeyCode>,
    pub down_keys: Vec<KeyCode>,
    pub left_keys: Vec<KeyCode>,
    pub right_keys: Vec<KeyCode>,
    pub accept_keys: Vec<KeyCode>,
    pub cancel_keys: Vec<KeyCode>,

    pub up_buttons: Vec<GamepadButton>,
    pub down_buttons: Vec<GamepadButton>,
    pub left_buttons: Vec<GamepadButton>,
    pub right_buttons: Vec<GamepadButton>,
    pub accept_buttons: Vec<GamepadButton>,
    pub cancel_buttons: Vec<GamepadButton>,
}

#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct RuiNavigationConfig {
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

#[derive(Message, Clone, Debug)]
pub struct RuiClickEvent {
    pub entity: Entity,
}

#[derive(Component)]
pub struct RuiSimulateClick(pub Timer);

impl Default for RuiSimulateClick {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Once))
    }
}

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

pub fn check_keys(input: &ButtonInput<KeyCode>, keys: &[KeyCode]) -> bool {
    keys.iter().any(|k| input.just_pressed(*k))
}

pub fn check_buttons(buttons: &[GamepadButton], gamepads: &Query<&Gamepad>) -> bool {
    buttons.iter().any(|b| {
        gamepads.iter().any(|g| g.just_pressed(*b))
    })
}

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

    let mut move_dir = Vec2::ZERO;

    if check_keys(&keys, &bindings.up_keys) || check_buttons(&bindings.up_buttons, &gamepads) {
        move_dir.y += 1.0;
    }
    if check_keys(&keys, &bindings.down_keys) || check_buttons(&bindings.down_buttons, &gamepads) {
        move_dir.y -= 1.0;
    }
    if check_keys(&keys, &bindings.left_keys) || check_buttons(&bindings.left_buttons, &gamepads) {
        move_dir.x -= 1.0;
    }
    if check_keys(&keys, &bindings.right_keys) || check_buttons(&bindings.right_buttons, &gamepads) {
        move_dir.x += 1.0;
    }

    if let Some(focus_entity) = navigator.input_focus() {
        if sliders.get(focus_entity).is_ok() {
            move_dir.x = 0.0;
        }
    }

    if move_dir.length_squared() < 0.1 {
        return;
    }

    let maybe_direction = Dir2::from_xy(move_dir.x, move_dir.y)
        .ok()
        .map(CompassOctant::from);

    if let Some(direction) = maybe_direction {
        if navigator.input_focus().is_none() {
            if let Some(entity) = navigable.iter().next() {
                navigator.manual_directional_navigation.focus.set(entity, FocusCause::Navigated);
                *last_nav_time = now;
            }
        } else {
            if navigator.navigate(direction).is_ok() {
                *last_nav_time = now;
            }
        }
    }
}

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

pub fn simulate_click_visuals_system(
    mut commands: Commands,
    mut click_events: MessageReader<RuiClickEvent>,
    mut interactions: Query<&mut Interaction, Without<RuiSimulateClick>>,
    mut simulate_tasks: Query<(Entity, &mut Interaction, &mut RuiSimulateClick)>,
    time: Res<Time>,
) {
    for ev in click_events.read() {
        if let Ok(mut interaction) = interactions.get_mut(ev.entity) {
            *interaction = Interaction::Pressed;
            if let Ok(mut e_cmds) = commands.get_entity(ev.entity) {
                e_cmds.try_insert(RuiSimulateClick::default());
            }
        }
    }

    for (entity, mut interaction, mut task) in &mut simulate_tasks {
        task.0.tick(time.delta());
        if task.0.just_finished() {
            *interaction = Interaction::Hovered;
            commands.entity(entity).remove::<RuiSimulateClick>();
        }
    }
}
