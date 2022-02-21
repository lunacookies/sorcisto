use std::ops::Range;
use tincture::Oklch;

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> (u8, u8, u8) {
        oklch(scale.lightness(), scale.chroma(), 100.0)
    }

    const DARK_LIGHTNESS: f32 = 0.62;
    const MEDIUM_LIGHTNESS: f32 = 0.72;
    const BRIGHT_LIGHTNESS: f32 = 0.78;
    const VERY_BRIGHT_LIGHTNESS: f32 = 0.96;

    const VERY_LOW_CHROMA: f32 = 0.0325;
    const LOW_CHROMA: f32 = 0.04;
    const MEDIUM_CHROMA: f32 = 0.06;
    const HIGH_CHROMA: f32 = 0.13;
    const VERY_HIGH_CHROMA: f32 = 0.156;

    pub(crate) fn red(&self) -> (u8, u8, u8) {
        oklch(Self::DARK_LIGHTNESS, Self::HIGH_CHROMA, 25.0)
    }

    pub(crate) fn orange(&self) -> (u8, u8, u8) {
        oklch(Self::BRIGHT_LIGHTNESS, Self::VERY_HIGH_CHROMA, 60.0)
    }

    pub(crate) fn dark_orange(&self) -> (u8, u8, u8) {
        oklch(Self::MEDIUM_LIGHTNESS, Self::HIGH_CHROMA, 60.0)
    }

    pub(crate) fn yellow(&self) -> (u8, u8, u8) {
        oklch(Self::VERY_BRIGHT_LIGHTNESS, Self::MEDIUM_CHROMA, 100.0)
    }

    pub(crate) fn olive(&self) -> (u8, u8, u8) {
        oklch(Self::DARK_LIGHTNESS, Self::HIGH_CHROMA, 120.0)
    }

    pub(crate) fn green(&self) -> (u8, u8, u8) {
        oklch(Self::DARK_LIGHTNESS, Self::MEDIUM_CHROMA, 140.0)
    }

    pub(crate) fn teal(&self) -> (u8, u8, u8) {
        oklch(Self::DARK_LIGHTNESS, Self::LOW_CHROMA, 200.0)
    }

    pub(crate) fn sky_blue(&self) -> (u8, u8, u8) {
        oklch(Self::MEDIUM_LIGHTNESS, Self::VERY_LOW_CHROMA, 220.0)
    }

    pub(crate) fn dark_sky_blue(&self) -> (u8, u8, u8) {
        oklch(Self::DARK_LIGHTNESS, Self::VERY_LOW_CHROMA, 220.0)
    }

    pub(crate) fn blue(&self) -> (u8, u8, u8) {
        oklch(Self::MEDIUM_LIGHTNESS, Self::MEDIUM_CHROMA, 250.0)
    }

    pub(crate) fn dark_blue(&self) -> (u8, u8, u8) {
        oklch(Self::DARK_LIGHTNESS, Self::MEDIUM_CHROMA, 250.0)
    }

    pub(crate) fn pale_blue(&self) -> (u8, u8, u8) {
        oklch(Self::MEDIUM_LIGHTNESS, Self::VERY_LOW_CHROMA, 250.0)
    }

    pub(crate) fn purple(&self) -> (u8, u8, u8) {
        oklch(Self::DARK_LIGHTNESS, Self::LOW_CHROMA, 265.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    LightBg,
    LighterBg,
    DarkFg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.24..0.98)
    }

    fn chroma(self) -> f32 {
        lerp(self.value(), 0.0..0.029)
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::LightBg => 0.03,
            Self::LighterBg => 0.15,
            Self::DarkFg => 0.3,
            Self::Fg => 0.75,
            Self::BrightFg => 1.0,
        }
    }
}

fn oklch(l: f32, c: f32, h: f32) -> (u8, u8, u8) {
    let oklch = Oklch { l, c, h: h.to_radians() };
    let oklab = tincture::oklch_to_oklab(oklch);
    let linear_srgb = tincture::oklab_to_linear_srgb(oklab);
    let srgb = tincture::linear_srgb_to_srgb(linear_srgb);

    assert!(
        srgb.r > 0.0
            && srgb.r < 1.0
            && srgb.g > 0.0
            && srgb.g < 1.0
            && srgb.b > 0.0
            && srgb.b < 1.0,
        "\n{:?}\n{:?}\n{:?}\nwas out of range",
        oklch,
        oklab,
        srgb
    );

    srgb.components()
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
