use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
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
    const VERY_HIGH_CHROMA: f32 = 0.16;

    pub(crate) fn red(&self) -> Oklch {
        oklch(Self::DARK_LIGHTNESS, Self::HIGH_CHROMA, 25.0)
    }

    pub(crate) fn orange(&self) -> Oklch {
        oklch(Self::BRIGHT_LIGHTNESS, Self::VERY_HIGH_CHROMA, 60.0)
    }

    pub(crate) fn dark_orange(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::HIGH_CHROMA, 60.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(Self::VERY_BRIGHT_LIGHTNESS, Self::MEDIUM_CHROMA, 100.0)
    }

    pub(crate) fn olive(&self) -> Oklch {
        oklch(Self::DARK_LIGHTNESS, Self::HIGH_CHROMA, 120.0)
    }

    pub(crate) fn green(&self) -> Oklch {
        oklch(Self::DARK_LIGHTNESS, Self::MEDIUM_CHROMA, 140.0)
    }

    pub(crate) fn teal(&self) -> Oklch {
        oklch(Self::DARK_LIGHTNESS, Self::LOW_CHROMA, 200.0)
    }

    pub(crate) fn sky_blue(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::VERY_LOW_CHROMA, 220.0)
    }

    pub(crate) fn dark_sky_blue(&self) -> Oklch {
        oklch(Self::DARK_LIGHTNESS, Self::VERY_LOW_CHROMA, 220.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::MEDIUM_CHROMA, 250.0)
    }

    pub(crate) fn dark_blue(&self) -> Oklch {
        oklch(Self::DARK_LIGHTNESS, Self::MEDIUM_CHROMA, 250.0)
    }

    pub(crate) fn pale_blue(&self) -> Oklch {
        oklch(Self::MEDIUM_LIGHTNESS, Self::VERY_LOW_CHROMA, 250.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
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
        lerp(self.value(), 0.24..0.99)
    }

    fn chroma(self) -> f32 {
        lerp(self.value(), 0.0..0.03)
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

fn oklch(l: f32, c: f32, h: f32) -> Oklch {
    Oklch {
        l,
        c,
        h: Hue::from_degrees(h).unwrap(),
    }
}

fn lerp(x: f32, range: Range<f32>) -> f32 {
    x * (range.end - range.start) + range.start
}
