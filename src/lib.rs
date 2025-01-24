#![doc = include_str!("../README.md")]

use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

/// A `Plugin` providing the systems and components required to make a ScrollView work.
///
/// # Example
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_simple_scroll_view::*;
///
/// App::new()
///     .add_plugins((DefaultPlugins,ScrollViewPlugin))
///     .run();
/// ```
pub struct ScrollViewPlugin;

impl Plugin for ScrollViewPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ScrollView>()
            .register_type::<ScrollableContent>()
            .add_systems(
                Update,
                (
                    create_scroll_view,
                    update_size,
                    input_mouse_pressed_move,
                    input_touch_pressed_move,
                    scroll_events,
                    scroll_update,
                )
                    .chain(),
            );
    }
}

/// Root component of scroll, it should have clipped style.
#[derive(Component, Debug, Reflect)]
#[require(Interaction, Node)]
pub struct ScrollView {
    /// Field which control speed of the scrolling.
    /// Could be negative number to implement invert scroll
    pub scroll_speed: f32,
}

impl Default for ScrollView {
    fn default() -> Self {
        Self {
            scroll_speed: 1200.0,
        }
    }
}

/// Component containing offset value of the scroll container to the parent.
/// It is possible to update the field `pos_y` manually to move scrollview to desired location.
#[derive(Component, Debug, Reflect, Default)]
#[require(Node(scroll_view_node))]
pub struct ScrollableContent {
    /// Scroll container offset to the `ScrollView`.
    pub pos_y: f32,
    pub max_scroll: f32,
}

pub fn scroll_view_node() -> Node {
    Node {
        overflow: Overflow::clip(),
        align_items: AlignItems::Start,
        align_self: AlignSelf::Stretch,
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

impl ScrollableContent {
    pub fn scroll_to_top(&mut self) {
        self.pos_y = 0.0;
    }
    pub fn scroll_to_bottom(&mut self) {
        self.pos_y = -self.max_scroll;
    }
    pub fn scroll_by(&mut self, value: f32) {
        self.pos_y += value;
        self.pos_y = self.pos_y.clamp(-self.max_scroll, 0.);
    }
}

pub fn create_scroll_view(mut q: Query<&mut Node, Added<ScrollView>>) {
    let Node {
        overflow,
        align_items,
        align_self,
        flex_direction,
        ..
    } = scroll_view_node();
    for mut style in q.iter_mut() {
        style.overflow = overflow;
        style.align_items = align_items;
        style.align_self = align_self;
        style.flex_direction = flex_direction;
    }
}

fn input_mouse_pressed_move(
    mut motion_evr: EventReader<MouseMotion>,
    mut q: Query<(&Children, &Interaction), With<ScrollView>>,
    mut content_q: Query<&mut ScrollableContent>,
) {
    for evt in motion_evr.read() {
        for (children, &interaction) in q.iter_mut() {
            if interaction != Interaction::Pressed {
                continue;
            }
            for &child in children.iter() {
                let Ok(mut scroll) = content_q.get_mut(child) else {
                    continue;
                };
                scroll.scroll_by(evt.delta.y);
            }
        }
    }
}

fn update_size(
    mut q: Query<(&Children, &ComputedNode), With<ScrollView>>,
    mut content_q: Query<(&mut ScrollableContent, &ComputedNode), Changed<ComputedNode>>,
) {
    for (children, scroll_view_node) in q.iter_mut() {
        let container_height = scroll_view_node.size().y * scroll_view_node.inverse_scale_factor();
        for &child in children.iter() {
            let Ok((mut scroll, node)) = content_q.get_mut(child) else {
                continue;
            };

            scroll.max_scroll =
                (node.size().y * node.inverse_scale_factor() - container_height).max(0.0);
            #[cfg(feature = "extra_logs")]
            info!(
                "CONTAINER {}, max_scroll: {}",
                container_height, scroll.max_scroll
            );
        }
    }
}

fn input_touch_pressed_move(
    touches: Res<Touches>,
    mut q: Query<(&Children, &Interaction), With<ScrollView>>,
    mut content_q: Query<&mut ScrollableContent>,
) {
    for t in touches.iter() {
        let Some(touch) = touches.get_pressed(t.id()) else {
            continue;
        };

        for (children, &interaction) in q.iter_mut() {
            if interaction != Interaction::Pressed {
                continue;
            }
            for &child in children.iter() {
                let Ok(mut scroll) = content_q.get_mut(child) else {
                    continue;
                };
                scroll.scroll_by(touch.delta().y);
            }
        }
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut q: Query<(&Children, &Interaction, &ScrollView), With<ScrollView>>,
    time: Res<Time>,
    mut content_q: Query<&mut ScrollableContent>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.read() {
        for (children, &interaction, scroll_view) in q.iter_mut() {
            if interaction != Interaction::Hovered {
                continue;
            }
            let y = match ev.unit {
                MouseScrollUnit::Line => {
                    ev.y * time.delta().as_secs_f32() * scroll_view.scroll_speed
                }
                MouseScrollUnit::Pixel => ev.y,
            };
            #[cfg(feature = "extra_logs")]
            info!("Scroolling by {:#?}: {} movement", ev.unit, y);

            for &child in children.iter() {
                let Ok(mut scroll) = content_q.get_mut(child) else {
                    continue;
                };
                scroll.scroll_by(y);
            }
        }
    }
}

fn scroll_update(mut q: Query<(&ScrollableContent, &mut Node), Changed<ScrollableContent>>) {
    for (scroll, mut style) in q.iter_mut() {
        style.top = Val::Px(scroll.pos_y);
    }
}
