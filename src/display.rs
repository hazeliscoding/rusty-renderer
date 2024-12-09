extern crate sdl2;

use sdl2::{
    render::Canvas,
    video::{Window, WindowBuildError},
};

/// Width of the application window.
pub const WINDOW_WIDTH: u32 = 800;
/// Height of the application window.
pub const WINDOW_HEIGHT: u32 = 600;
/// Target frames per second for the application.
pub const FRAMES_PER_SECOND: u32 = 30;
/// Title of the application window.
const WINDOW_TITLE: &str = "Rusty Renderer";

/// Initializes an SDL2 window.
///
/// # Arguments
/// - `sdl_context`: The SDL2 context to create the window with.
///
/// # Returns
/// A result containing the SDL2 `Window` or an error if creation fails.
pub fn initialize_window(sdl_context: &sdl2::Sdl) -> Result<Window, WindowBuildError> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .borderless() // Window without borders for a more immersive experience.
        .build()?;
    Ok(window)
}

/// Clears the color buffer by resetting all pixel values to zero (black).
///
/// # Arguments
/// - `color_buffer`: The mutable reference to the color buffer.
pub fn clear_color_buffer(color_buffer: &mut Vec<u8>) {
    *color_buffer = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT * 3) as usize];
}

/// Renders the contents of the color buffer onto the SDL canvas.
///
/// # Arguments
/// - `canvas`: The mutable reference to the SDL canvas for rendering.
/// - `color_buffer`: The color buffer containing pixel data.
pub fn render_color_buffer(canvas: &mut Canvas<Window>, color_buffer: &Vec<u8>) {
    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGB24, // Pixel format for RGB.
            WINDOW_WIDTH,
            WINDOW_HEIGHT,
        )
        .unwrap();

    // Update the texture with color buffer data.
    texture
        .update(None, &color_buffer, (WINDOW_WIDTH * 3) as usize)
        .unwrap();
    canvas.copy(&texture, None, None).unwrap(); // Copy the texture to the canvas.
}
