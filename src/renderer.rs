use skia_safe::{
    textlayout::{FontCollection, ParagraphBuilder, ParagraphStyle, TextStyle},
    FontMgr, Point,
};

pub fn render_frame(canvas: &skia_safe::canvas::Canvas) {
    let mut font = FontCollection::new();
    font.set_default_font_manager(FontMgr::default(), None);

    let mut text_style = TextStyle::new();
    let text_style = text_style.set_color(0xff_ff0000).set_font_size(30.0);
    let mut style = ParagraphStyle::new();
    let style = style.set_text_style(text_style);

    let mut paragraph = ParagraphBuilder::new(&style, &font)
        .add_text("Hello, World!")
        .build();

    paragraph.layout(300.0);
    paragraph.paint(canvas, Point::new(100.0, 100.0));
}
