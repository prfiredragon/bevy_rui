use bevy::{prelude::*, window::PresentMode};
use bevy_rui::prelude::*;

#[derive(Component)]
struct RotatingCube;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "bevy_rui - example_game_viewport".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(RuiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, rotate_cube)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut theme: ResMut<RuiTheme>,
) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));
    apply_game_theme(&asset_server, &mut theme);

    commands.spawn((
        Camera2d,
        Camera {
            order: 1,
            clear_color: ClearColorConfig::None,
            ..default()
        },
        Msaa::Off,
        UiPickingCamera,
    ));

    let viewport_camera = commands
        .spawn((
            Camera3d::default(),
            Transform::from_xyz(0.0, 1.8, 4.5).looking_at(Vec3::ZERO, Vec3::Y),
            Camera {
                order: 0,
                is_active: false,
                ..default()
            },
            Msaa::Off,
            RuiViewportCamera,
        ))
        .id();

    commands.spawn((
        DirectionalLight {
            shadow_maps_enabled: false,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.2, 1.2))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.8, 0.1),
            ..default()
        })),
        Transform::from_xyz(0.0, 0.0, 0.0),
        RotatingCube,
    ));

    commands.rui_root(
        |s| {
            s.justify_content = JustifyContent::Center;
            s.align_items = AlignItems::Center;
        },
        |ui| {
            let mut root = ui.reborrow();
            root.window(
                "Game Viewport (Vista 3D)",
                false,
                |s| {
                    s.width = Val::Px(520.0);
                    s.height = Val::Px(360.0);
                    s.left = Val::Auto;
                    s.top = Val::Auto;
                },
                |win, _| {
                    let mut c = win.reborrow();
                    c.reborrow()
                        .vbox(
                            |s| {
                                s.flex_grow = 1.0;
                                s.border = UiRect::all(Val::Px(2.0));
                            },
                            move |vp_frame| {
                                vp_frame.reborrow().viewport(viewport_camera, |s| {
                                    s.flex_grow = 1.0;
                                });
                            },
                        )
                        .insert(BorderColor::from(Color::srgb(1.0, 0.8, 0.0)));
                },
            );
        },
    );
}

fn rotate_cube(time: Res<Time>, mut q_cube: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut q_cube {
        transform.rotate_y(time.delta_secs() * 1.1);
        transform.rotate_x(time.delta_secs() * 0.6);
    }
}

fn apply_game_theme(asset_server: &AssetServer, theme: &mut RuiTheme) {
    theme.image_panel = Some(asset_server.load("theme/kenney/blue/button_square_flat.png"));
    theme.panel_margin = 10.0;
    theme.image_window_header = Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.window_header_margin = 10.0;
    theme.window_header_layout_margin = UiRect::all(Val::Px(5.0));
    theme.color_window_header_text = Color::srgb(0.2, 0.2, 0.2);
    theme.image_button_normal = Some(asset_server.load("theme/kenney/yellow/button_square_depth_gradient.png"));
    theme.image_button_hover = Some(asset_server.load("theme/kenney/yellow/button_square_depth_flat.png"));
    theme.image_button_pressed = Some(asset_server.load("theme/kenney/yellow/button_square_flat.png"));
    theme.button_margin = 10.0;
    theme.color_button_text = Color::srgb(0.2, 0.2, 0.2);
}
