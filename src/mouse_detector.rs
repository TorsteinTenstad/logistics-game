use bevy::{prelude::*, window::PrimaryWindow};
#[derive(PartialEq, Debug)]
pub enum MouseDetectorState {
    None,
    Hover,
    Press,
}

#[derive(Component)]
pub struct MouseDetector {
    pub detector_state: MouseDetectorState,
    pub x_radius: f32,
    pub y_radius: f32,
}

impl MouseDetector {
    pub fn new(x_radius: f32, y_radius: f32) -> Self {
        Self {
            detector_state: MouseDetectorState::None,
            x_radius: x_radius,
            y_radius: y_radius,
        }
    }
}

pub fn update_mouse_detector(
    mouse_buttons: Res<Input<MouseButton>>,
    q_windows: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<(&mut MouseDetector, &GlobalTransform)>,
) {
    let (camera, camera_global_transform) = camera_q.single();

    let left_button_down = mouse_buttons.pressed(MouseButton::Left);

    if let Some(mouse_pos) = q_windows.single().cursor_position().and_then(|cursor| {
        camera
            .viewport_to_world(camera_global_transform, cursor)
            .map(|ray| ray.origin)
    }) {
        for (mut mouse_detector, &global_transform) in query.iter_mut() {
            let relative_mouse_pos = global_transform
                .compute_transform()
                .compute_affine()
                .inverse()
                .transform_point3(mouse_pos);
            if relative_mouse_pos.x.abs() < mouse_detector.x_radius
                && relative_mouse_pos.y.abs() < mouse_detector.y_radius
            {
                if left_button_down {
                    if mouse_detector.detector_state != MouseDetectorState::Press {
                        mouse_detector.detector_state = MouseDetectorState::Press
                    }
                } else {
                    if mouse_detector.detector_state != MouseDetectorState::Hover {
                        mouse_detector.detector_state = MouseDetectorState::Hover
                    }
                }
            } else {
                if mouse_detector.detector_state != MouseDetectorState::None {
                    mouse_detector.detector_state = MouseDetectorState::None
                }
            }
        }
    } else {
        for (mut mouse_detector, _) in query.iter_mut() {
            if mouse_detector.detector_state != MouseDetectorState::None {
                mouse_detector.detector_state = MouseDetectorState::None
            }
        }
    }
}
