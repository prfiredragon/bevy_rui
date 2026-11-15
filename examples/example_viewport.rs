use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;

#[derive(Component)]
struct RotatingCube;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_viewport".to_string(),
                ..default()
            }),
            ..default()
        }))
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
) {
    commands.insert_resource(RuiDefaultFont(asset_server.load("fonts/afacad.ttf")));

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
            base_color: Color::srgb(0.25, 0.65, 0.95),
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
            root.vbox(
                |s| {
                    s.width = Val::Px(540.0);
                    s.height = Val::Px(380.0);
                    s.padding = UiRect::all(Val::Px(20.0));
                    s.row_gap = Val::Px(12.0);
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Viewport (Cámara 3D en UI)", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

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
                        .insert(BorderColor::from(Color::srgb(0.4, 0.6, 0.9)));
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}

fn rotate_cube(time: Res<Time>, mut q_cube: Query<&mut Transform, With<RotatingCube>>) {
    for mut transform in &mut q_cube {
        transform.rotate_y(time.delta_secs() * 1.1);
        transform.rotate_x(time.delta_secs() * 0.6);
    }
}
