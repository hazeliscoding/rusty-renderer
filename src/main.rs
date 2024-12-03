extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

fn main() {
    // Initialize SDL2
    let sdl_context = sdl2::init().expect("Failed to initialize SDL2");
    let video_subsystem = sdl_context.video().expect("Failed to initialize video subsystem");

    // Create a window
    let window = video_subsystem
        .window("SDL2 Window", 800, 600)
        .position_centered()
        .build()
        .expect("Failed to create window");

    // Event pump to handle events
    let mut event_pump = sdl_context.event_pump().expect("Failed to get event pump");

    // Main loop
    'running: loop {
        // Handle events
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        // Simulate frame delay
        std::thread::sleep(Duration::from_millis(16));
    }
}
