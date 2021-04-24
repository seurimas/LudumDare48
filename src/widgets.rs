use crate::prelude::*;
use amethyst::ui::{ToNativeWidget, UiCreator, UiWidget};
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub enum DiggingUi {
    Alertable { item: UiWidget<DiggingUi> },
    Card { item: UiWidget<DiggingUi> },
}

impl ToNativeWidget for DiggingUi {
    type PrefabData = ();
    fn to_native_widget(
        self,
        _parent_data: Self::PrefabData,
    ) -> (UiWidget<DiggingUi>, Self::PrefabData) {
        match self {
            DiggingUi::Card { item } => (item, ()),
            DiggingUi::Alertable { item } => (item, ()),
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
#[storage(DenseVecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(SystemData)]
pub struct WidgetSpawner<'a> {
    creator: UiCreator<'a, DiggingUi>,
    positions: WriteStorage<'a, Position>,
}

impl<'a> WidgetSpawner<'a> {
    pub fn spawn_ui_widget(&mut self, path: &'static str, position: Position) -> Entity {
        let entity = self.creator.create(path, ());
        self.positions
            .insert(entity, position)
            .expect("Unreachable: Entity was just created");
        entity
    }
}

pub struct WidgetPositioningSystem;

impl<'s> System<'s> for WidgetPositioningSystem {
    type SystemData = (ReadStorage<'s, Position>, WriteStorage<'s, UiTransform>);
    fn run(&mut self, (positions, mut transforms): Self::SystemData) {
        for (position, mut transform) in (&positions, &mut transforms).join() {
            transform.local_x = position.x;
            transform.local_y = position.y;
        }
    }
}
