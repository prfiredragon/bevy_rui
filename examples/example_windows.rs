use bevy::prelude::*;
use bevy::text::FontSize;
use bevy::ui_widgets::Activate;
use bevy_rui::prelude::*;

#[derive(Component)]
struct WindowRoot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_windows".to_string(),
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

    let root_id = commands
        .rui_root(
            |s| {
                s.justify_content = JustifyContent::Center;
                s.align_items = AlignItems::Center;
            },
            |ui| {
                let mut root = ui.reborrow();
                spawn_centered_demo_window(&mut root);
            },
        )
        .id();

    commands.entity(root_id).insert(WindowRoot);
}

fn spawn_centered_demo_window<'a>(ui: &'a mut ChildSpawnerCommands<'a>) {
    ui.window(
        "Ventana Flotante (Arrástrame)",
        true,
        |s| {
            s.width = Val::Px(380.0);
            s.left = Val::Auto;
            s.top = Val::Auto;
        },
        |win, _| {
            let mut w = win.reborrow();
            w.reborrow().label("Widget: Windows (RuiWindow)", |f, col| {
                f.font_size = FontSize::Px(22.0);
                col.0 = Color::WHITE;
            });

            w.reborrow().label(
                "Puedes arrastrar esta ventana desde su barra de título superior.",
                |f, col| {
                    f.font_size = FontSize::Px(16.0);
                    col.0 = Color::srgb(0.75, 0.75, 0.8);
                },
            );

            w.reborrow()
                .button(
                    |s| {
                        s.width = Val::Percent(100.0);
                        s.height = Val::Px(40.0);
                        s.margin = UiRect::top(Val::Px(14.0));
                    },
                    |btn| {
                        btn.reborrow()
                            .label("Crear Otra Ventana", |f, _| f.font_size = FontSize::Px(16.0))
                            .insert(RuiThemeElement::ButtonText);
                    },
                )
                .observe(
                    |_: On<Activate>,
                     mut commands: Commands,
                     q_root: Query<Entity, With<WindowRoot>>| {
                        if let Ok(root_e) = q_root.single() {
                            commands.entity(root_e).with_children(|ui| {
                                let mut spawner = ui.reborrow();
                                spawn_centered_demo_window(&mut spawner);
                            });
                        }
                    },
                );
        },
    );
}
