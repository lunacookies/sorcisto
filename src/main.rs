mod imp;
mod palette;

use mottle::dsl::ThemeBuilder;

fn main() -> anyhow::Result<()> {
    let palette = palette::Palette::new();

    let mut b = ThemeBuilder::default();
    imp::add_rules(&mut b, palette);
    mottle::save_theme(&b.build("Sorcisto"))?;

    Ok(())
}
