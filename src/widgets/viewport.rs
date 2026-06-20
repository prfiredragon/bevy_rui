use bevy::prelude::*;
use bevy::camera::Viewport as BevyViewport;

#[derive(Component)]
pub struct RuiViewportCamera;

#[derive(Component)]
pub struct RuiViewport {
    pub camera_entity: Entity,
}

pub fn spawn_viewport<'a>(
    parent: &'a mut ChildSpawnerCommands,
    camera_entity: Entity,
    modifier: impl FnOnce(&mut Node),
) -> EntityCommands<'a> {
    let mut s = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        overflow: Overflow::clip(),
        ..default()
    };
    modifier(&mut s);

    parent.spawn((s, Pickable::IGNORE, RuiViewport { camera_entity }))
}

pub fn update_rui_viewports(
    q_viewports: Query<(&bevy::ui::UiGlobalTransform, &ComputedNode, &RuiViewport, &InheritedVisibility)>,
    mut q_cameras: Query<(Entity, &mut Camera), With<RuiViewportCamera>>,
    q_windows: Query<&Window>,
) {
    // Si la ventana principal no existe, salimos
    if q_windows.is_empty() {
        return;
    }

    // Guardaremos el viewport que debe tener cada cámara
    let mut desired_viewports = std::collections::HashMap::new();

    for (transform, node, viewport, inherited_vis) in &q_viewports {
        // Si el nodo no es visible en la jerarquía (ej. Visibility::Hidden)
        if !inherited_vis.get() {
            continue;
        }

        let size = node.size();
        // Si el tamaño es cero o inválido (ej. pestaña oculta con display: None), lo ignoramos
        if size.x <= 0.0 || size.y <= 0.0 {
            continue;
        }

        // Obtenemos la posición de la UI usando la nueva transformación en Bevy 0.18
        let pos = transform.to_scale_angle_translation().2;
        
        // En Bevy 0.15+, UiGlobalTransform y ComputedNode.size() están en píxeles FÍSICOS.
        // Además, UiGlobalTransform representa el centro del nodo.
        let left = pos.x - size.x / 2.0;
        let top = pos.y - size.y / 2.0;
        
        let physical_pos = UVec2::new(
            left.max(0.0) as u32,
            top.max(0.0) as u32,
        );

        let physical_size = UVec2::new(
            size.x.max(0.0) as u32, 
            size.y.max(0.0) as u32
        );

        // Debug prints to trace the viewport values
        /* eprintln!("UI Viewport: node size (physical)={:?}, pos (center)={:?} -> rect=({}, {}, {}, {})", 
            size, pos, left, top, physical_size.x, physical_size.y); */

        // Agregamos a los viewports deseados
        desired_viewports.insert(viewport.camera_entity, BevyViewport {
            physical_position: physical_pos,
            physical_size,
            ..default()
        });
    }

    // Ahora aplicamos los cambios a las cámaras evitando modificaciones innecesarias
    for (entity, mut camera) in &mut q_cameras {
        if let Some(viewport) = desired_viewports.remove(&entity) {
            // La cámara debe estar activa
            if !camera.is_active {
                camera.is_active = true;
            }
            
            // Verificamos si el viewport realmente cambió antes de mutar
            let needs_update = match &camera.viewport {
                Some(v) => v.physical_position != viewport.physical_position || v.physical_size != viewport.physical_size,
                None => true,
            };
            
            if needs_update {
                camera.viewport = Some(viewport);
            }
        } else {
            // Ningún panel de UI reclama esta cámara, así que la apagamos
            if camera.is_active {
                camera.is_active = false;
            }
        }
    }
}