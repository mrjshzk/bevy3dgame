use bevy::{prelude::*, render::camera::Exposure, window::*};
use bevy_fps_controller::controller::*;
use bevy_inspector_egui::InspectorOptions;
use bevy_rapier3d::prelude::*;
use std::f32::consts::TAU;

use crate::{Interact, Interactable};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app
        .add_plugins(FpsControllerPlugin)
        .add_systems(Startup, spawn_player)
        .add_systems(Update, (respawn, manage_cursor))
        .add_systems(Update, cast_ray);
    }

}

const SPAWN_POINT: Vec3 = Vec3::new(0.0, 1.5, 0.0);

#[derive(Component, InspectorOptions, Default)]
pub struct TimeOfImpact(f32);

#[derive(Component, Debug)]
pub struct PlayerLogical;

#[derive(Component, Debug)]
pub struct PlayerRender;

#[derive(Component, Debug)]
pub struct ForwardHelper;

fn cast_ray(rapier_context: Res<RapierContext>, 
    forward_helper: Query<&GlobalTransform, With<ForwardHelper>>,
    player_render: Query<(&GlobalTransform, &TimeOfImpact), With<PlayerRender>>,
    player_logical: Query<Entity, With<PlayerLogical>>,
    interactables: Query<(Entity, &Interactable)>,
    mut gizmos: Gizmos,) {

        let player_collider = player_logical.get_single().unwrap();
        let player_transform = player_render.get_single().unwrap().0;
        let forward_vec = forward_helper.get_single().unwrap().translation();

        let ray_pos = player_transform.translation();
        let ray_dir = (forward_vec - player_transform.translation()).normalize_or_zero();
        let max_toi = player_render.get_single().unwrap().1;
        let solid = true;
        let filter = QueryFilter::new().exclude_collider(player_collider);
    
        if let Some((entity, _toi)) = rapier_context.cast_ray(
            ray_pos, ray_dir, max_toi.0, solid, filter
        ) {
            // The first collider hit has the entity `entity` and it hit after
            // the ray travelled a distance equal to `ray_dir * toi`.
            // let hit_point = ray_pos + ray_dir * toi;

            
        if let Ok((_, comp)) = interactables.get(entity) { 
            comp.interact();
        }
            
            


        }

        gizmos.ray(ray_pos, ray_dir, Color::GREEN);
}


// player stuff
fn spawn_player(mut commands: Commands) {
    // Note that we have two entities for the player
    // One is a "logical" player that handles the physics computation and collision
    // The other is a "render" player that is what is displayed to the user
    // This distinction is useful for later on if you want to add multiplayer,
    // where often time these two ideas are not exactly synced up
    let logical_entity = commands
        .spawn((
            Collider::capsule(Vec3::Y * 0.5, Vec3::Y * 1.5, 0.5),
            Friction {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            Restitution {
                coefficient: 0.0,
                combine_rule: CoefficientCombineRule::Min,
            },
            ActiveEvents::COLLISION_EVENTS,
            Velocity::zero(),
            RigidBody::Dynamic,
            Sleeping::disabled(),
            LockedAxes::ROTATION_LOCKED,
            AdditionalMassProperties::Mass(1.0),
            GravityScale(0.0),
            Ccd { enabled: true }, // Prevent clipping when going fast
            TransformBundle::from_transform(Transform::from_translation(SPAWN_POINT)),
            LogicalPlayer,
            FpsControllerInput {
                jump: false,
                pitch: -TAU / 12.0,
                yaw: TAU * 5.0 / 8.0,
                ..default()
            },
            FpsController {
                air_acceleration: 80.0,
                ..default()
            },
            
        ))
        .insert(CameraConfig {
            height_offset: 0.0,
            radius_scale: 0.75,
        })
        .insert(Name::new("Player Logical"))
        .insert(PlayerLogical)
        .id();

        commands.spawn((
        Camera3dBundle {
            projection: Projection::Perspective(PerspectiveProjection {
                fov: TAU / 5.0,
                ..default()
            }),
            exposure: Exposure::SUNLIGHT,
            ..default()
        },
        TimeOfImpact(2.5),
        RenderPlayer { logical_entity },
        Name::new("Player Render"),
        PlayerRender,
    )).with_children(
        |parent| {
            parent.spawn((
                Name::new("ForwardHelper"),
                ForwardHelper,
                TransformBundle {
                    local: Transform::from_xyz(0.0, 0.0, -1.0),
                    ..default()
                }
            ));
        }
    ).with_children(|parent| {
        parent.spawn(PointLightBundle::default());
    });

    

    
    
}

fn respawn(mut query: Query<(&mut Transform, &mut Velocity)>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y > -50.0 {
            continue;
        }

        velocity.linvel = Vec3::ZERO;
        transform.translation = SPAWN_POINT;
    }
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