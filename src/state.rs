use crate::assets::*;
use crate::prelude::*;
use amethyst::{
    assets::{AssetStorage, Loader},
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

use log::info;

/// A dummy game state that shows 3 sprites.
pub struct MyState;

impl SimpleState for MyState {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - 3 sprites places around the center.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let sprites = load_sprites(world);
        init_sprites(world, &sprites, &dimensions);

        create_ui_example(world);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

/// Loads and splits the `logo.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/logo.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/logo.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    (0..3)
        .map(|i| SpriteRender {
            sprite_sheet: sheet_handle.clone(),
            sprite_number: i,
        })
        .collect()
}

/// Creates an entity in the `world` for each of the provided `sprites`.
/// They are individually placed around the center of the screen.
fn init_sprites(world: &mut World, sprites: &[SpriteRender], dimensions: &ScreenDimensions) {
    for (i, sprite) in sprites.iter().enumerate() {
        // Center our sprites around the center of the window
        let x = (i as f32 - 1.) * 100. + dimensions.width() * 0.5;
        let y = (i as f32 - 1.) * 100. + dimensions.height() * 0.5;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);

        // Create an entity for each sprite and attach the `SpriteRender` as
        // well as the transform. If you want to add behaviour to your sprites,
        // you'll want to add a custom `Component` that will identify them, and a
        // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
        world
            .create_entity()
            .with(sprite.clone())
            .with(transform)
            .build();
    }
}

/// Creates a simple UI background and a UI text label
/// This is the pure code only way to create UI with amethyst.
pub fn create_ui_example(world: &mut World) {
    // this creates the simple gray background UI element.
    let ui_background = world
        .create_entity()
        .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            30.0,
            -30.,
            0.,
            250.,
            50.,
        ))
        .build();

    // This simply loads a font from the asset folder and puts it in the world as a resource,
    // we also get a ref to the font that we then can pass to the text label we crate later.
    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // This creates the actual label and places it on the screen.
    // Take note of the z position given, this ensures the label gets rendered above the background UI element.
    world
        .create_entity()
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            40.0,
            -40.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font,
            "Hello, Amethyst UI!".to_string(),
            [1., 1., 1., 1.],
            30.,
            LineMode::Single,
            Anchor::TopLeft,
        ))
        .build();
}

struct GameplayState {
    assets: GameAssets,
}

impl SimpleState for GameplayState {
    fn on_start(&mut self, mut data: StateData<'_, GameData<'_, '_>>) {
        data.world.delete_all();
        data.world.insert(self.assets.0.clone());
        let dimensions = (*data.world.read_resource::<ScreenDimensions>()).clone();
        init_camera(data.world, &dimensions);
        data.world.exec(
            |(mut spawner, mut alertables, mut buckets): (
                WidgetSpawner,
                WriteStorage<'_, crate::cards::Alertable>,
                WriteStorage<'_, crate::digging::Bucket>,
            )| {
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
                let alert_entity =
                    spawner.spawn_ui_widget("prefabs/alertable.ron", Position { x: -64., y: -32. });
                alertables
                    .insert(
                        alert_entity,
                        crate::cards::Alertable {
                            state: crate::cards::AlertState::Shovel,
                            clicked: false,
                        },
                    )
                    .expect("Unreachable: entity just created");
                let bucket_alert_entity =
                    spawner.spawn_ui_widget("prefabs/alertable.ron", Position { x: -64., y: -96. });
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
        self.progress = Some(progress_counter);
        self.assets = Some((SpriteStorage { master },));
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
