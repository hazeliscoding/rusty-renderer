extern crate sdl2;

use display::FRAMES_PER_SECOND;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;
mod display;

struct Renderer {
    sdl_context: Sdl,
    canvas: Canvas<Window>,
    color_buffer: Vec<u8>,
    is_running: bool,
}

impl Renderer {
    pub fn new(window: Window, sdl_context: Sdl) -> Renderer {
        let canvas = window
            .into_canvas()
            .present_vsync()
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

    pub fn process_input(&mut self) {
        let mut events = self.sdl_context.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false,
                _ => {}
            }
        }
    }

    pub fn update(&mut self) {}

    pub fn render(&mut self) {
        display::render_color_buffer(&mut self.canvas, &self.color_buffer);
        display::clear_color_buffer(&mut self.color_buffer);
        self.canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SECOND));
    }
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let window = display::initialize_window(&sdl_context);
    let mut renderer = Renderer::new(window.unwrap(), sdl_context);

    while renderer.is_running {
        renderer.process_input();
        renderer.update();
        renderer.render();
    }
}
