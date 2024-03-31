use bevy::prelude::*;
use bevy_rapier3d::geometry::{Collider, ComputedColliderShape};


#[derive(Component, Debug)]
pub struct MakeMeshCollider(pub Handle<Mesh>);

pub struct ColliderGeneratorPlugin;

impl Plugin for ColliderGeneratorPlugin {
    fn build(&self, app: &mut App) {
        // add things to your app here
        app
        .add_systems(FixedUpdate, gen_collider);
    }

}


pub fn gen_collider(mut commands: Commands, q: Query<(Entity, &MakeMeshCollider), With<MakeMeshCollider>>, ass: Res<Assets<Mesh>>) {
    
    for (et, handle) in q.iter() {
        println!("added collider to {:?}", et);
        let m = ass.get(handle.0.id());
        match m {
            Some(_) => (),
            None => return,
        }

        commands.entity(et).insert(
            Collider::from_bevy_mesh(m.unwrap(), &ComputedColliderShape::TriMesh).unwrap()
        );

        commands.entity(et).remove::<MakeMeshCollider>();
    }
}