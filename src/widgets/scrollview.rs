use bevy::prelude::*;
use bevy::ui::RelativeCursorPosition;

#[derive(Component, Default)]
pub struct RuiScrollView {
    pub scroll_offset: Vec2,
    pub dragging_v_scroll: bool,
    pub dragging_h_scroll: bool,
}

#[derive(Component)] pub struct RuiScrollContent;
#[derive(Component)] pub struct RuiVerticalScrollbar;
#[derive(Component)] pub struct RuiHorizontalScrollbar;
#[derive(Component)] pub struct RuiScrollThumbV;
#[derive(Component)] pub struct RuiScrollThumbH;

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
            Node { position_type: PositionType::Absolute, right: Val::Px(0.0), top: Val::Px(0.0), height: Val::Percent(100.0), width: Val::Px(12.0), justify_content: JustifyContent::FlexEnd, ..default() },
            BackgroundColor(Color::NONE),
            crate::theme::RuiThemeElement::ScrollbarTrack,
            Visibility::Hidden,
            RuiVerticalScrollbar,
            Pickable { should_block_lower: false, is_hoverable: true },
        ));
        
        v_thumb_cmds.with_children(|v_gutter| {
            let mut thumb_v = v_gutter.spawn((
                Node { width: Val::Px(6.0), height: Val::Percent(0.0), margin: UiRect::top(Val::Px(2.0)), ..default() },
                BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                crate::theme::RuiThemeElement::ScrollbarThumb,
                RuiScrollThumbV,
                Pickable { should_block_lower: true, is_hoverable: true },
            ));
            
            // CORRECCIÓN DEFINITIVA: El observador escucha el arrastre directamente en el Thumb que recibe el impacto
            thumb_v.observe(move |on_drag: On<PointerDrag>, mut q_scroll: Query<(&mut RuiScrollView, &ComputedNode)>, q_content: Query<&ComputedNode, With<RuiScrollContent>>| {
                if let Ok((mut scroll, comp)) = q_scroll.get_mut(scrollview_id) {
                    if let Ok(content_comp) = q_content.get(content_id) {
                        let size_y = comp.size().y;
                        let content_y = content_comp.size().y;
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
            Node { position_type: PositionType::Absolute, left: Val::Px(0.0), bottom: Val::Px(0.0), width: Val::Percent(100.0), height: Val::Px(12.0), align_items: AlignItems::Center, ..default() },
            BackgroundColor(Color::NONE),
            crate::theme::RuiThemeElement::ScrollbarTrack,
            Visibility::Hidden,
            RuiHorizontalScrollbar,
            Pickable { should_block_lower: false, is_hoverable: true },
        ));
        
        h_thumb_cmds.with_children(|h_gutter| {
            let mut thumb_h = h_gutter.spawn((
                Node { height: Val::Px(6.0), width: Val::Percent(0.0), margin: UiRect::left(Val::Px(2.0)), ..default() },
                BackgroundColor(Color::srgba(0.8, 0.8, 0.8, 0.5)),
                crate::theme::RuiThemeElement::ScrollbarThumb,
                RuiScrollThumbH,
                Pickable { should_block_lower: true, is_hoverable: true },
            ));
            
            // CORRECCIÓN DEFINITIVA: El observador escucha el arrastre directamente en el Thumb que recibe el impacto
            thumb_h.observe(move |on_drag: On<PointerDrag>, mut q_scroll: Query<(&mut RuiScrollView, &ComputedNode)>, q_content: Query<&ComputedNode, With<RuiScrollContent>>| {
                if let Ok((mut scroll, comp)) = q_scroll.get_mut(scrollview_id) {
                    if let Ok(content_comp) = q_content.get(content_id) {
                        let size_x = comp.size().x;
                        let content_x = content_comp.size().x;
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

pub fn handle_scrollview_scroll(
    mut events: MessageReader<MouseWheel>,
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

pub fn handle_scrollview_clicks() {}

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
