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
        .add_systems(Update, add_content)
        .run();
}

fn prepare(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            background_color: BackgroundColor(Color::BLUE),
            ..default()
        })
        .with_children(|p| {
            p.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        height: Val::Percent(50.0),
                        ..default()
                    },
                    background_color: BackgroundColor(Color::YELLOW),
                    ..default()
                },
                ScrollView::default(),
            ));
        });
}

fn add_content(mut commands: Commands, q: Query<Entity, Added<ScrollableContent>>) {
    for e in q.iter() {
        commands.entity(e).with_children(|parent| {
            for _ in 0..10 {
                parent.spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        height: Val::Px(200.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        border: UiRect::all(Val::Px(5.0)),
                        padding: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    border_color: BORDER_COLOR_ACTIVE.into(),
                    background_color: BACKGROUND_COLOR.into(),
                    ..default()
                });
            }
        });
    }
}
