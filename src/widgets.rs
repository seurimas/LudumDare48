use crate::prelude::*;
use amethyst::ui::UiCreator;
use serde::Deserialize;

#[derive(Component, Debug, Clone, Copy)]
#[storage(DenseVecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn spawn_ui_widget<'a>(world: &mut World, path: &'static str, position: Position) -> Entity {
    world.exec(
        |(mut creator, mut positions): (UiCreator, WriteStorage<'_, Position>)| {
            let entity = creator.create(path, ());
            positions
                .insert(entity, position)
                .expect("Unreachable: Entity was just created");
            entity
        },
    )
}

pub struct WidgetPositioningSystem;

impl<'s> System<'s> for WidgetPositioningSystem {
    type SystemData = (ReadStorage<'s, Position>, WriteStorage<'s, UiTransform>);
    fn run(&mut self, (mut positions, mut transforms): Self::SystemData) {
        for (position, mut transform) in (&positions, &mut transforms).join() {
            transform.local_x = position.x;
            transform.local_y = position.y;
        }
    }
}
