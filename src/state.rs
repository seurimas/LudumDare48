use crate::assets::*;
use crate::hole::spawn_hole;
use crate::prelude::*;
use amethyst::{
    assets::{AssetStorage, Loader},
    audio::output::init_output,
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{
        Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiImage, UiText,
        UiTransform,
    },
    window::ScreenDimensions,
};

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 0., 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

struct GameplayState {
    assets: GameAssets,
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
        data.world.insert(self.assets.0.clone());
        data.world.insert(self.assets.1.clone());
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        init_camera(data.world, &dimensions);
        spawn_hole(data.world);
        data.world.exec(
            |(mut spawner, mut alertables, mut buckets): (
                WidgetSpawner,
                WriteStorage<'_, crate::cards::Alertable>,
                WriteStorage<'_, crate::digging::Bucket>,
            )| {
                spawner.spawn_ui_widget("prefabs/depth.ron", Position { x: 0., y: -16. });
                for i in 0..16 {
                    let bucket_entity = spawner.spawn_ui_widget(
                        "prefabs/bucket.ron",
                        Position {
                            x: -16.,
                            y: 16. + (i as f32 * 32.),
                        },
                    );
                    buckets
                        .insert(bucket_entity, crate::digging::Bucket { index: i })
                        .expect("Unreachable, entity just created");
                }
                let alert_entity = spawner.spawn_ui_widget(
                    "prefabs/shovel_alertable.ron",
                    Position { x: -64., y: -32. },
                );
                alertables
                    .insert(
                        alert_entity,
                        crate::cards::Alertable {
                            state: crate::cards::AlertState::Shovel(
                                crate::cards::ShovelAlertState::Ready,
                            ),
                            clicked: false,
                        },
                    )
                    .expect("Unreachable: entity just created");
                let bucket_alert_entity = spawner.spawn_ui_widget(
                    "prefabs/bucket_alertable.ron",
                    Position { x: -64., y: -96. },
                );
                alertables
                    .insert(
                        bucket_alert_entity,
                        crate::cards::Alertable {
                            state: crate::cards::AlertState::Bucket(
                                crate::cards::BucketAlertState::Empty,
                            ),
                            clicked: false,
                        },
                    )
                    .expect("Unreachable: entity just created");
            },
        );
    }
}

#[derive(Default)]
pub struct LoadingState {
    progress: Option<ProgressCounter>,
    assets: Option<GameAssets>,
}

impl LoadingState {
    pub fn new() -> Self {
        LoadingState {
            progress: None,
            assets: None,
        }
    }
}

impl SimpleState for LoadingState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        println!("Starting loading");
        let mut progress_counter = ProgressCounter::new();

        let master = load_spritesheet(
            data.world,
            "sprites/sheet".to_string(),
            &mut progress_counter,
        );
        let tile_spritesheet = load_spritesheet(
            data.world,
            "sprites/tiles".to_string(),
            &mut progress_counter,
        );
        let main_theme = load_sound_file(
            data.world,
            "audio/DiggingDeeper.wav".to_string(),
            &mut progress_counter,
        );
        self.progress = Some(progress_counter);
        self.assets = Some((
            SpriteStorage {
                master,
                tile_spritesheet,
            },
            SoundStorage { main_theme },
        ));

        init_output(data.world);
    }

    fn update(&mut self, _data: &mut StateData<GameData>) -> SimpleTrans {
        if let Some(progress) = &self.progress {
            if !progress.errors().is_empty() {
                println!("{:?}", progress);
                return SimpleTrans::Quit;
            }
            if progress.is_complete() {
                return SimpleTrans::Switch(Box::new(GameplayState {
                    assets: self.assets.clone().unwrap(),
                }));
            }
        }
        SimpleTrans::None
    }
}
