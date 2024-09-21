use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use simdnoise::NoiseBuilder;

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

fn generate_map(x_offset: f32, y_offset: f32) -> Image {
    let width = 128;
    let height = 128;
    let noise = NoiseBuilder::fbm_2d_offset(x_offset, width, y_offset, height)
        .with_freq(0.5)
        .with_octaves(8)
        .generate();
    let pixel_data = noise
        .0
        .iter()
        .flat_map(|&v| {
            let v = ((v + 0.5) * 255. / 4.) as u8;
            vec![v, v, v, 255]
        })
        .collect::<Vec<u8>>();
    let texture_size = Extent3d {
        width: width as u32,
        height: height as u32,
        depth_or_array_layers: 1,
    };

    Image::new(
        texture_size,
        TextureDimension::D2,
        pixel_data,
        TextureFormat::Rgba8UnormSrgb,
        RenderAssetUsages::RENDER_WORLD,
        // change to this line if access to noise values after displaying them is needed
        // RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD,
    )
}

fn setup_map_gen(mut commands: Commands, mut textures: ResMut<Assets<Image>>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::default(),
            texture: textures.add(generate_map(0., 0.)),
            ..default()
        },
        MapGenDisplay,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_translation(Vec3 {
                x: 0.0,
                y: 128.0,
                z: 0.0,
            }),
            texture: textures.add(generate_map(0., -128.)),
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
