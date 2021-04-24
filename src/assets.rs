use crate::prelude::*;
pub use amethyst::assets::{
    AssetStorage, Handle, Loader, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat,
};
use amethyst::derive::PrefabData;
use amethyst::renderer::sprite::prefab::SpriteScenePrefab;
use amethyst::renderer::sprite::SpriteSheetFormat;
use amethyst::renderer::ImageFormat;
use serde::Deserialize;

#[derive(Debug, Clone, Default, Deserialize, PrefabData)]
pub struct DiggingPrefabData {
    sprite_scene: Option<SpriteScenePrefab>,
}

pub fn load_prefab<'a>(
    world: &mut World,
    path: String,
    progress: &'a mut ProgressCounter,
) -> Handle<Prefab<DiggingPrefabData>> {
    world.exec(|loader: PrefabLoader<'_, DiggingPrefabData>| loader.load(path, RonFormat, progress))
}

pub fn load_texture<'a>(
    world: &mut World,
    path: String,
    progress: &'a mut ProgressCounter,
) -> Handle<Texture> {
    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load(path, ImageFormat::default(), progress, &texture_storage)
}

pub fn load_spritesheet<'a>(
    world: &mut World,
    path: String,
    progress: &'a mut ProgressCounter,
) -> SpriteSheetHandle {
    let texture_handle = load_texture(world, format!("{}.png", path), progress);
    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", path), // Here we load the associated ron file
        SpriteSheetFormat(texture_handle),
        progress,
        &sprite_sheet_store,
    )
}

#[derive(Clone)]
pub struct SpriteStorage {
    pub master: SpriteSheetHandle,
}

pub type GameAssets = (SpriteStorage,);
