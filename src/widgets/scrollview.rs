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
                Node { position_type: PositionType::Absolute, right: Val::Px(2.0), top: Val::Px(2.0), width: Val::Px(6.0), height: Val::Percent(0.0), ..default() },
                ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                Visibility::Hidden,
                RuiVerticalScrollbar,
                Pickable::IGNORE,
            ));

            parent.spawn((
                Node { position_type: PositionType::Absolute, left: Val::Px(2.0), bottom: Val::Px(2.0), width: Val::Percent(0.0), height: Val::Px(6.0), ..default() },
                ImageNode::solid_color(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                Visibility::Hidden,
                RuiHorizontalScrollbar,
                Pickable::IGNORE,
            ));
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
) {
    for (rel_pos, interaction, container_comp, children, mut scrollview) in &mut query {
        if !mouse.pressed(MouseButton::Left) { scrollview.dragging_v_scroll = false; scrollview.dragging_h_scroll = false; continue; }
        if let Some(pos) = rel_pos.normalized {
            let size = container_comp.size();
            let (lx, ly) = ((pos.x + 0.5) * size.x, (pos.y + 0.5) * size.y);
            if mouse.just_pressed(MouseButton::Left) && *interaction == Interaction::Pressed {
                if lx >= size.x - 14.0 { scrollview.dragging_v_scroll = true; }
                else if ly >= size.y - 14.0 { scrollview.dragging_h_scroll = true; }
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
    mut v_scroll_query: Query<(&mut Node, &mut Visibility), (With<RuiVerticalScrollbar>, Without<RuiScrollContent>, Without<RuiHorizontalScrollbar>)>,
    mut h_scroll_query: Query<(&mut Node, &mut Visibility), (With<RuiHorizontalScrollbar>, Without<RuiScrollContent>, Without<RuiVerticalScrollbar>)>,
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
                    if let Ok((mut node, mut vis)) = v_scroll_query.get_mut(children[1]) {
                        if max_y > 0.0 && show { *vis = Visibility::Visible; let h = (size.y / c_size.y) * size.y; node.height = Val::Px(h.max(10.0)); node.top = Val::Px(2.0 + (scrollview.scroll_offset.y / max_y) * (size.y - h - 4.0)); } else { *vis = Visibility::Hidden; }
                    }
                }
                if children.len() > 2 {
                    if let Ok((mut node, mut vis)) = h_scroll_query.get_mut(children[2]) {
                        if max_x > 0.0 && show { *vis = Visibility::Visible; let w = (size.x / c_size.x) * size.x; node.width = Val::Px(w.max(10.0)); node.left = Val::Px(2.0 + (scrollview.scroll_offset.x / max_x) * (size.x - w - 4.0)); } else { *vis = Visibility::Hidden; }
                    }
                }
            }
        }
    }
}