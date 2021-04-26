use crate::assets::load_texture;
use crate::prelude::*;
use amethyst::{
    assets::{
        AssetStorage, Handle, Loader, Prefab, PrefabData, PrefabLoader, ProgressCounter, RonFormat,
    },
    renderer::{
        rendy::{
            hal::{
                format::Format,
                image::{Anisotropic, Filter, Kind, PackedColor, SamplerInfo, ViewKind, WrapMode},
            },
            texture::TextureBuilder,
        },
        types::TextureData,
        // ImageFormat,
    },
};
use captcha::{filters::Noise, Captcha};
use image::{self, ImageFormat};

pub const CAPTCHA_COUNT: u32 = 5;
pub const CAPTCHA_WIDTH: u32 = 220;
pub const CAPTCHA_HEIGHT: u32 = 120;

#[derive(Clone)]
pub struct CaptchaData {
    pub answer: String,
    pub texture: Handle<Texture>,
}

pub fn get_captchas<'a>(world: &mut World, progress: &'a mut ProgressCounter) -> Vec<CaptchaData> {
    let mut captchas = Vec::new();
    for _n in 1..=CAPTCHA_COUNT {
        captchas.push(gen_captcha(world, progress));
    }
    captchas
}

fn gen_captcha<'a>(world: &mut World, progress: &'a mut ProgressCounter) -> CaptchaData {
    let (answer, image_png_buffer) = Captcha::new()
        .add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(CAPTCHA_WIDTH, CAPTCHA_HEIGHT)
        .as_tuple()
        .expect("failed to gen captcha");
    let image =
        image::load_from_memory_with_format(&image_png_buffer, ImageFormat::Png).expect("image???");
    // let texture = load_2d_texture(
    //     world,
    //     progress,
    //     image.into_rgb8().into_vec(),
    //     CAPTCHA_WIDTH,
    //     CAPTCHA_HEIGHT,
    // );
    let file_path = format!("generated/{}.png", answer);
    image.save(format!("assets/{}", file_path));
    let texture = load_texture(world, file_path, progress);
    CaptchaData { answer, texture }
}

// file:///home/basicdays/wip/games/LudumDare48/target/doc/amethyst_rendy/struct.ImageFormat.html
// https://community.amethyst.rs/t/how-to-load-a-texture-from-memory/1090
// https://stackoverflow.com/questions/57691913/how-to-load-a-texture-from-memory-in-amethyst-engine

fn load_2d_texture<'a>(
    world: &mut World,
    progress: &'a mut ProgressCounter,
    image_buffer: Vec<u8>,
    width: u32,
    height: u32,
) -> Handle<Texture> {
    let builder = TextureBuilder::new()
        .with_data_width(width)
        .with_data_height(height)
        .with_kind(Kind::D2(width, height, 1, 1))
        .with_view_kind(ViewKind::D2)
        // TextureBuilder::new already has this...
        // .with_sampler_info(SamplerInfo {
        //     min_filter: Filter::Linear,
        //     mag_filter: Filter::Linear,
        //     mip_filter: Filter::Linear,
        //     wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
        //     lod_bias: 0.0.into(),
        //     lod_range: std::ops::Range {
        //         start: 0.0.into(),
        //         end: 1000.0.into(),
        //     },
        //     comparison: None,
        //     border: PackedColor(0),
        //     anisotropic: Anisotropic::Off,
        //     normalized: false,
        // })
        .with_raw_data(image_buffer, Format::Rgb8Uint);

    let loader = world.read_resource::<Loader>();
    let texture_storage = world.read_resource::<AssetStorage<Texture>>();
    loader.load_from_data(TextureData(builder), progress, &texture_storage)
}
