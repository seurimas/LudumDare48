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
            None
        } else if point.y > digging.level() {
            Some(0)
        } else if point.x < digging.current_block() {
            None
        } else if point.x == digging.current_block() {
            Some(digging.current_block_height() as usize)
        } else {
            Some(0)
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
            Some(tilesheet),
        ))
        .with(transform)
        .build();
}
