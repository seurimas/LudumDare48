#![windows_subsystem = "windows"]
use crate::cards::CardsBundle;
use crate::digging::DiggingBundle;
use crate::hole::{HoleTile, SpriteTile};
use crate::prelude::DjSystem;
use crate::widgets::WidgetPositioningSystem;
use amethyst::tiles::RenderTiles2D;
use amethyst::{
    audio::AudioBundle,
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

mod assets;
mod captcha;
mod cards;
mod digging;
mod hole;
mod prelude;
mod state;
mod widgets;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?.with_clear([0., 0., 0., 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderFlat2D::default())
                .with_plugin(RenderTiles2D::<HoleTile>::default())
                .with_plugin(RenderTiles2D::<SpriteTile>::default()),
        )?
        .with_bundle(AudioBundle::default())?
        .with_bundle(CardsBundle)?
        .with_bundle(DiggingBundle)?
        .with(DjSystem, "dj", &[])
        .with(crate::state::EndGameRenderer, "endgame", &[])
        .with(WidgetPositioningSystem, "widget_pos", &[]);

    let mut game = Application::new(resources, state::LoadingState::new(), game_data)?;
    game.run();

    Ok(())
}
