pub use crate::assets::{SoundStorage, SpriteStorage};
pub use crate::captcha::CaptchaData;
pub use crate::cards::Alertable;
pub use crate::digging::{
    DiggingStatus, DrillStatus, RobotStatus, BLOCKS_PER_METER, SCOOPS_PER_BLOCK, SCOOPS_PER_METER,
};
pub use crate::widgets::*;
pub use amethyst::{
    assets::{AssetStorage, PrefabData},
    audio::{output::Output, Source, SourceHandle},
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
    renderer::palette::Srgba,
    renderer::{sprite::SpriteSheetHandle, SpriteRender, SpriteSheet, Texture},
    shred::ResourceId,
    ui::{
        Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiEvent, UiEventType,
        UiFinder, UiImage, UiText, UiTransform,
    },
};
pub use rand::{random, thread_rng, Rng};
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

#[derive(SystemData)]
pub struct SoundPlayer<'a> {
    storage: Option<Read<'a, SoundStorage>>,
    output: Option<Read<'a, Output>>,
    sources: Read<'a, AssetStorage<Source>>,
}

impl<'a> SoundPlayer<'a> {
    pub fn shovel(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.shovel.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn empty_bucket(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.empty_bucket.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn drill_spin(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.drill_spin.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn drill_start(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.drill_start.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn drill_unlock(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.drill_unlock.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn robot_captcha(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.robot_captcha.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn robot_captcha_key(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.robot_captcha_key.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn robot_captcha_fail(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.robot_captcha_fail.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn robot_captcha_success(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.robot_captcha_success.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn robot_unlock(&self) {
        if let Some(ref output) = self.output.as_ref() {
            if let Some(ref sounds) = self.storage.as_ref() {
                if let Some(sound) = self.sources.get(&sounds.robot_unlock.clone()) {
                    output.play_once(sound, 1.);
                }
            }
        }
    }
    pub fn play_main_theme(&self, sink: &amethyst::audio::AudioSink) {
        if let Some(ref sounds) = self.storage.as_ref() {
            if let Some(sound) = self.sources.get(&sounds.main_theme.clone()) {
                sink.append(sound);
            }
        }
    }
}

pub struct DjSystem;

impl<'a> System<'a> for DjSystem {
    type SystemData = (
        Option<Read<'a, amethyst::audio::AudioSink>>,
        SoundPlayer<'a>,
    );

    fn run(&mut self, (sink, player): Self::SystemData) {
        if let Some(ref sink) = sink {
            if sink.empty() {
                player.play_main_theme(sink);
            }
        }
    }
}
