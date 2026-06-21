use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

#[derive(Component, Default)]
pub struct RuiScrollView {
    pub scroll_offset: Vec2,
    pub dragging_v_scroll: bool,
    pub dragging_h_scroll: bool,
}

#[derive(Component)]
pub struct RuiScrollContent;

#[derive(Component)]
pub struct RuiVerticalScrollbar;

#[derive(Component)]
pub struct RuiHorizontalScrollbar;

pub fn spawn_scrollview<'a>(
    parent_cmd: &'a mut ChildSpawnerCommands,
    modifier: impl FnOnce(&mut Node),
    children: impl FnOnce(&mut ChildSpawnerCommands),
) -> EntityCommands<'a> {
        let mut s = Node {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            min_width: Val::Px(0.0),
            min_height: Val::Px(0.0),
            overflow: Overflow::clip(),
            align_items: AlignItems::FlexStart,
            justify_content: JustifyContent::FlexStart,
            ..default()
        };
        modifier(&mut s);

        let mut cmds = parent_cmd.spawn((
            s,
            RuiScrollView::default(),
            Interaction::None,
            bevy::ui::FocusPolicy::Block,
            Pickable::default(),
            RelativeCursorPosition::default(),
        ));

        cmds.with_children(|parent| {
            parent.spawn((
                Node { display: Display::Flex, flex_direction: FlexDirection::Column, width: Val::Auto, height: Val::Auto, flex_shrink: 0.0, ..default() },
                RuiScrollContent,
            )).with_children(children);

            parent.spawn((
                Node { position_type: PositionType::Absolute, right: Val::Px(0.0), top: Val::Px(0.0), width: Val::Px(12.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, ..default() },
                Visibility::Hidden,
                RuiVerticalScrollbar,
                Interaction::None,
                Pickable::default(),
            )).with_children(|v_gutter| {
                v_gutter.spawn((
                    Node { width: Val::Px(6.0), height: Val::Percent(0.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                    ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                ));
            });

            parent.spawn((
                Node { position_type: PositionType::Absolute, left: Val::Px(0.0), bottom: Val::Px(0.0), width: Val::Percent(100.0), height: Val::Px(12.0), align_items: AlignItems::Center, ..default() },
                Visibility::Hidden,
                RuiHorizontalScrollbar,
                Interaction::None,
                Pickable::default(),
            )).with_children(|h_gutter| {
                h_gutter.spawn((
                    Node { height: Val::Px(6.0), width: Val::Percent(0.0), margin: UiRect::left(Val::Px(2.0)), ..default() },
                    ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                ));
            });
        });

        cmds
}

pub fn handle_scrollview_scroll(
    mut events: MessageReader<bevy::input::mouse::MouseWheel>,
    mut query: Query<(&RelativeCursorPosition, &mut RuiScrollView)>,
) {
    for event in events.read() {
        for (rel_pos, mut scrollview) in &mut query {
            if rel_pos.cursor_over() {
                use bevy::input::mouse::MouseScrollUnit;
                let amount = match event.unit { MouseScrollUnit::Line => 30.0, MouseScrollUnit::Pixel => 1.0 };
                scrollview.scroll_offset.y -= event.y * amount;
                scrollview.scroll_offset.x -= event.x * amount;
            }
        }
    }
}

pub fn handle_scrollview_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    mut query: Query<(&RelativeCursorPosition, &Interaction, &ComputedNode, &Children, &mut RuiScrollView)>,
    q_content: Query<&ComputedNode, With<RuiScrollContent>>,
    q_v_scroll: Query<&Interaction, With<RuiVerticalScrollbar>>,
    q_h_scroll: Query<&Interaction, With<RuiHorizontalScrollbar>>,
) {
    for (rel_pos, interaction, container_comp, children, mut scrollview) in &mut query {
        if !mouse.pressed(MouseButton::Left) { scrollview.dragging_v_scroll = false; scrollview.dragging_h_scroll = false; continue; }
        if let Some(pos) = rel_pos.normalized {
            let size = container_comp.size();
            let (lx, ly) = ((pos.x + 0.5) * size.x, (pos.y + 0.5) * size.y);
            if mouse.just_pressed(MouseButton::Left) {
                let mut clicked_v = false;
                let mut clicked_h = false;
                if children.len() > 1 {
                    if let Ok(v_int) = q_v_scroll.get(children[1]) {
                        if *v_int == Interaction::Pressed { clicked_v = true; }
                    }
                }
                if children.len() > 2 {
                    if let Ok(h_int) = q_h_scroll.get(children[2]) {
                        if *h_int == Interaction::Pressed { clicked_h = true; }
                    }
                }
                
                if clicked_v || (*interaction == Interaction::Pressed && lx >= size.x - 14.0) { scrollview.dragging_v_scroll = true; }
                else if clicked_h || (*interaction == Interaction::Pressed && ly >= size.y - 14.0) { scrollview.dragging_h_scroll = true; }
            }
            if let Some(content_entity) = children.first() {
                if let Ok(content_comp) = q_content.get(*content_entity) {
                    let c_size = content_comp.size();
                    if scrollview.dragging_v_scroll {
                        let max_y = (c_size.y - size.y).max(0.0);
                        if max_y > 0.0 {
                            let h = (size.y / c_size.y) * size.y;
                            scrollview.scroll_offset.y = ((ly - 2.0 - h / 2.0) / (size.y - h - 4.0)).clamp(0.0, 1.0) * max_y;
                        }
                    }
                    if scrollview.dragging_h_scroll {
                        let max_x = (c_size.x - size.x).max(0.0);
                        if max_x > 0.0 {
                            let w = (size.x / c_size.x) * size.x;
                            scrollview.scroll_offset.x = ((lx - 2.0 - w / 2.0) / (size.x - w - 4.0)).clamp(0.0, 1.0) * max_x;
                        }
                    }
                }
            }
        }
    }
}

pub fn update_scrollview_visuals(
    mut q_scrollview: Query<(&RelativeCursorPosition, &mut RuiScrollView, &ComputedNode, &Children)>,
    mut q_content: Query<(&mut Node, &ComputedNode), (With<RuiScrollContent>, Without<RuiVerticalScrollbar>, Without<RuiHorizontalScrollbar>)>,
    mut v_scroll_query: Query<(&mut Visibility, &Children), (With<RuiVerticalScrollbar>, Without<RuiScrollContent>, Without<RuiHorizontalScrollbar>)>,
    mut h_scroll_query: Query<(&mut Visibility, &Children), (With<RuiHorizontalScrollbar>, Without<RuiScrollContent>, Without<RuiVerticalScrollbar>)>,
    mut q_thumb_node: Query<&mut Node, (Without<RuiVerticalScrollbar>, Without<RuiHorizontalScrollbar>, Without<RuiScrollContent>)>,
) {
    for (rel_pos, mut scrollview, container_comp, children) in &mut q_scrollview {
        let size = container_comp.size();
        let show = rel_pos.cursor_over() || scrollview.dragging_v_scroll || scrollview.dragging_h_scroll;
        if let Some(content_entity) = children.first() {
            if let Ok((mut content_node, content_comp)) = q_content.get_mut(*content_entity) {
                let c_size = content_comp.size();
                let (max_x, max_y) = ((c_size.x - size.x).max(0.0), (c_size.y - size.y).max(0.0));
                scrollview.scroll_offset = scrollview.scroll_offset.clamp(Vec2::ZERO, Vec2::new(max_x, max_y));
                content_node.top = Val::Px(-scrollview.scroll_offset.y);
                content_node.left = Val::Px(-scrollview.scroll_offset.x);
                if children.len() > 1 {
                    if let Ok((mut vis, gutter_children)) = v_scroll_query.get_mut(children[1]) {
                        if max_y > 0.0 && show {
                            *vis = Visibility::Visible;
                            let h = (size.y / c_size.y) * size.y;
                            if let Some(&thumb_ent) = gutter_children.first() {
                                if let Ok(mut thumb_node) = q_thumb_node.get_mut(thumb_ent) {
                                    thumb_node.height = Val::Px(h.max(10.0));
                                    thumb_node.top = Val::Px(2.0 + (scrollview.scroll_offset.y / max_y) * (size.y - h - 4.0));
                                }
                            }
                        } else { *vis = Visibility::Hidden; }
                    }
                }
                if children.len() > 2 {
                    if let Ok((mut vis, gutter_children)) = h_scroll_query.get_mut(children[2]) {
                        if max_x > 0.0 && show {
                            *vis = Visibility::Visible;
                            let w = (size.x / c_size.x) * size.x;
                            if let Some(&thumb_ent) = gutter_children.first() {
                                if let Ok(mut thumb_node) = q_thumb_node.get_mut(thumb_ent) {
                                    thumb_node.width = Val::Px(w.max(10.0));
                                    thumb_node.left = Val::Px(2.0 + (scrollview.scroll_offset.x / max_x) * (size.x - w - 4.0));
                                }
                            }
                        } else { *vis = Visibility::Hidden; }
                    }
                }
            }
        }
    }
}