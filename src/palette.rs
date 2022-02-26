use once_cell::sync::OnceCell;
use std::ops::Range;
use tincture::Oklch;

static PALETTE: OnceCell<Palette> = OnceCell::new();

pub(crate) struct Palette {
    pub(crate) bg: (u8, u8, u8),
    pub(crate) light_bg: (u8, u8, u8),
    pub(crate) lighter_bg: (u8, u8, u8),
    pub(crate) dark_fg: (u8, u8, u8),
    pub(crate) fg: (u8, u8, u8),
    pub(crate) bright_fg: (u8, u8, u8),
    pub(crate) red: (u8, u8, u8),
    pub(crate) orange: (u8, u8, u8),
    pub(crate) dark_orange: (u8, u8, u8),
    pub(crate) yellow: (u8, u8, u8),
    pub(crate) olive: (u8, u8, u8),
    pub(crate) green: (u8, u8, u8),
    pub(crate) teal: (u8, u8, u8),
    pub(crate) sky_blue: (u8, u8, u8),
    pub(crate) blue: (u8, u8, u8),
    pub(crate) dark_blue: (u8, u8, u8),
    pub(crate) purple: (u8, u8, u8),
}

impl Palette {
    pub(crate) fn new() -> &'static Self {
        PALETTE.get_or_init(|| Palette {
            bg: base(0.0),
            light_bg: base(0.03),
            lighter_bg: base(0.15),
            dark_fg: base(0.3),
            fg: base(0.75),
            bright_fg: base(1.0),
            red: oklch(DARK_LIGHTNESS, HIGH_CHROMA, 25.0),
            orange: oklch(BRIGHT_LIGHTNESS, VERY_HIGH_CHROMA, 60.0),
            dark_orange: oklch(MEDIUM_LIGHTNESS, HIGH_CHROMA, 60.0),
            yellow: oklch(VERY_BRIGHT_LIGHTNESS, MEDIUM_CHROMA, 100.0),
            olive: oklch(DARK_LIGHTNESS, HIGH_CHROMA, 120.0),
            green: oklch(DARK_LIGHTNESS, MEDIUM_CHROMA, 140.0),
            teal: oklch(DARK_LIGHTNESS, LOW_CHROMA, 200.0),
            sky_blue: oklch(MEDIUM_LIGHTNESS, VERY_LOW_CHROMA, 220.0),
            blue: oklch(MEDIUM_LIGHTNESS, MEDIUM_CHROMA, 250.0),
            dark_blue: oklch(DARK_LIGHTNESS, LOW_CHROMA, 240.0),
            purple: oklch(DARK_LIGHTNESS, LOW_CHROMA, 265.0),
        })
    }
}

fn base(value: f32) -> (u8, u8, u8) {
    let lightness = lerp(value, 0.24..0.98);
    let chroma = lerp(value, 0.0..0.029);
    oklch(lightness, chroma, 100.0)
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
