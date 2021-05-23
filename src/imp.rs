use crate::palette::{BaseScale, Palette};
use mottle::style::FontStyle;
use mottle::theme::Scope::*;
use mottle::theme::ThemeBuilder;

pub(crate) fn add_rules(builder: &mut ThemeBuilder, palette: &Palette) {
    workspace_colors(builder, palette);
    syntax_highlighting(builder, palette);
}

fn workspace_colors(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_workspace_rule("editor.background", palette.base(BaseScale::Bg));
    builder.add_workspace_rules(
        &["editor.foreground", "foreground"],
        palette.base(BaseScale::Fg),
    );

    builder.add_workspace_rule(
        "editor.lineHighlightBackground",
        palette.base(BaseScale::LightBg),
    );

    builder.add_workspace_rules(
        &[
            "statusBar.background",
            "statusBar.debuggingBackground",
            "statusBar.noFolderBackground",
        ],
        palette.base(BaseScale::LighterBg),
    );
    builder.add_workspace_rule("statusBar.foreground", palette.base(BaseScale::DarkFg));

    builder.add_workspace_rule(
        "editor.selectionBackground",
        palette.base(BaseScale::LighterBg),
    );

    builder.add_workspace_rule(
        "rust_analyzer.inlayHints.foreground",
        palette.base(BaseScale::DarkFg),
    );
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[
            Semantic("keyword"),
            Semantic("operator"),
            Semantic("arithmetic"),
            Semantic("logical"),
            Semantic("bitwise"),
        ],
        palette.blue(),
    );

    builder.add_rules(
        &[Semantic("function"), Semantic("method")],
        palette.yellow(),
    );

    builder.add_rules(
        &[
            Semantic("type"),
            Semantic("class"),
            Semantic("struct"),
            Semantic("union"),
            Semantic("typeAlias"),
        ],
        palette.pale_blue(),
    );
    builder.add_rule(Semantic("interface"), palette.light_purple());
    builder.add_rule(Semantic("typeParameter"), palette.orange());
    builder.add_rule(Semantic("builtinType"), palette.slate());

    builder.add_rule(Semantic("enum"), (palette.sky_blue(), FontStyle::Italic));
    builder.add_rule(
        Semantic("enumMember"),
        (palette.dark_sky_blue(), FontStyle::Italic),
    );

    builder.add_rules(
        &[
            Semantic("macro"),
            Semantic("attribute"),
            Semantic("*.attribute"),
        ],
        palette.teal(),
    );

    builder.add_rule(Semantic("lifetime"), (palette.teal(), FontStyle::Italic));

    builder.add_rule(Semantic("string"), palette.green());

    builder.add_rules(
        &[Semantic("escapeSequence"), Semantic("formatSpecifier")],
        palette.olive(),
    );

    builder.add_rule(Semantic("namespace"), palette.light_green());

    builder.add_rule(Semantic("property"), palette.indigo());

    builder.add_rule(Semantic("boolean"), palette.orange());
    builder.add_rules(
        &[Semantic("*.constant"), Semantic("variable.static")],
        (palette.orange(), FontStyle::Italic),
    );
    builder.add_rules(
        &[Semantic("number"), Semantic("characterLiteral")],
        palette.dark_orange(),
    );

    builder.add_rule(
        Semantic("comment"),
        (palette.base(BaseScale::BrightFg), FontStyle::Italic),
    );
    builder.add_rule(
        Semantic("comment.documentation"),
        palette.base(BaseScale::BrightFg),
    );

    builder.add_rule(
        Textmate("markup.heading"),
        palette.base(BaseScale::BrightFg),
    );
}
