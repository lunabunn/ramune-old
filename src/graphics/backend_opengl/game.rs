use super::gl;
use super::graphics::GLGraphics;
use crate::graphics::GraphicsBackends;
use crate::Event as GameEvent;
use crate::{Context, GameOptions};
use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;

pub fn run(options: GameOptions, mut callback: impl FnMut(GameEvent) + 'static) {
    let aspect_ratio = options.size.0 as f32 / options.size.1 as f32;

    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(options.title)
        .with_inner_size(LogicalSize::new(options.size.0, options.size.1));
    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    gl::load(|s| windowed_context.get_proc_address(s) as *const std::ffi::c_void);
    let mut context = Context {
        graphics: GraphicsBackends::GL(GLGraphics::new(&options)),
    };

    callback(GameEvent::Init(&mut context));
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    let (width, height): (u32, u32) = physical_size
                        .to_logical::<u32>(windowed_context.window().scale_factor())
                        .into();
                    let mut new_x = 0.0;
                    let mut new_y = 0.0;
                    let mut new_width = width as f32;
                    let mut new_height = height as f32;
                    let ratio = width as f32 / height as f32;
                    if ratio < aspect_ratio {
                        new_height = (1.0 / aspect_ratio) * width as f32;
                        new_y = (height as f32 - new_height) / 2.0;
                    } else if ratio > aspect_ratio {
                        new_width = aspect_ratio * height as f32;
                        new_x = (width as f32 - new_width) / 2.0;
                    }

                    gl::viewport(
                        new_x.round() as i32,
                        new_y.round() as i32,
                        new_width.round() as i32,
                        new_height.round() as i32,
                    );
                    callback(GameEvent::WindowResized(&mut context));
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                callback(GameEvent::Update(&mut context));
                callback(GameEvent::Draw(&mut context));
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
