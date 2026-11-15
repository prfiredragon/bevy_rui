use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::prelude::*;

#[derive(Component)]
struct CheckboxStatusText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_checkbox".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    commands.spawn(Camera2d);

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.vbox(
                |s| {
                    s.width = Val::Px(380.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(14.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Checkbox", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow()
                        .checkbox("Activar Música", true, |_| {})
                        .observe(update_status);

                    c.reborrow()
                        .checkbox("Pantalla Completa", false, |_| {})
                        .observe(update_status);

                    c.reborrow()
                        .checkbox("Mostrar FPS", true, |_| {})
                        .observe(update_status);

                    c.reborrow()
                        .label("Activos: 2 / 3", |font, color| {
                            font.font_size = FontSize::Px(16.0);
                            color.0 = Color::srgb(0.5, 0.9, 0.6);
                        })
                        .insert(CheckboxStatusText);
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}

fn update_status(
    _: On<Activate>,
    q_checkboxes: Query<&RuiCheckbox>,
    mut q_status: Query<&mut Text, With<CheckboxStatusText>>,
) {
    let total = q_checkboxes.iter().count();
    let checked = q_checkboxes.iter().filter(|c| c.checked).count();
    if let Ok(mut txt) = q_status.single_mut() {
        txt.0 = format!("Activos: {checked} / {total}");
    }
}
