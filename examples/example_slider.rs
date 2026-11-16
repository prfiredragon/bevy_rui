use bevy::prelude::*;
use bevy::text::FontSize;
use bevy_rui::prelude::*;
use bevy_rui::widgets::slider::RuiSlider;

#[derive(Component)]
struct SliderValueLabel;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "bevy_rui - example_slider".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RuiPlugin)
        .insert_resource(RuiNavigationConfig { enabled: true })
        .add_systems(Startup, setup)
        .add_systems(Update, sync_slider_label)
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
                    s.width = Val::Px(400.0);
                    s.height = Val::Auto;
                    s.padding = UiRect::all(Val::Px(24.0));
                    s.row_gap = Val::Px(18.0);
                    s.align_items = AlignItems::Center;
                    s.border = UiRect::all(Val::Px(2.0));
                    s.border_radius = BorderRadius::all(Val::Px(8.0));
                },
                |card| {
                    let mut c = card.reborrow();

                    c.reborrow().label("Widget: Slider", |font, color| {
                        font.font_size = FontSize::Px(24.0);
                        color.0 = Color::WHITE;
                    });

                    c.reborrow().slider(0.0, 100.0, 50.0, |s| {
                        s.width = Val::Px(280.0);
                    });

                    c.reborrow()
                        .label("Valor actual: 50.0", |font, color| {
                            font.font_size = FontSize::Px(18.0);
                            color.0 = Color::srgb(0.5, 0.9, 0.6);
                        })
                        .insert(SliderValueLabel);
                },
            )
            .insert((
                BackgroundColor(Color::srgb(0.15, 0.15, 0.17)),
                BorderColor::from(Color::srgb(0.3, 0.3, 0.35)),
            ));
        },
    );
}

fn sync_slider_label(
    q_slider: Query<&RuiSlider, Changed<RuiSlider>>,
    mut q_label: Query<&mut Text, With<SliderValueLabel>>,
) {
    for slider in &q_slider {
        if let Ok(mut txt) = q_label.single_mut() {
            txt.0 = format!("Valor actual: {:.1}", slider.value);
        }
    }
}
