use bevy::prelude::*;

pub struct GLTFHelperPlugin;

use crate::collider_generator::ColliderGeneratorPlugin;
use crate::collider_generator::MakeMeshCollider;

impl Plugin for GLTFHelperPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app
        .add_plugins(ColliderGeneratorPlugin)
        .add_systems(Update, create_meshes);
    }

}


#[derive(Component, Debug)]
pub struct MakeMesh{pub path: String, pub position: Transform}

impl Default for MakeMesh {
    fn default() -> MakeMesh {
        return MakeMesh {
            path: String::default(),
            position: Transform::from_xyz(0.0, 0.0, 0.0)
        }
    }
}

fn create_meshes(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    entities: Query<(Entity, &MakeMesh), With<MakeMesh>>
) {
    for (entity, make_mesh) in entities.iter() {
        println!("added mesh to {:?}", entity);
        let scene_path = make_mesh.path.to_owned() + "#Scene0";
        let mesh_path = make_mesh.path.to_owned() + "#Mesh0/Primitive0";
        let mesh_handle = asset_server.load(mesh_path);

        commands.entity(entity).insert(
            (SceneBundle {
                scene: asset_server.load(scene_path),
                transform: make_mesh.position,
                ..default()
            },
            MakeMeshCollider(mesh_handle),)
        );

        commands.entity(entity).remove::<MakeMesh>();
        
    }
}
