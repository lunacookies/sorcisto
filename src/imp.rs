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
    t.a([s("keyword"), s("builtinType")], p.purple);
    t.a([s("keyword.controlFlow"), s("selfKeyword")], p.blue);

    t.a([s("function"), s("method")], p.yellow);

    t.a(
        [
            s("type"),
            s("class"),
            s("struct"),
            s("enum"),
            s("union"),
            s("typeAlias"),
            s("interface"),
            s("typeParameter"),
        ],
        p.sky_blue,
    );

    t.a([s("macro"), s("derive")], p.teal);

    t.a([s("lifetime"), s("attribute")], p.olive);

    t.a([s("string")], p.green);

    t.a([s("escapeSequence"), s("formatSpecifier")], p.olive);

    t.a([s("property")], p.dark_blue);

    t.a([s("number"), s("boolean"), s("character")], p.orange);
    t.a([s("*.constant"), s("variable.static"), s("constParameter")], p.dark_orange);

    t.a([s("unresolvedReference")], p.red);

    t.a([s("*.unsafe")], (p.red, FontStyle::Bold));

    t.a([s("comment")], p.bright_fg);

    t.a([tm("markup.heading")], p.bright_fg);
}
