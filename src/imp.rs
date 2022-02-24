use crate::palette::Palette;
use mottle::dsl::{s, tm, FontStyle, ThemeBuilder};

pub(crate) fn add_rules(t: &mut ThemeBuilder, p: &Palette) {
    workspace_colors(t, p);
    syntax_highlighting(t, p);
}

fn workspace_colors(t: &mut ThemeBuilder, p: &Palette) {
    t.w(["editor.background"], p.bg);
    t.w(["editor.foreground", "foreground"], p.fg);

    t.w(["editor.lineHighlightBackground"], p.light_bg);

    t.w(
        ["statusBar.background", "statusBar.debuggingBackground", "statusBar.noFolderBackground"],
        p.lighter_bg,
    );
    t.w(["statusBar.foreground"], p.fg);

    t.w(["editor.selectionBackground"], p.lighter_bg);

    t.w(["editorCodeLens.foreground"], p.dark_fg);

    t.w(["rust_analyzer.inlayHints.foreground"], p.dark_fg);
}

fn syntax_highlighting(t: &mut ThemeBuilder, p: &Palette) {
    t.a([s("keyword"), s("operator"), s("arithmetic"), s("logical"), s("bitwise")], p.blue);

    t.a([s("function"), s("method")], p.yellow);

    t.a(
        [s("type"), s("class"), s("struct"), s("union"), s("typeAlias"), s("builtinType")],
        p.purple,
    );
    t.a([s("interface")], p.pale_blue);
    t.a([s("typeParameter")], p.orange);

    t.a([s("enum")], p.sky_blue);
    t.a([s("enumMember")], p.dark_sky_blue);

    t.a([s("macro"), s("attribute"), s("*.attribute")], p.teal);

    t.a([s("lifetime")], p.olive);

    t.a([s("string")], p.green);

    t.a([s("escapeSequence"), s("formatSpecifier")], p.olive);

    t.a([s("property")], p.dark_blue);

    t.a([s("number"), s("boolean"), s("characterLiteral")], p.orange);
    t.a([s("*.constant"), s("variable.static"), s("constParameter")], p.dark_orange);

    t.a([s("unresolvedReference")], p.red);

    t.a([s("*.unsafe")], (p.red, FontStyle::Bold));

    t.a([s("comment")], p.bright_fg);

    t.a([tm("markup.heading")], p.bright_fg);
}
