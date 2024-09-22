use bevy::{
    prelude::*,
    render::{
        render_asset::RenderAssetUsages,
        render_resource::{Extent3d, TextureDimension, TextureFormat},
    },
};
use simdnoise::NoiseBuilder;

use crate::GameState;

const CHUNK_SIZE: u16 = 128;

pub struct MapGenPlugin;

impl Plugin for MapGenPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapGenSettings>()
            .add_systems(OnEnter(GameState::MapGeneration), setup_map_gen)
            .add_systems(OnExit(GameState::MapGeneration), cleanup_map_gen);
    }
}

#[derive(Resource)]
pub struct MapGenSettings {
    pub val_offset: f32,
    pub val_multiplier: f32,
}

#[derive(Component)]
struct MapGenDisplay;

impl Default for MapGenSettings {
    fn default() -> Self {
        Self {
            val_offset: 0.0,
            val_multiplier: 1.0,
        }
    }
}

fn generate_map(x_offset: f32, y_offset: f32, val_offset: f32, val_multiplier: f32) -> Image {
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
            let v = ((v + val_offset) * 255. * val_multiplier) as u8;
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

fn setup_map_gen(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    window: Query<&Window>,
    map_gen_settings: Res<MapGenSettings>,
) {
    let window = window.single();
    let width = window.resolution.width() / CHUNK_SIZE as f32;
    let height = window.resolution.height() / CHUNK_SIZE as f32;
    let max_length = (width.max(height) + 0.5).round() as i32;
    let max_length = max_length - 2; // debug
    let val_offset = map_gen_settings.val_offset;
    let val_multiplier = map_gen_settings.val_multiplier;
    commands
        .spawn((MapGenDisplay, SpatialBundle::default()))
        .with_children(|parent| {
            for (x, y) in generate_spiral_max(max_length) {
                parent.spawn((SpriteBundle {
                    transform: Transform::from_translation(Vec3 {
                        x: x as f32 * 128.0,
                        y: -y as f32 * 128.0,
                        z: 0.0,
                    }),
                    texture: textures.add(generate_map(
                        x as f32 * 128.0,
                        y as f32 * 128.0,
                        val_offset,
                        val_multiplier,
                    )),
                    ..default()
                },));
            }
        });
}

#[allow(dead_code)]
fn generate_spiral() -> impl Iterator<Item = (i32, i32)> {
    // generate tuples starting at center (0, 0), going up and counter-clockwise, then up again
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;
    std::iter::from_fn(move || {
        let result = (x, y);
        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }
        x += dx;
        y += dy;
        Some(result)
    })
}

fn generate_spiral_max(max_length: i32) -> impl Iterator<Item = (i32, i32)> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut dx = 0;
    let mut dy = -1;
    let max_radius = max_length / 2;
    std::iter::from_fn(move || {
        // checking if -x is greater than max_radius should suffice
        if x.abs() > max_radius || y.abs() > max_radius {
            return None;
        }
        let result = (x, y);
        if x == y || (x < 0 && x == -y) || (x > 0 && x == 1 - y) {
            let temp = dx;
            dx = -dy;
            dy = temp;
        }
        x += dx;
        y += dy;
        Some(result)
    })
}

fn cleanup_map_gen(mut commands: Commands, query: Query<Entity, With<MapGenDisplay>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
