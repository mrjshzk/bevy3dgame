use bevy::{prelude::*, render::camera::Exposure, window::*};
use bevy_fps_controller::controller::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

use crate::camera::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app
        .add_plugins((FpsControllerPlugin, CameraPlugin))
        .add_systems(Startup, setup_player)
        .add_systems(Update, manage_cursor)
        //.add_systems(Update, cast_ray)
        ;
    }

}

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.5, 0.0);

#[derive(Component, Debug)]
pub struct PlayerLogical;

#[derive(Component, Debug)]
pub struct PlayerRender;

#[derive(Component, Debug)]
pub struct Player;

#[derive(Component, Debug)]
pub struct ForwardHelper;

fn setup_player(mut commands: Commands) {
    commands.spawn((
        Player,
        Name::new("Player"),
        RigidBody::KinematicVelocityBased,
        Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.35),
        KinematicCharacterController::default(),

        TransformBundle {
            local: Transform::from_translation(SPAWN_POINT),
            ..default()
        }
    ))
    .with_children(|p| {
        p.spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
                projection: Projection::Perspective(PerspectiveProjection {
                    fov: TAU / 5.0,
                    ..default()
                }),
                exposure: Exposure::INDOOR,
                ..default()
            },
            CamConfig::default(),
        ));
    }).with_children(|p| {
        p.spawn((
            ForwardHelper,
            TransformBundle {
                local: Transform::from_xyz(0.0, 0.0, -1.0),
                ..default()
            }
        ));
    });
    
}

fn manage_cursor(
    key: Res<ButtonInput<KeyCode>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    let mut window = window_query.single_mut();
    
    if key.just_pressed(KeyCode::Escape) {

        if window.cursor.grab_mode == CursorGrabMode::Locked {
            window.cursor.grab_mode = CursorGrabMode::None;
            window.cursor.visible = true;
            for mut controller in &mut controller_query {
                controller.enable_input = false;
            }
        } else {
            window.cursor.grab_mode = CursorGrabMode::Locked;
            window.cursor.visible = false;
            for mut controller in &mut controller_query {
                controller.enable_input = true;
    }
        }

       
    }
}