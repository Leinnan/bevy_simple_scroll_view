use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_simple_scroll_view::*;

const BORDER_COLOR_ACTIVE: Color = Color::rgb(0.75, 0.52, 0.99);
const BACKGROUND_COLOR: Color = Color::rgb(0.15, 0.15, 0.15);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ScrollViewPlugin,
            WorldInspectorPlugin::new(),
        ))
        .add_systems(Startup, prepare)
        .add_systems(Update, reset_scroll)
        .run();
}

fn prepare(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                margin: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: Color::rgb(0.05, 0.05, 0.05).into(),
            ..default()
        })
        .with_children(|p| {
            p.spawn(ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    padding: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: BORDER_COLOR_ACTIVE.into(),
                ..default()
            })
            .with_children(|p| {
                p.spawn(TextBundle::from_section(
                    "Reset scroll",
                    TextStyle {
                        font_size: 25.0,
                        color: Color::ANTIQUE_WHITE,
                        ..default()
                    },
                ));
            });
            p.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(50.0),
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                },
                ScrollView::default(),
            ))
            .with_children(|p| {
                p.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: bevy::ui::FlexDirection::Column,
                            ..default()
                        },
                        ..default()
                    },
                    ScrollableContent::default(),
                ))
                .with_children(|scroll_area| {
                    for i in 0..10 {
                        scroll_area
                            .spawn(NodeBundle {
                                style: Style {
                                    width: Val::Percent(150.0),
                                    margin: UiRect::all(Val::Px(5.0)),
                                    border: UiRect::all(Val::Px(3.0)),
                                    padding: UiRect::all(Val::Px(25.0)),
                                    ..default()
                                },
                                border_color: BORDER_COLOR_ACTIVE.into(),
                                background_color: BACKGROUND_COLOR.into(),
                                ..default()
                            })
                            .with_children(|p| {
                                p.spawn(
                                    TextBundle::from_section(
                                        format!("Nr {}", i),
                                        TextStyle {
                                            font_size: 25.0,
                                            color: Color::ANTIQUE_WHITE,
                                            ..default()
                                        },
                                    )
                                    .with_text_justify(JustifyText::Center),
                                );
                            });
                    }
                });
            });
        });
}

fn reset_scroll(
    q: Query<(&Button, &Interaction), Changed<Interaction>>,
    mut scrolls_q: Query<&mut ScrollableContent>,
) {
    for (_, interaction) in q.iter() {
        if interaction == &Interaction::Pressed {
            for mut scroll in scrolls_q.iter_mut() {
                scroll.pos_y = 0.0;
            }
        }
    }
}
