use crate::assets::SpriteStorage;
use crate::prelude::*;
use amethyst::core::math::{Point3, Vector3};
use amethyst::tiles::*;

#[derive(Default, Clone)]
pub struct HoleTile;
impl Tile for HoleTile {
    fn sprite(&self, point: Point3<u32>, world: &World) -> Option<usize> {
        let (digging,): (Read<DiggingStatus>,) = world.system_data();
        if point.y < digging.level() {
            Some(4)
        } else if point.y > digging.level() {
            Some(0)
        } else if point.x < digging.current_block() {
            Some(4)
        } else if point.x == digging.current_block() {
            Some(digging.current_block_height() as usize)
        } else {
            Some(0)
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
    let mut transform = Transform::default();
    transform.set_translation_x(16.);
    transform.set_translation_y(-250. * 32.);
    world
        .create_entity()
        .with(TileMap::<HoleTile, MortonEncoder2D>::new(
            Vector3::<u32>::new(BLOCKS_PER_METER, 500, 1),
            Vector3::<u32>::new(32, 32, 1),
            Some(tilesheet.clone()),
        ))
        .with(transform.clone())
        .build();
    transform.set_translation_z(0.5);
    world
        .create_entity()
        .with(TileMap::<SpriteTile, MortonEncoder2D>::new(
            Vector3::<u32>::new(BLOCKS_PER_METER, 500, 1),
            Vector3::<u32>::new(32, 32, 1),
            Some(tilesheet),
        ))
        .with(transform)
        .build();
}
