use std::f32::consts::PI;

use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_inspector_egui::{inspector_options::ReflectInspectorOptions, InspectorOptions};

use crate::player::Player;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<CamConfig>();
        app.add_systems(FixedUpdate, camera_look);
    }
}

#[derive(Component, Reflect, InspectorOptions)]
#[reflect(Component, InspectorOptions)] // will register ReflectComponent, ReflectSerialize, ReflectDeserialize
pub struct CamConfig {
    yaw: f32,
    pitch: f32,
    invert_camera: bool,
    cam_sensitivity: f32
}

impl Default for CamConfig {
    fn default() -> Self {
        Self { yaw: 0.0, 
            pitch: 0.0, 
            invert_camera: false, 
            cam_sensitivity: 0.5 }
    }
}

fn camera_look(
    mut camera: Query<(&mut Transform, &mut CamConfig), (With<Camera>, Without<Player>)>,
    mut player: Query<&mut Transform, (With<Player>, Without<Camera>)>,
    mut mouse_input: EventReader<MouseMotion>) {

    
    for event in mouse_input.read() {
        let cursor_delta = event.delta;
        
        if let Ok((mut transform, mut cam_config)) = camera.get_single_mut() {
            
            let invert = bool_to_int(cam_config.invert_camera);
            cam_config.yaw += (cursor_delta.x / 180.0) * invert * cam_config.cam_sensitivity;
            cam_config.pitch += (cursor_delta.y / 180.0) * invert * cam_config.cam_sensitivity;
            cam_config.pitch = cam_config.pitch.clamp(deg2rad(-85.0), deg2rad(80.0));

            // apply Y rotation to camera
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, 0.0, cam_config.pitch);

            // apply X rotation to player (camera's parent)
            let mut player_transform = player.get_single_mut().unwrap();
            player_transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, cam_config.yaw, 0.0);
        }
        
    }
}

fn bool_to_int(b: bool) -> f32 {
    match b {
        true => return 1.0,
        false => return -1.0,
    }
}

fn rad2deg(rad: f32) -> f32 {
    rad * (180.0 / PI)
}

fn deg2rad(deg: f32) -> f32 {
    deg * (PI / 180.0)
}