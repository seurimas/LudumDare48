use amethyst::renderer::{
    rendy::{
        hal::{
            format::Format,
            image::{Anisotropic, Filter, Kind, PackedColor, SamplerInfo, ViewKind, WrapMode},
        },
        texture::TextureBuilder,
    },
    types::TextureData,
};
use captcha::{filters::Noise, Captcha};

pub const CAPTCHA_WIDTH: u32 = 220;
pub const CAPTCHA_HEIGHT: u32 = 120;

pub struct CaptchaData {
    answer: String,
    texture: TextureData,
}

pub fn get_captchas() -> Vec<CaptchaData> {
    let mut captchas = Vec::new();
    for _n in 1..=50 {
        captchas.push(gen_captcha());
    }
    captchas
}

fn gen_captcha() -> CaptchaData {
    let (answer, image_buffer) = Captcha::new()
        .add_chars(5)
        .apply_filter(Noise::new(0.1))
        .view(CAPTCHA_WIDTH, CAPTCHA_HEIGHT)
        .as_tuple()
        .expect("failed to gen captcha");
    let texture = load_2d_texture(image_buffer, CAPTCHA_WIDTH, CAPTCHA_HEIGHT);
    CaptchaData { answer, texture }
}

// yanked from file:///home/basicdays/wip/games/LudumDare48/target/doc/amethyst_rendy/struct.ImageFormat.html

fn load_2d_texture<'a>(image_buffer: Vec<u8>, width: u32, height: u32) -> TextureData {
    let builder = TextureBuilder::new()
        .with_data_width(width)
        .with_data_height(height)
        .with_kind(Kind::D2(width, height, 1, 1))
        .with_view_kind(ViewKind::D2)
        // TextureBuilder::new already has this...
        //.with_sampler_info(SamplerInfo::new(Filter::Linear, WrapMode::Clamp))
        .with_raw_data(image_buffer, Format::Rgba8Uint);
    TextureData(builder)
}
