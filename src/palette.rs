use std::ops::Range;
use tincture::{Hue, Oklch};

pub(crate) struct Palette;

impl Palette {
    pub(crate) fn base(&self, scale: BaseScale) -> Oklch {
        oklch(scale.lightness(), scale.chroma(), 100.0)
    }

    pub(crate) fn blue(&self) -> Oklch {
        oklch(0.75, 0.06, 250.0)
    }

    pub(crate) fn purple(&self) -> Oklch {
        oklch(0.65, 0.04, 265.0)
    }

    pub(crate) fn yellow(&self) -> Oklch {
        oklch(0.95, 0.06, 100.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum BaseScale {
    Bg,
    Fg,
    BrightFg,
}

impl BaseScale {
    fn lightness(self) -> f32 {
        lerp(self.value(), 0.25..0.95)
    }

    fn chroma(self) -> f32 {
        lerp(self.value(), 0.0..0.025)
    }

    fn value(self) -> f32 {
        match self {
            Self::Bg => 0.0,
            Self::Fg => 0.8,
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
