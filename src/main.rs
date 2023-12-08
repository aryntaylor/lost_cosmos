use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const DEFAULT_WIDTH: u32 = 1920;
const DEFAULT_HEIGHT: u32 = 1080;

fn create_window(title: &str) {
    let event_loop = EventLoop::new().unwrap();
    let window = WindowBuilder::new()
    .with_title(title)
    .with_resizable(true)
    .with_transparent(false)
    .with_inner_size(winit::dpi::PhysicalSize {
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
    })
    .build(&event_loop)
    .unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let _ = event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("The close button was pressed; stopping");
                elwt.exit();
            },
            Event::AboutToWait => {
                window.request_redraw();
            },
            Event::WindowEvent {
                event: WindowEvent::RedrawRequested,
                ..
            } => {
            },
            _ => ()
        }
    });
}

fn main() {
    create_window("Lost Cosmos");
}
