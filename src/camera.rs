use bevy::ecs::schedule::ShouldRun;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::prelude::*;
use bevy_inspector_egui::bevy_egui::EguiContext;

#[derive(Default)]
pub struct CameraPlugin {
    need_update: bool,
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, RunCriteriaLabel)]
pub struct CameraRunCriteria;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum CameraSystem {
    PanOrbit,
    Adjust,
}

fn plugin_enabled(
    mut egui_context: ResMut<EguiContext>,
) -> ShouldRun {
    // don't adjust camera if the mouse pointer in over an egui window
    let ctx = egui_context.ctx_mut();
    let pointer_over_area = ctx.is_pointer_over_area();
    let using_pointer = ctx.is_using_pointer();
    let wants_pointer = ctx.wants_pointer_input();
    if wants_pointer || pointer_over_area || using_pointer {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set_to_stage(
            CoreStage::Update,
            SystemSet::new()
                .with_run_criteria(plugin_enabled.label(CameraRunCriteria))
                .with_system(pan_orbit_camera.label(CameraSystem::PanOrbit))
                .with_system(
                    adjust
                        .label(CameraSystem::Adjust)
                        .after(CameraSystem::PanOrbit),
                )
                .with_system(center_selection)
        );
    }
}

fn center_selection(
    selection: Query<(&Transform, &bevy_mod_picking::Selection)>,
    mut camera: Query<(&mut PanOrbitCamera, &Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !selection.iter().any(|(_, selection)| selection.selected()) {
        return;
    }

    if keyboard_input.just_released(KeyCode::Period) {
        let mut total = Vec3::ZERO;
        let mut point_count = 0;
        for (transform, selection) in &selection {
            if selection.selected() {
                total += transform.translation;
                point_count += 1;
            }
        }
        let center = total / point_count as f32;
        let (mut camera, camera_transform) = camera.single_mut();
        camera.radius = (camera_transform.translation - center).length();
        camera.focus = center;
    }
}

/// Tags an entity as capable of panning and orbiting.
#[derive(Component)]
pub struct PanOrbitCamera {
    /// The "focus point" to orbit around. It is automatically updated when panning the camera
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.0,
            upside_down: false,
        }
    }
}

fn pan_orbit_camera(
    windows: Res<Windows>,
    mut ev_motion: EventReader<MouseMotion>,
    mut ev_scroll: EventReader<MouseWheel>,
    input_mouse: Res<Input<MouseButton>>,
    mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    // change input mapping for orbit and panning here
    let orbit_button = MouseButton::Middle;
    let pan_button = MouseButton::Middle;
    let pan_key_left = KeyCode::LShift;
    let pan_key_right = KeyCode::RShift;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.0;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button)
        && !(keyboard_input.pressed(pan_key_right) || keyboard_input.pressed(pan_key_left))
    {
        for ev in ev_motion.iter() {
            rotation_move += ev.delta;
        }
    } else if input_mouse.pressed(pan_button)
        && (keyboard_input.pressed(pan_key_right) || keyboard_input.pressed(pan_key_left))
    {
        // Pan only if we're not rotating at the moment
        for ev in ev_motion.iter() {
            pan += ev.delta;
        }
    }
    for ev in ev_scroll.iter() {
        scroll += ev.y;
    }
    if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
        orbit_button_changed = true;
    }

    for (mut pan_orbit, mut transform, projection) in query.iter_mut() {
        if orbit_button_changed {
            // only check for upside down when orbiting started or ended this frame
            // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
            let up = transform.rotation * Vec3::Y;
            pan_orbit.upside_down = up.y <= 0.0;
        }

        let mut any = false;
        if rotation_move.length_squared() > 0.0 {
            any = true;
            let window = get_primary_window_size(&windows);
            let delta_x = {
                let delta = rotation_move.x / window.x * std::f32::consts::PI * 2.0;
                if pan_orbit.upside_down {
                    -delta
                } else {
                    delta
                }
            };
            let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
            let yaw = Quat::from_rotation_y(-delta_x);
            let pitch = Quat::from_rotation_x(-delta_y);
            transform.rotation = yaw * transform.rotation; // rotate around global y axis
            transform.rotation = transform.rotation * pitch; // rotate around local x axis
        } else if pan.length_squared() > 0.0 {
            any = true;
            // make panning distance independent of resolution and FOV,
            let window = get_primary_window_size(&windows);
            if let Projection::Perspective(projection) = projection {
                pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
            }
            // translate by local axes
            let right = transform.rotation * Vec3::X * -pan.x;
            let up = transform.rotation * Vec3::Y * pan.y;
            // make panning proportional to distance away from focus point
            let translation = (right + up) * pan_orbit.radius;
            pan_orbit.focus += translation;
        } else if scroll.abs() > 0.0 {
            any = true;
            pan_orbit.radius -= scroll * pan_orbit.radius * 0.002;
            // dont allow zoom to reach zero or you get stuck
            pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
        }

        if any {
            // emulating parent/child to make the yaw/y-axis rotation behave like a turntable
            // parent = x and y rotation
            // child = z-offset
            let rot_matrix = Mat3::from_quat(transform.rotation);
            transform.translation =
                pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
        }
    }
}

fn adjust(mut query: Query<(&mut PanOrbitCamera, &mut Transform)>) {
    for (pan_orbit, mut transform) in query.iter_mut() {
        let rot_matrix = Mat3::from_quat(transform.rotation);
        transform.translation =
            pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
    }
}

fn get_primary_window_size(windows: &Res<Windows>) -> Vec2 {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    window
}
