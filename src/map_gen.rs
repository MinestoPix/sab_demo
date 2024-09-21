use bevy::{color::palettes::css::PURPLE, prelude::*, sprite::MaterialMesh2dBundle};

use crate::GameState;

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MapGeneration), setup_map_gen)
            .add_systems(OnExit(GameState::MapGeneration), cleanup_map_gen);
    }
}

#[derive(Component)]
struct MapGenDisplay;

fn setup_map_gen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Rectangle::new(2., 2.)).into(),
            transform: Transform::default().with_scale(Vec3::splat(128.)),
            material: materials.add(Color::from(PURPLE)),
            ..default()
        },
        MapGenDisplay,
    ));
}

fn cleanup_map_gen(mut commands: Commands, query: Query<Entity, With<MapGenDisplay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
