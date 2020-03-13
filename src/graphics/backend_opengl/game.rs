use glutin::dpi::LogicalSize;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{ControlFlow, EventLoop};
use glutin::window::WindowBuilder;
use glutin::ContextBuilder;
use crate::GameOptions;
use crate::Event as GameEvent;

pub fn run(options: GameOptions, mut callback: impl FnMut(GameEvent) + 'static) {
    let el = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title(options.title)
        .with_inner_size(LogicalSize::new(options.size.0, options.size.1));
    let windowed_context = ContextBuilder::new()
        .with_vsync(true)
        .build_windowed(wb, &el)
        .unwrap();
    let windowed_context = unsafe { windowed_context.make_current().unwrap() };

    callback(GameEvent::Init());
    el.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    windowed_context.resize(physical_size);
                    callback(GameEvent::WindowResized());
                }
                WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            },
            Event::MainEventsCleared => {
                callback(GameEvent::Update());
                callback(GameEvent::Draw());
                windowed_context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}