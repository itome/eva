use skia_safe::{Paint, Point};

pub fn render_frame(canvas: &skia_safe::canvas::Canvas) {
    canvas.draw_circle(
        Point::new(100.0, 100.0),
        40.0,
        &Paint::default().set_color(0xff_8bc34a),
    );

    canvas.draw_circle(
        Point::new(200.0, 100.0),
        40.0,
        &Paint::default().set_color(0xff_ffeb3b),
    );

    canvas.draw_circle(
        Point::new(300.0, 100.0),
        40.0,
        &Paint::default().set_color(0xff_f44336),
    );
}
