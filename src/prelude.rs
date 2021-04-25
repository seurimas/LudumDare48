pub use crate::cards::Alertable;
pub use crate::digging::DiggingStatus;
pub use crate::widgets::*;
pub use amethyst::{
    assets::PrefabData,
    core::timing::Time,
    core::transform::Transform,
    core::{HiddenPropagate, Parent, SystemBundle},
    ecs::storage::GenericReadStorage,
    ecs::*,
    error::Error,
    input::{
        get_key, is_close_requested, is_key_down, InputHandler, StringBindings, VirtualKeyCode,
    },
    prelude::*,
    renderer::{sprite::SpriteSheetHandle, SpriteRender, SpriteSheet, Texture},
    shred::ResourceId,
    ui::{
        Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiEvent, UiEventType,
        UiFinder, UiImage, UiText, UiTransform,
    },
};
pub use rand::random;
pub use shrev::EventChannel;

pub fn get_ui_name(
    entity: Entity,
    transforms: &impl GenericReadStorage<Component = UiTransform>,
) -> String {
    transforms
        .get(entity)
        .map(|transform| transform.id.clone())
        .unwrap_or("".to_string())
}

pub fn update_texture(
    image: &mut UiImage,
    new_left: Option<f32>,
    new_right: Option<f32>,
    new_top: Option<f32>,
    new_bottom: Option<f32>,
) {
    if let UiImage::PartialTexture {
        left,
        right,
        top,
        bottom,
        ..
    } = image
    {
        if let Some(new_left) = new_left {
            *left = new_left;
        }
        if let Some(new_right) = new_right {
            *right = new_right;
        }
        if let Some(new_top) = new_top {
            *top = new_top;
        }
        if let Some(new_bottom) = new_bottom {
            *bottom = new_bottom;
        }
    }
}
