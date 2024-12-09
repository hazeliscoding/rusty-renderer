// This file contains the code for the display module, responsible for window management,
// rendering, and drawing functions.

extern crate sdl2;
use sdl2::render::Canvas;
use sdl2::video::{Window, WindowBuildError};

use crate::vector::Vec2;

/// The width of the application window in pixels.
pub const WINDOW_WIDTH: u32 = 800;
/// The height of the application window in pixels.
pub const WINDOW_HEIGHT: u32 = 600;
/// The target frame rate for the application.
pub const FRAMES_PER_SECOND: u32 = 30;

/// The title of the application window.
const WINDOW_TITLE: &str = "Renderer Learning";

/// Initializes the SDL2 window.
///
/// # Arguments
/// - `sdl_context`: The SDL2 context for creating the window.
///
/// # Returns
/// A result containing the SDL2 `Window` or an error if window creation fails.
pub fn initialize_window(sdl_context: &sdl2::Sdl) -> Result<Window, WindowBuildError> {
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(WINDOW_TITLE, WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .borderless() // Window without borders for a clean look.
        .build()?;
    Ok(window)
}

/// Clears the color buffer by resetting all pixel values to black.
///
/// # Arguments
/// - `color_buffer`: A mutable reference to the color buffer.
pub fn clear_color_buffer(color_buffer: &mut Vec<u8>) {
    *color_buffer = vec![0; (WINDOW_WIDTH * WINDOW_HEIGHT * 3) as usize];
}

/// Renders the contents of the color buffer onto the SDL canvas.
///
/// # Arguments
/// - `canvas`: The mutable SDL canvas for rendering.
/// - `color_buffer`: The buffer containing pixel data to be rendered.
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

/// Draws a single pixel at a specified position in the color buffer.
///
/// # Arguments
/// - `color_buffer`: A mutable reference to the color buffer.
/// - `x`: The x-coordinate of the pixel.
/// - `y`: The y-coordinate of the pixel.
/// - `color`: The color of the pixel (RGBA).
pub fn draw_pixel(color_buffer: &mut [u8], x: u32, y: u32, color: sdl2::pixels::Color) {
    if x >= WINDOW_WIDTH || y >= color_buffer.len() as u32 / (WINDOW_WIDTH * 3) {
        return; // Ignore out-of-bound pixels.
    }
    let pixel_index = (y * WINDOW_WIDTH + x) as usize * 3;
    color_buffer[pixel_index] = color.r;
    color_buffer[pixel_index + 1] = color.g;
    color_buffer[pixel_index + 2] = color.b;
}

/// Draws a filled rectangle on the color buffer.
///
/// # Arguments
/// - `color_buffer`: A mutable reference to the color buffer.
/// - `x`: The x-coordinate of the top-left corner of the rectangle.
/// - `y`: The y-coordinate of the top-left corner of the rectangle.
/// - `width`: The width of the rectangle.
/// - `height`: The height of the rectangle.
/// - `color`: The color of the rectangle (RGBA).
pub fn draw_rect(
    color_buffer: &mut Vec<u8>,
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    color: sdl2::pixels::Color,
) {
    if x >= WINDOW_WIDTH || y >= WINDOW_HEIGHT {
        return; // Ignore out-of-bound rectangles.
    }

    for row in y..y + height {
        for col in x..x + width {
            draw_pixel(color_buffer, col, row, color);
        }
    }
}

/// Draws a grid on the color buffer.
///
/// # Arguments
/// - `color_buffer`: A mutable reference to the color buffer.
/// - `size`: The size of each grid cell in pixels.
#[allow(dead_code)] // Suppresses warnings for unused function during development.
pub fn draw_grid(color_buffer: &mut Vec<u8>, size: usize) {
    for y in (0..WINDOW_HEIGHT).step_by(size) {
        for x in (0..WINDOW_WIDTH).step_by(size) {
            draw_pixel(
                color_buffer,
                x,
                y,
                sdl2::pixels::Color::RGBA(255, 255, 255, 255), // White grid lines.
            );
        }
    }
}

/// Draws a triangle by connecting its vertices with lines.
///
/// # Arguments
/// - `color_buffer`: A mutable reference to the color buffer.
/// - `points`: An array of three 2D points (`Vec2`) representing the vertices of the triangle.
/// - `color`: The color of the triangle (RGBA).
pub fn draw_triangle(color_buffer: &mut Vec<u8>, points: [Vec2; 3], color: sdl2::pixels::Color) {
    for i in 0..3 {
        let p0 = points[i];
        let p1 = points[(i + 1) % 3]; // Connect the last point to the first.

        draw_line(
            color_buffer,
            p0.x as i32,
            p0.y as i32,
            p1.x as i32,
            p1.y as i32,
            color,
        );
    }
}

/// Draws a line using the Bresenham's line algorithm.
///
/// # Arguments
/// - `color_buffer`: A mutable reference to the color buffer.
/// - `x0`: The x-coordinate of the starting point.
/// - `y0`: The y-coordinate of the starting point.
/// - `x1`: The x-coordinate of the ending point.
/// - `y1`: The y-coordinate of the ending point.
/// - `color`: The color of the line (RGBA).
pub fn draw_line(
    color_buffer: &mut Vec<u8>,
    mut x0: i32,
    mut y0: i32,
    x1: i32,
    y1: i32,
    color: sdl2::pixels::Color,
) {
    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    let mut e2;

    loop {
        draw_pixel(color_buffer, x0 as u32, y0 as u32, color); // Draw the current point.

        if x0 == x1 && y0 == y1 {
            break; // Stop when the end point is reached.
        }

        e2 = 2 * err;

        if e2 > dy {
            err += dy;
            x0 += sx;
        }

        if e2 < dx {
            err += dx;
            y0 += sy;
        }
    }
}
