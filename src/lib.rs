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
                    fling_update,
                    scroll_update,
                )
                    .chain(),
            );
    }
}

/// Root component of scroll, it should have clipped style.
#[derive(Component, Debug, Reflect)]
#[require(Interaction, Node = scroll_view_node())]
pub struct ScrollView {
    /// Field which control speed of the scrolling.
    /// Could be negative number to implement invert scroll
    pub scroll_speed: f32,
    /// Amount of friction to apply to slow down the fling
    pub friction: f32,
    /// Current vertical velocity
    pub velocity: f32,
}

impl Default for ScrollView {
    fn default() -> Self {
        Self {
            scroll_speed: 1200.0,
            friction: 4.2,
            velocity: 0.0,
        }
    }
}

/// Component containing offset value of the scroll container to the parent.
/// It is possible to update the field `pos_y` manually to move scrollview to desired location.
#[derive(Component, Debug, Reflect, Default)]
#[require(Node = scroll_content_node())]
pub struct ScrollableContent {
    /// Scroll container offset to the `ScrollView`.
    pub pos_y: f32,
    /// Maximum value for the scroll. It is updated automatically based on the size of the children nodes.
    pub max_scroll: f32,
}

impl ScrollableContent {
    /// Scrolls to the top of the scroll view.
    pub fn scroll_to_top(&mut self) {
        self.pos_y = 0.0;
    }
    /// Scrolls to the bottom of the scroll view.
    pub fn scroll_to_bottom(&mut self) {
        self.pos_y = -self.max_scroll;
    }

    /// Scrolls by a specified amount.
    ///
    /// # Parameters
    /// - `value`: The amount to scroll vertically. Positive values scroll down,
    ///   and negative values scroll up.
    ///
    /// Ensures the new position is clamped between the valid scroll range.
    pub fn scroll_by(&mut self, value: f32) {
        self.pos_y += value;
        self.pos_y = self.pos_y.clamp(-self.max_scroll, 0.);
    }
}

/// Creates a default scroll view node.
///
/// This function defines the visual and layout properties of a scrollable container.
pub fn scroll_view_node() -> Node {
    Node {
        overflow: Overflow::clip(),
        align_items: AlignItems::Start,
        align_self: AlignSelf::Stretch,
        flex_direction: FlexDirection::Row,
        ..default()
    }
}

/// Creates a default scroll content node.
pub fn scroll_content_node() -> Node {
    Node {
        flex_direction: bevy::ui::FlexDirection::Column,
        width: Val::Percent(100.0),
        ..default()
    }
}

/// Applies the default scroll view style to newly added `ScrollView` components.
///
/// This function updates the style of all new `ScrollView` nodes with the default
/// properties defined in `scroll_view_node`.
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
            for child in children.iter() {
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
        for child in children.iter() {
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
    mut q: Query<(&Interaction, &mut ScrollView)>,
    time: Res<Time>,
) {
    for t in touches.iter() {
        let Some(touch) = touches.get_pressed(t.id()) else {
            continue;
        };

        for (&interaction, mut view) in q.iter_mut() {
            if interaction != Interaction::Pressed {
                continue;
            }
            view.velocity = (view.velocity + touch.delta().y / time.delta_secs()) / 2.0;
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

            for child in children.iter() {
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

fn fling_update(
    mut q_view: Query<(&mut ScrollView, &Children)>,
    mut q_scroll: Query<&mut ScrollableContent>,
    time: Res<Time>,
) {
    for (mut view, children) in q_view.iter_mut() {
        let mut iter = q_scroll.iter_many_mut(children);
        while let Some(mut scroll) = iter.fetch_next() {
            if view.velocity.abs() > 16.0 {
                let (value, velocity) = calc_velocity(
                    scroll.pos_y,
                    view.velocity,
                    -view.friction,
                    time.delta_secs(),
                );
                view.velocity = velocity;
                scroll.pos_y = value.clamp(-scroll.max_scroll, 0.);
            } else {
                view.velocity = 0.0;
            }
        }
    }
}

fn calc_velocity(value: f32, velocity: f32, friction: f32, delta_t: f32) -> (f32, f32) {
    (
        value - velocity / friction + velocity / friction * (friction * delta_t).exp(),
        velocity * (delta_t * friction).exp(),
    )
}
