use crate::assets::SpriteStorage;
use crate::prelude::*;
use amethyst::core::math::{Point3, Vector3};
use amethyst::tiles::*;

pub const VICTORY_DEPTH: u32 = 100;
pub const TILE_SCREEN_SIZE: f32 = 64.;

#[derive(Default, Clone)]
pub struct HoleTile;
impl Tile for HoleTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        let (digging,): (Read<DiggingStatus>,) = world.system_data();
        let sprite_idx = if point.y < crate::digging::DRILL_METER {
            0
        } else if point.y < crate::digging::ROBOT_METER {
            16
        } else if point.y < VICTORY_DEPTH - 1 {
            24
        } else {
            32
        };
        if point.y < digging.level() {
            Some(sprite_idx + 4)
        } else if point.y > digging.level() {
            Some(sprite_idx)
        } else if point.x < digging.current_block() {
            Some(sprite_idx + 4)
        } else if point.x == digging.current_block() {
            Some(sprite_idx + digging.current_block_height() as usize)
        } else {
            Some(sprite_idx)
        }
    }

    fn tint(&self, coordinates: Point3<u32>, world: &World) -> Srgba {
        Srgba::new(1.0, 1.0, 1.0, 1.0)
    }
}

#[derive(Default, Clone)]
pub struct SpriteTile;
impl Tile for SpriteTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        let (digging,): (Read<DiggingStatus>,) = world.system_data();
        let block_index = digging.block_index();
        let tile_index = point.y * BLOCKS_PER_METER + point.x;
        if block_index > 0 && tile_index == block_index - 1 {
            if digging.time_since_shovel < 0.125 {
                Some(9)
            } else {
                Some(8)
            }
        } else if block_index > 1 && tile_index == block_index - 2 {
            match digging.robot_status {
                RobotStatus::Locked => None,
                RobotStatus::Running { .. } => None,
                RobotStatus::Idling => Some(12),
            }
        } else if block_index >= BLOCKS_PER_METER && tile_index == block_index - BLOCKS_PER_METER {
            match digging.drill_status {
                DrillStatus::Locked => None,
                DrillStatus::Running { .. } => Some(10),
                DrillStatus::Idling => Some(11),
            }
        } else {
            None
        }
    }
}

pub fn spawn_hole(world: &mut World) {
    let tilesheet = world
        .read_resource::<SpriteStorage>()
        .tile_spritesheet
        .clone();
    let master = world.read_resource::<SpriteStorage>().master.clone();
    let mut transform = Transform::default();
    transform.set_translation_x(32.);
    transform.set_translation_y(VICTORY_DEPTH as f32 * -32.);
    transform.set_translation_z(0.2);
    transform.set_scale(Vector3::new(2., 2., 1.));
    world
        .create_entity()
        .with(TileMap::<HoleTile, MortonEncoder2D>::new(
            Vector3::<u32>::new(BLOCKS_PER_METER, VICTORY_DEPTH, 1),
            Vector3::<u32>::new(32, 32, 1),
            Some(tilesheet.clone()),
        ))
        .with(transform.clone())
        .build();
    transform.set_translation_z(0.5);
    world
        .create_entity()
        .with(TileMap::<SpriteTile, MortonEncoder2D>::new(
            Vector3::<u32>::new(BLOCKS_PER_METER, VICTORY_DEPTH, 1),
            Vector3::<u32>::new(32, 32, 1),
            Some(tilesheet),
        ))
        .with(transform)
        .build();
    let mut fixture_transform = Transform::default();
    // Left border
    fixture_transform.set_translation_x(-512. + 32.);
    fixture_transform.set_translation_y(-250. + 96.);
    fixture_transform.set_translation_z(0.6);
    fixture_transform.set_scale(Vector3::new(4., 4., 1.));
    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: master.clone(),
            sprite_number: 2,
        })
        .with(fixture_transform.clone())
        .build();
    // Left
    fixture_transform.set_translation_x(512. - 32.);
    fixture_transform.set_scale(Vector3::new(-4., 4., 1.));
    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: master.clone(),
            sprite_number: 2,
        })
        .with(fixture_transform.clone())
        .build();
    // Sky
    fixture_transform.set_translation_x(0.);
    fixture_transform.set_translation_y(512.);
    fixture_transform.set_translation_z(0.1);
    fixture_transform.set_scale(Vector3::new(64., 64., 1.));
    world
        .create_entity()
        .with(SpriteRender {
            sprite_sheet: master.clone(),
            sprite_number: 3,
        })
        .with(fixture_transform.clone())
        .build();
}
