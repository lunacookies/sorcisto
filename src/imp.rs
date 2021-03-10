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
}

fn syntax_highlighting(builder: &mut ThemeBuilder, palette: &Palette) {
    builder.add_rules(
        &[Semantic("keyword"), Semantic("builtinType")],
        palette.purple(),
    );

    builder.add_rules(
        &[
            Semantic("keyword.controlFlow"),
            Semantic("selfKeyword"),
            Semantic("operator"),
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
            Semantic("enum"),
            Semantic("union"),
            Semantic("typeAlias"),
            Semantic("interface"),
            Semantic("typeParameter"),
        ],
        palette.sky_blue(),
    );

    builder.add_rule(Semantic("enumMember"), palette.dark_sky_blue());

    builder.add_rules(
        &[
            Semantic("macro"),
            Semantic("attribute"),
            Semantic("*.attribute"),
        ],
        palette.teal(),
    );

    builder.add_rule(Semantic("string"), palette.green());

    builder.add_rules(
        &[Semantic("escapeSequence"), Semantic("formatSpecifier")],
        palette.olive(),
    );

    builder.add_rules(
        &[
            Semantic("boolean"),
            Semantic("*.constant"),
            Semantic("variable.static"),
        ],
        palette.orange(),
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
