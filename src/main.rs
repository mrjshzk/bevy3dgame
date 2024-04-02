
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod player;
mod gtlf_helper;
pub mod camera;
pub mod collider_generator;

use gtlf_helper::MakeMesh;
use gtlf_helper::GLTFHelperPlugin;
use player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(AmbientLight {
            color: Color::DARK_GRAY,
            brightness: 10000.0,
        })
        .insert_resource(RapierConfiguration::default())
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins((PlayerPlugin, GLTFHelperPlugin))
        .add_systems(Startup, spawn_stuff)
        .run();
}

#[derive(Component, Debug)]
pub struct Sword;

#[derive(Component, Debug)]
pub struct Interactable;

pub trait Interact {
    fn interact(&self);
}

impl Interact for Interactable {
    fn interact(&self) {
        println!("Called")
    }
}

fn spawn_stuff(
    mut commands: Commands,
    ) {
        commands.spawn((
            Sword,
            Name::new("Cemetery"),
            MakeMesh { path: "Cemetery.glb".to_owned(), ..default()},
            RigidBody::Fixed,
            Interactable,
        ));
    }


