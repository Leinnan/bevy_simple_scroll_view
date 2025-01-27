#![allow(missing_docs)]

use bevy::picking::events::{Pointer, Up};
use bevy::prelude::*;
use bevy_simple_scroll_view::*;

const BG_COLOR: BackgroundColor = BackgroundColor(Color::srgb(0.168, 0.168, 0.168));
const BG_COLOR_2: BackgroundColor = BackgroundColor(Color::srgb(0.109, 0.109, 0.109));
const CLR_3: Color = Color::srgb(0.569, 0.592, 0.647);
const TEXT_COLOR: TextColor = TextColor(Color::srgb(0.902, 0.4, 0.004));

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ScrollViewPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    let base_node = Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        min_width: Val::Px(200.0),
        margin: UiRect::all(Val::Px(10.0)),
        border: UiRect::all(Val::Px(3.0)),
        padding: UiRect::all(Val::Px(15.0)),
        ..default()
    };
    commands.spawn(Camera2d);
    commands
        .spawn((
            BG_COLOR,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(Node {
                width: Val::Percent(20.0),
                margin: UiRect::all(Val::Px(15.0)),
                flex_direction: FlexDirection::Column,
                ..default()
            })
            .with_children(|p| {
                p.spawn((Text::new("Scroll to:"), TEXT_COLOR));
                let btn = (base_node.clone(), BG_COLOR_2, Button);
                p.spawn(btn.clone())
                    .observe(scroll_to_top)
                    .with_child((Text::new("top"), TEXT_COLOR));
                p.spawn(btn)
                    .observe(scroll_to_bottom)
                    .with_child((Text::new("bottom"), TEXT_COLOR));
            });
            p.spawn((
                Node {
                    width: Val::Percent(80.0),
                    margin: UiRect::all(Val::Px(15.0)),
                    ..scroll_view_node()
                },
                BG_COLOR_2,
                ScrollView::default(),
            ))
            .with_children(|p| {
                p.spawn(ScrollableContent::default())
                    .with_children(|scroll_area| {
                        for i in 1..21 {
                            scroll_area
                                .spawn((base_node.clone(), BorderColor(CLR_3)))
                                .with_child((Text::new(format!("Nr {} out of 20", i)), TEXT_COLOR));
                        }
                    });
            });
        });
}

fn scroll_to_top(_t: Trigger<Pointer<Up>>, mut scroll: Single<&mut ScrollableContent>) {
    scroll.scroll_to_top();
}

fn scroll_to_bottom(_t: Trigger<Pointer<Up>>, mut scroll: Single<&mut ScrollableContent>) {
    scroll.scroll_to_bottom();
}
