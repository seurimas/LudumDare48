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

const WIDTH: u32 = 220;
const HEIGHT: u32 = 120;

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
        .view(WIDTH, HEIGHT)
        .as_tuple()
        .expect("failed to gen captcha");
    let texture = load_texture(image_buffer);
    CaptchaData { answer, texture }
}

fn load_texture<'a>(image_buffer: Vec<u8>) -> TextureData {
    let builder = TextureBuilder::new()
        .with_data_width(WIDTH)
        .with_data_height(HEIGHT)
        .with_kind(Kind::D2(WIDTH, HEIGHT, 1, 1))
        .with_view_kind(ViewKind::D2)
        .with_sampler_info(SamplerInfo {
            min_filter: Filter::Linear,
            mag_filter: Filter::Linear,
            mip_filter: Filter::Linear,
            wrap_mode: (WrapMode::Clamp, WrapMode::Clamp, WrapMode::Clamp),
            lod_bias: 0.0.into(),
            lod_range: std::ops::Range {
                start: 0.0.into(),
                end: 1000.0.into(),
            },
            comparison: None,
            border: PackedColor(0),
            anisotropic: Anisotropic::Off,
            normalized: true,
        })
        .with_raw_data(image_buffer, Format::Rgba8Uint);
    TextureData(builder)
}
