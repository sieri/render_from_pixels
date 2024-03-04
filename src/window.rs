use pixels::SurfaceTexture;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use crate::canvas::WindowCanvas;

///blocking method running the window, panics if not able to initialize
pub fn window<F>(builder: WindowBuilder, update: F)
where
    F: Fn(&mut WindowCanvas, &MouseState),
{
    let scale = builder.scale.unwrap_or_else(|| 1f64);
    println!("Scale{}", scale);
    let (pixel_size, logical_size) = match builder {
        WindowBuilder {
            pixel_size: Some(pixel_size),
            window_size: None,
            ..
        } => {
            let logical_size = ((pixel_size.0 as f64) * scale, (pixel_size.1 as f64) * scale);
            (pixel_size, logical_size)
        }
        WindowBuilder {
            pixel_size: None,
            window_size: Some(logical_size),
            ..
        } => {
            let pixel_size = (
                (logical_size.0 / scale) as usize,
                (logical_size.1 / scale) as usize,
            );
            (pixel_size, logical_size)
        }
        WindowBuilder {
            pixel_size: None,
            window_size: None,
            ..
        } => {
            let pixel_size = (640, 480);
            let logical_size = ((pixel_size.0 as f64) * scale, (pixel_size.1) as f64);
            (pixel_size, logical_size)
        }
        WindowBuilder {
            pixel_size: Some(_),
            window_size: Some(_),
            ..
        } => {
            panic!("Cannot define both window size and pixel size")
        }
    };
    println!("logical_size {:?}", logical_size);
    let title = builder
        .title
        .unwrap_or("render_from_pixel canvas".to_string());

    let size = PhysicalSize::new(logical_size.0, logical_size.1);

    if let Ok(event_loop) = EventLoop::new() {
        let winit_window = winit::window::WindowBuilder::new()
            .with_fullscreen(None)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .with_resizable(false)
            .with_title(title)
            .build(&event_loop)
            .unwrap();

        let surface_texture =
            SurfaceTexture::new(size.width as u32, size.height as u32, &winit_window);
        let canvas = WindowCanvas::new(pixel_size.0, pixel_size.1, surface_texture);

        looping(winit_window, event_loop, canvas, update).expect("Loop error");
    } else {
        panic!("Couldn't initialize event loop");
    }
}

fn looping<F>(
    winit_window: Window,
    event_loop: EventLoop<()>,
    mut canvas: WindowCanvas,
    update: F,
) -> Result<(), EventLoopError>
where
    F: Fn(&mut WindowCanvas, &MouseState),
{
    let mut mouse_state = MouseState {
        x: 0,
        y: 0,
        clicked: false,
    };

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                elwt.exit();
            }
            Event::AboutToWait => {
                // Application update code.
                winit_window.request_redraw();
            }
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
                if mouse_state.clicked {}

                update(&mut canvas, &mouse_state);

                if let Err(err) = canvas.pixels.render() {
                    println!("pixels.render {:?}", err);
                    elwt.exit();
                }
            }
            Event::WindowEvent {
                event:
                    WindowEvent::MouseInput {
                        state: pressed,
                        button: MouseButton::Left,
                        ..
                    },
                ..
            } => match pressed {
                ElementState::Pressed => {
                    mouse_state.clicked = true;
                }

                ElementState::Released => {
                    mouse_state.clicked = false;
                }
            },
            Event::WindowEvent {
                event: WindowEvent::CursorMoved { position: pos, .. },
                ..
            } => {
                let cursor_position: (f32, f32) = pos.into();
                if let Ok((x, y)) = canvas.pixels.window_pos_to_pixel(cursor_position) {
                    mouse_state.x = x;
                    mouse_state.y = y;
                }
            }
            _ => (),
        }
    })
}

pub struct WindowBuilder {
    pixel_size: Option<(usize, usize)>,
    window_size: Option<(f64, f64)>,
    scale: Option<f64>,
    title: Option<String>,
    control_flow: Option<ControlFlow>,
}

impl WindowBuilder {
    pub fn new() -> Self {
        WindowBuilder {
            pixel_size: None,
            window_size: None,
            scale: None,
            title: None,
            control_flow: None,
        }
    }

    pub fn with_pixel_size(mut self, x: usize, y: usize) -> Self {
        self.pixel_size = Some((x, y));
        self
    }
    pub fn with_window_size(mut self, x: f64, y: f64) -> Self {
        self.window_size = Some((x, y));
        self
    }
    pub fn with_scale_size(mut self, scale: f64) -> Self {
        self.scale = Some(scale);
        self
    }

    pub fn with_title_size(mut self, title: String) -> Self {
        self.title = Some(title);
        self
    }

    pub fn with_control_flow(mut self, title: ControlFlow) -> Self {
        self.control_flow = Some(title);
        self
    }
}

#[derive(Debug)]
pub struct MouseState {
    pub x: usize,
    pub y: usize,
    pub clicked: bool,
}
