use crate::palette::{BaseScale, Palette};
use mottle::dsl::{s, tm, FontStyle, ThemeBuilder};

pub(crate) fn add_rules(t: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(t, palette);
    syntax_highlighting(t, palette);
}

fn workspace_colors(t: &mut ThemeBuilder, palette: &Palette) {
    t.w(["editor.background"], palette.base(BaseScale::Bg));
    t.w(["editor.foreground", "foreground"], palette.base(BaseScale::Fg));

    t.w(["editor.lineHighlightBackground"], palette.base(BaseScale::LightBg));

    t.w(
        ["statusBar.background", "statusBar.debuggingBackground", "statusBar.noFolderBackground"],
        palette.base(BaseScale::LighterBg),
    );
    t.w(["statusBar.foreground"], palette.base(BaseScale::Fg));

    t.w(["editor.selectionBackground"], palette.base(BaseScale::LighterBg));

    t.w(["editorCodeLens.foreground"], palette.base(BaseScale::DarkFg));

    t.w(["rust_analyzer.inlayHints.foreground"], palette.base(BaseScale::DarkFg));
}

fn syntax_highlighting(t: &mut ThemeBuilder, palette: &Palette) {
    t.a([s("keyword"), s("operator"), s("arithmetic"), s("logical"), s("bitwise")], palette.blue());

    t.a([s("function"), s("method")], palette.yellow());

    t.a(
        [s("type"), s("class"), s("struct"), s("union"), s("typeAlias"), s("builtinType")],
        palette.purple(),
    );
    t.a([s("interface")], palette.pale_blue());
    t.a([s("typeParameter")], palette.orange());

    t.a([s("enum")], (palette.sky_blue(), FontStyle::Italic));
    t.a([s("enumMember")], (palette.dark_sky_blue(), FontStyle::Italic));

    t.a([s("macro"), s("attribute"), s("*.attribute")], palette.teal());

    t.a([s("lifetime")], (palette.olive(), FontStyle::Italic));

    t.a([s("string")], palette.green());

    t.a([s("escapeSequence"), s("formatSpecifier")], palette.olive());

    t.a([s("property")], palette.dark_blue());

    t.a([s("number"), s("boolean"), s("characterLiteral")], palette.orange());
    t.a([s("*.constant"), s("variable.static"), s("constParameter")], palette.dark_orange());

    t.a([s("unresolvedReference")], palette.red());

    t.a([s("*.unsafe")], (palette.red(), FontStyle::Bold));

    t.a([s("comment")], (palette.base(BaseScale::BrightFg), FontStyle::Italic));
    t.a([s("comment.documentation")], palette.base(BaseScale::BrightFg));

    t.a([tm("markup.heading")], palette.base(BaseScale::BrightFg));
}
