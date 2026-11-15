use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

#[derive(Component)]
struct AnimatedProgress {
    value: f32,
}

#[derive(Component)]
struct ProgressLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_progress_bar".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, animate_progress_bar)
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
                    s.width = Val::Px(420.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(16.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Progress Bar", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow()
                        .progress_bar(0.0, 100.0, 65.0, |s| {
                            s.width = Val::Percent(100.0);
                            s.height = Val::Px(22.0);
                        })
                        .insert(AnimatedProgress { value: 65.0 });

                    c.reborrow()
                        .label("Progreso: 65%", |font, color| {
                            font.font_size = FontSize::Px(18.0);
                            color.0 = Color::srgb(0.5, 0.9, 0.6);
                        })
                        .insert(ProgressLabel);
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}

fn animate_progress_bar(
    time: Res<Time>,
    mut q_bar: Query<(&mut AnimatedProgress, &Children)>,
    mut q_node: Query<&mut Node>,
    mut q_label: Query<&mut Text, With<ProgressLabel>>,
) {
    for (mut prog, children) in &mut q_bar {
        prog.value = (prog.value + time.delta_secs() * 20.0) % 100.0;
        if let Some(&fill_entity) = children.first() {
            if let Ok(mut fill_node) = q_node.get_mut(fill_entity) {
                fill_node.width = Val::Percent(prog.value);
            }
        }
        if let Ok(mut txt) = q_label.single_mut() {
            txt.0 = format!("Progreso: {:.0}%", prog.value);
        }
    }
}
