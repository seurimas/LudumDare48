use crate::assets::*;
use crate::captcha;
use crate::hole::spawn_hole;
use crate::hole::VICTORY_DEPTH;
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
        data.world.insert(self.assets.2.clone());
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        init_camera(data.world, &dimensions);
        spawn_hole(data.world);
        data.world.exec(
            |(mut spawner, mut alertables, mut buckets, mut robots): (
                WidgetSpawner,
                WriteStorage<'_, crate::cards::Alertable>,
                WriteStorage<'_, crate::digging::Bucket>,
                WriteStorage<'_, crate::digging::Robot>,
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
                let robot_entity =
                    spawner.spawn_ui_widget("prefabs/robot.ron", Position { x: -48., y: 16. });
                robots
                    .insert(robot_entity, crate::digging::Robot { index: 0 })
                    .expect("Unreachable, entity just created");
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

    fn update(&mut self, data: &mut StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        data.world
            .exec(|(digging,): (Read<'_, crate::digging::DiggingStatus>,)| {
                if digging.depth > VICTORY_DEPTH {
                    return Trans::Switch(Box::new(GameOverState {
                        assets: self.assets.clone(),
                    }));
                }
                Trans::None
            })
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
        let shovel = load_sound_file(
            data.world,
            "audio/shovel.wav".to_string(),
            &mut progress_counter,
        );
        let empty_bucket = load_sound_file(
            data.world,
            "audio/empty_bucket.wav".to_string(),
            &mut progress_counter,
        );
        let drill_spin = load_sound_file(
            data.world,
            "audio/drill_spin.wav".to_string(),
            &mut progress_counter,
        );
        let drill_start = load_sound_file(
            data.world,
            "audio/drill_start.wav".to_string(),
            &mut progress_counter,
        );
        let drill_unlock = load_sound_file(
            data.world,
            "audio/drill_unlock.wav".to_string(),
            &mut progress_counter,
        );
        let robot_captcha = load_sound_file(
            data.world,
            "audio/robot_captcha.wav".to_string(),
            &mut progress_counter,
        );
        let robot_captcha_success = load_sound_file(
            data.world,
            "audio/robot_captcha_success.wav".to_string(),
            &mut progress_counter,
        );
        let robot_captcha_fail = load_sound_file(
            data.world,
            "audio/robot_captcha_fail.wav".to_string(),
            &mut progress_counter,
        );
        let robot_captcha_key = load_sound_file(
            data.world,
            "audio/robot_captcha_key.wav".to_string(),
            &mut progress_counter,
        );
        let robot_unlock = load_sound_file(
            data.world,
            "audio/robot_unlock.wav".to_string(),
            &mut progress_counter,
        );
        let captchas = captcha::get_captchas(data.world, &mut progress_counter);
        self.progress = Some(progress_counter);
        self.assets = Some((
            SpriteStorage {
                master,
                tile_spritesheet,
            },
            SoundStorage {
                main_theme,
                shovel,
                empty_bucket,
                drill_spin,
                drill_start,
                drill_unlock,
                robot_captcha,
                robot_captcha_success,
                robot_captcha_fail,
                robot_captcha_key,
                robot_unlock,
            },
            captchas,
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
                return SimpleTrans::Switch(Box::new(TitleViewState {
                    assets: self.assets.clone().unwrap(),
                }));
            }
        }
        SimpleTrans::None
    }
}

struct TitleViewState {
    assets: GameAssets,
}

impl SimpleState for TitleViewState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.exec(|mut spawner: WidgetSpawner| {
            spawner.spawn_ui_widget("prefabs/title_view.ron", Position { x: 0., y: 0. })
        });
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Ui(ui_event) => data.world.exec(|finder: UiFinder<'_>| {
                if ui_event.event_type == UiEventType::Click {
                    if let Some(play) = finder.find("play") {
                        if play == ui_event.target {
                            return SimpleTrans::Switch(Box::new(GameplayState {
                                assets: self.assets.clone(),
                            }));
                        }
                    }
                    if let Some(exit) = finder.find("exit") {
                        if exit == ui_event.target {
                            return Trans::Quit;
                        }
                    }
                }
                Trans::None
            }),
            _ => Trans::None,
        }
    }
}

struct GameOverState {
    assets: GameAssets,
}

impl SimpleState for GameOverState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        data.world.exec(
            |(mut spawner, digging): (WidgetSpawner, Read<'_, DiggingStatus>)| {
                spawner.spawn_ui_widget("prefabs/game_over.ron", Position { x: 0., y: 0. })
            },
        );
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        match event {
            StateEvent::Ui(ui_event) => data.world.exec(
                |(finder, mut digging): (UiFinder<'_>, Write<'_, DiggingStatus>)| {
                    if ui_event.event_type == UiEventType::Click {
                        if let Some(play) = finder.find("play") {
                            *digging = DiggingStatus::default();
                            if play == ui_event.target {
                                return SimpleTrans::Switch(Box::new(GameplayState {
                                    assets: self.assets.clone(),
                                }));
                            }
                        }
                        if let Some(exit) = finder.find("exit") {
                            if exit == ui_event.target {
                                return Trans::Quit;
                            }
                        }
                    }
                    Trans::None
                },
            ),
            _ => Trans::None,
        }
    }
}
