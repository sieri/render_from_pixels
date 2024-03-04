use render_from_pixels::canvas::Canvas;
use render_from_pixels::Color;

fn main() {
    let builder = render_from_pixels::window::WindowBuilder::new()
        .with_pixel_size(200, 100)
        .with_scale_size(5f64);

    render_from_pixels::window::window(builder, |canvas, mouse_state| {
        if mouse_state.clicked {
            println!("{:?}", mouse_state);
            canvas.change_pixel(
                mouse_state.x,
                mouse_state.y,
                Color {
                    r: 1.0,
                    g: 0.0,
                    b: 0.0,
                    a: 1.0,
                },
            )
        }
    })
}
