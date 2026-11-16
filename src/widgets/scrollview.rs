//! Scrollable container widget with automatic vertical and horizontal scrollbars.

use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

/// Component storing the pixel scroll offset and scrollbar drag state of a scroll view.
#[derive(Component, Default)]
pub struct RuiScrollView {
    /// Current scroll offset `(x, y)` in pixels.
    pub scroll_offset: Vec2,
    pub dragging_v_scroll: bool,
    pub dragging_h_scroll: bool,
}

/// Marker component for the inner content container inside a [`RuiScrollView`].
#[derive(Component)] pub struct RuiScrollContent;
/// Marker component for the vertical scrollbar track inside a [`RuiScrollView`].
#[derive(Component)] pub struct RuiVerticalScrollbar;
/// Marker component for the horizontal scrollbar track inside a [`RuiScrollView`].
#[derive(Component)] pub struct RuiHorizontalScrollbar;
/// Marker component for the vertical draggable scrollbar thumb.
#[derive(Component)] pub struct RuiScrollThumbV;
/// Marker component for the horizontal draggable scrollbar thumb.
#[derive(Component)] pub struct RuiScrollThumbH;

/// Spawns a scrollable container (`Overflow::clip()`) with mouse wheel support and draggable vertical/horizontal scrollbars.
///
/// Also available via [`crate::widgets::RuiBuilderExt::scrollview`].
pub fn spawn_scrollview<'w>(
    parent_cmd: &'w mut ChildSpawnerCommands<'w>,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'w> {
    let mut s = Node {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0), height: Val::Percent(100.0),
        min_width: Val::Px(0.0), min_height: Val::Px(0.0),
        overflow: Overflow::clip(),
        overflow_clip_margin: bevy::ui::OverflowClipMargin::content_box(),
        align_items: AlignItems::FlexStart,
        justify_content: JustifyContent::FlexStart,
        position_type: PositionType::Relative,
        ..default()
    };
    modifier(&mut s);

    let mut cmds = parent_cmd.spawn((
        s,
        RuiScrollView::default(),
        bevy::ui_widgets::Button,
        Pickable::default(),
        RelativeCursorPosition::default(),
    ));

    let scrollview_id = cmds.id();

        let mut content_id = Entity::PLACEHOLDER;

    cmds.with_children(|parent| {
        content_id = parent.spawn((
            Node { display: Display::Flex, flex_direction: FlexDirection::Column, width: Val::Auto, height: Val::Auto, flex_shrink: 0.0, ..default() },
            RuiScrollContent,
        )).with_children(children).id();

        // ======================================================================
        // BARRA VERTICAL (Gutter & Thumb Reactivos)
        // ======================================================================
        let mut v_thumb_cmds = parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                right: Val::Px(2.0),
                top: Val::Px(2.0),
                bottom: Val::Px(2.0),
                width: Val::Px(10.0),
                justify_content: JustifyContent::Center,
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 0.45)),
            crate::theme::RuiThemeElement::ScrollbarTrack,
            Visibility::Hidden,
            RuiVerticalScrollbar,
            Pickable { should_block_lower: false, is_hoverable: true },
        ));
        
        v_thumb_cmds.with_children(|v_gutter| {
            let mut thumb_v = v_gutter.spawn((
                Node {
                    width: Val::Px(8.0),
                    height: Val::Percent(0.0),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                ImageNode::default(),
                BackgroundColor(Color::srgba(0.9, 0.9, 0.95, 0.85)),
                crate::theme::RuiThemeElement::ScrollbarThumb,
                RuiScrollThumbV,
                Pickable { should_block_lower: true, is_hoverable: true },
            ));
            
            thumb_v.observe(move |on_drag: On<PointerDrag>, mut q_scroll: Query<(&mut RuiScrollView, &ComputedNode)>, q_content: Query<&ComputedNode, With<RuiScrollContent>>| {
                if let Ok((mut scroll, comp)) = q_scroll.get_mut(scrollview_id) {
                    if let Ok(content_comp) = q_content.get(content_id) {
                        let inv = comp.inverse_scale_factor();
                        let size_y = comp.size().y * inv;
                        let content_y = content_comp.size().y * inv;
                        let max_y = (content_y - size_y).max(0.0);
                        if max_y > 0.0 {
                            let h = (size_y / content_y) * size_y;
                            let track_size = (size_y - h).max(1.0);
                            let scale_factor = max_y / track_size;
                            scroll.scroll_offset.y = (scroll.scroll_offset.y + on_drag.delta.y * scale_factor).clamp(0.0, max_y);
                        }
                    }
                }
            });
        });

        // ======================================================================
        // BARRA HORIZONTAL (Gutter & Thumb Reactivos)
        // ======================================================================
        let mut h_thumb_cmds = parent.spawn((
            Node {
                position_type: PositionType::Absolute,
                left: Val::Px(2.0),
                right: Val::Px(2.0),
                bottom: Val::Px(2.0),
                height: Val::Px(10.0),
                align_items: AlignItems::Center,
                border_radius: BorderRadius::all(Val::Px(5.0)),
                ..default()
            },
            ImageNode::default(),
            BackgroundColor(Color::srgba(0.05, 0.05, 0.08, 0.45)),
            crate::theme::RuiThemeElement::ScrollbarTrack,
            Visibility::Hidden,
            RuiHorizontalScrollbar,
            Pickable { should_block_lower: false, is_hoverable: true },
        ));
        
        h_thumb_cmds.with_children(|h_gutter| {
            let mut thumb_h = h_gutter.spawn((
                Node {
                    height: Val::Px(8.0),
                    width: Val::Percent(0.0),
                    border_radius: BorderRadius::all(Val::Px(4.0)),
                    ..default()
                },
                ImageNode::default(),
                BackgroundColor(Color::srgba(0.9, 0.9, 0.95, 0.85)),
                crate::theme::RuiThemeElement::ScrollbarThumb,
                RuiScrollThumbH,
                Pickable { should_block_lower: true, is_hoverable: true },
            ));
            
            thumb_h.observe(move |on_drag: On<PointerDrag>, mut q_scroll: Query<(&mut RuiScrollView, &ComputedNode)>, q_content: Query<&ComputedNode, With<RuiScrollContent>>| {
                if let Ok((mut scroll, comp)) = q_scroll.get_mut(scrollview_id) {
                    if let Ok(content_comp) = q_content.get(content_id) {
                        let inv = comp.inverse_scale_factor();
                        let size_x = comp.size().x * inv;
                        let content_x = content_comp.size().x * inv;
                        let max_x = (content_x - size_x).max(0.0);
                        if max_x > 0.0 {
                            let w = (size_x / content_x) * size_x;
                            let track_size = (size_x - w).max(1.0);
                            let scale_factor = max_x / track_size;
                            scroll.scroll_offset.x = (scroll.scroll_offset.x + on_drag.delta.x * scale_factor).clamp(0.0, max_x);
                        }
                    }
                }
            });
        });
    });

    cmds
}
use bevy::input::mouse::MouseWheel;

