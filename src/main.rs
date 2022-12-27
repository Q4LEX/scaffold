use tracing::{trace, Level};
use winit::{
    event::{Event, WindowEvent},
    event_loop::EventLoop,
    window::WindowBuilder,
};

use crate::client::Client;

mod client;
mod input;
mod settings;

fn main() {
    // TRACING / LOGGING
    let subscriber = tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // WINIT
    let event_loop = EventLoop::new();
    trace!("Creating main window");
    let window = WindowBuilder::new()
        .with_title("Scaffold")
        .build(&event_loop)
        .expect("Unable to create window");

    trace!("Creating client");
    let mut client = Client::new();

    trace!("Running main event_loop");
    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::MainEventsCleared => {
                client.tick();
            }
            Event::WindowEvent { event, window_id } => {
                if window_id == window.id() && !client.input_helper.add_window_event(&event) {
                    match event {
                        WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                            trace!("Setting control_flow to exit due to event: {:?}", event);
                            control_flow.set_exit();
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    });
}
