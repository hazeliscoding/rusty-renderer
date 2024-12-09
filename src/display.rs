extern crate sdl2;

use sdl2::{render::Canvas, video::{Window, WindowBuildError}};

pub const WINDOW_WIDTH: u32 = 800;
pub const WINDOW_HEIGHT: u32 = 600;
pub const FRAMES_PER_SECOND: u32 = 30;

const WINDOW_TITLE: &str = "Rusty Renderer";

pub fn initialize_window(sdl_context: &sdl2::Sdl) -> Result<Window, WindowBuildError> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .borderless()
        .build()?;
    Ok(window)
}

pub fn clear_color_buffer(color_buffer: &mut Vec<u8>) {
    *color_buffer = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT * 3) as usize];
}

pub fn render_color_buffer(canvas: &mut Canvas<Window>, color_buffer: &Vec<u8>) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24,
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        .unwrap();

    texture
        .update(None, &color_buffer, (WINDOW_WIDTH * 3) as usize)
        .unwrap();
    canvas.copy(&texture, None, None).unwrap();
}