/// System that scrolls the hovered [`RuiScrollView`] in response to [`MouseWheel`] events.
pub fn handle_scrollview_scroll(
    mut events: MessageReader<MouseWheel>,
    mut query: Query<(&RelativeCursorPosition, Option<&bevy::picking::hover::Hovered>, &mut RuiScrollView)>,
) {
    for event in events.read() {
        for (rel_pos, hovered, mut scrollview) in &mut query {
            if rel_pos.cursor_over() || hovered.is_some_and(|h| h.0) {
                use bevy::input::mouse::MouseScrollUnit;
                let amount = match event.unit { MouseScrollUnit::Line => 30.0, MouseScrollUnit::Pixel => 1.0 };
                scrollview.scroll_offset.y -= event.y * amount;
                scrollview.scroll_offset.x -= event.x * amount;
            }
        }
    }
}

/// Placeholder system kept for backwards compatibility.
pub fn handle_scrollview_clicks() {}

/// System that clamps [`RuiScrollView::scroll_offset`], shifts [`RuiScrollContent`], and updates scrollbar thumb sizes/positions.
pub fn update_scrollview_visuals(
    mut q_scrollview: Query<(&Node, &mut RuiScrollView, &ComputedNode, &Children)>,
    mut q_content: Query<(&mut Node, &ComputedNode), (With<RuiScrollContent>, Without<RuiVerticalScrollbar>, Without<RuiHorizontalScrollbar>, Without<RuiScrollView>)>,
    mut v_scroll_query: Query<(&mut Node, &mut Visibility, &Children), (With<RuiVerticalScrollbar>, Without<RuiScrollContent>, Without<RuiHorizontalScrollbar>, Without<RuiScrollView>)>,
    mut h_scroll_query: Query<(&mut Node, &mut Visibility, &Children), (With<RuiHorizontalScrollbar>, Without<RuiScrollContent>, Without<RuiVerticalScrollbar>, Without<RuiScrollView>)>,
    mut q_thumb_node: Query<&mut Node, (Without<RuiVerticalScrollbar>, Without<RuiHorizontalScrollbar>, Without<RuiScrollContent>, Without<RuiScrollView>)>,
) {
    for (scroll_node, mut scrollview, container_comp, children) in &mut q_scrollview {
        let inv = container_comp.inverse_scale_factor();
        let size = container_comp.size() * inv;
        let pad_top = match scroll_node.padding.top { Val::Px(v) => v, _ => 0.0 };
        let pad_bottom = match scroll_node.padding.bottom { Val::Px(v) => v, _ => 0.0 };
        let pad_left = match scroll_node.padding.left { Val::Px(v) => v, _ => 0.0 };
        let pad_right = match scroll_node.padding.right { Val::Px(v) => v, _ => 0.0 };
        let pad_y = pad_top + pad_bottom;
        let pad_x = pad_left + pad_right;
        let inner_h = (size.y - pad_y).max(1.0);
        let inner_w = (size.x - pad_x).max(1.0);

        if let Some(content_entity) = children.first() {
            if let Ok((mut content_node, content_comp)) = q_content.get_mut(*content_entity) {
                let c_size = content_comp.size() * inv;
                let (max_x, max_y) = ((c_size.x - inner_w).max(0.0), (c_size.y - inner_h).max(0.0));
                scrollview.scroll_offset = scrollview.scroll_offset.clamp(Vec2::ZERO, Vec2::new(max_x, max_y));
                content_node.top = Val::Px(-scrollview.scroll_offset.y);
                content_node.left = Val::Px(-scrollview.scroll_offset.x);
                if children.len() > 1 {
                    if let Ok((mut gutter_node, mut vis, gutter_children)) = v_scroll_query.get_mut(children[1]) {
                        if max_y > 0.0 {
                            *vis = Visibility::Visible;
                            gutter_node.right = Val::Px(pad_right + 2.0);
                            gutter_node.top = Val::Px(pad_top + 2.0);
                            gutter_node.bottom = Val::Px(pad_bottom + 2.0);
                            let track_h = (inner_h - 4.0).max(10.0);
                            let h = ((inner_h / c_size.y.max(1.0)) * track_h).clamp(18.0, track_h);
                            if let Some(&thumb_ent) = gutter_children.first() {
                                if let Ok(mut thumb_node) = q_thumb_node.get_mut(thumb_ent) {
                                    thumb_node.height = Val::Px(h);
                                    thumb_node.top = Val::Px((scrollview.scroll_offset.y / max_y) * (track_h - h));
                                }
                            }
                        } else { *vis = Visibility::Hidden; }
                    }
                }
                if children.len() > 2 {
                    if let Ok((mut gutter_node, mut vis, gutter_children)) = h_scroll_query.get_mut(children[2]) {
                        if max_x > 0.0 {
                            *vis = Visibility::Visible;
                            gutter_node.left = Val::Px(pad_left + 2.0);
                            gutter_node.right = Val::Px(pad_right + 2.0);
                            gutter_node.bottom = Val::Px(pad_bottom + 2.0);
                            let track_w = (inner_w - 4.0).max(10.0);
                            let w = ((inner_w / c_size.x.max(1.0)) * track_w).clamp(18.0, track_w);
                            if let Some(&thumb_ent) = gutter_children.first() {
                                if let Ok(mut thumb_node) = q_thumb_node.get_mut(thumb_ent) {
                                    thumb_node.width = Val::Px(w);
                                    thumb_node.left = Val::Px((scrollview.scroll_offset.x / max_x) * (track_w - w));
                                }
                            }
                        } else { *vis = Visibility::Hidden; }
                    }
                }
            }
        }
    }
}
