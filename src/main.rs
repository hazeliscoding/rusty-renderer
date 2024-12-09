extern crate sdl2;

use display::FRAMES_PER_SECOND;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;
mod display;

/// Main renderer struct responsible for managing SDL context, rendering, and state.
struct Renderer {
    /// SDL2 context for managing SDL features like events and rendering.
    sdl_context: Sdl,
    /// Canvas for drawing graphical content on the window.
    canvas: Canvas<Window>,
    /// Color buffer holding pixel data for rendering.
    color_buffer: Vec<u8>,
    /// Application running state.
    is_running: bool,
}

impl Renderer {
    /// Creates a new instance of the `Renderer`.
    ///
    /// # Arguments
    /// - `window`: The SDL2 window to render to.
    /// - `sdl_context`: The SDL2 context for initializing SDL systems.
    pub fn new(window: Window, sdl_context: Sdl) -> Renderer {
        let canvas = window
            .into_canvas()
            .present_vsync() // Synchronize frame rendering with the monitor's refresh rate.
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let color_buffer = vec![0; (display::WINDOW_WIDTH * display::WINDOW_HEIGHT * 3) as usize];

        Renderer {
            sdl_context,
            canvas,
            color_buffer,
            is_running: true,
        }
    }

    /// Processes user input and handles SDL events.
    pub fn process_input(&mut self) {
        let mut events = self.sdl_context.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false, // Exit on quit event.
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false, // Exit when Escape key is pressed.
                _ => {}
            }
        }
    }

    /// Updates the application's state (currently a placeholder).
    pub fn update(&mut self) {
        // Update game logic or animation here in future.
    }

    /// Renders the color buffer to the canvas and updates the display.
    pub fn render(&mut self) {
        display::render_color_buffer(&mut self.canvas, &self.color_buffer);
        display::clear_color_buffer(&mut self.color_buffer); // Clear the color buffer after rendering.
        self.canvas.present(); // Swap the buffers (double buffering).
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SECOND)); // Cap frame rate.
    }
}

/// Application entry point.
pub fn main() {
    // Initialize SDL2 context.
    let sdl_context = sdl2::init().unwrap();

    // Initialize the application window.
    let window = display::initialize_window(&sdl_context);

    // Create the renderer.
    let mut renderer = Renderer::new(window.unwrap(), sdl_context);

    // Main application loop.
    while renderer.is_running {
        renderer.process_input();
        renderer.update();
        renderer.render();
    }
}
