extern crate sdl2;

use display::FRAMES_PER_SECOND;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::Sdl;
use std::time::Duration;
use vector::Vec3;

mod display;
mod mesh;
mod triangle;
mod vector;

/// The `Renderer` struct is responsible for managing the rendering process,
/// including initializing the SDL context, projecting 3D points to 2D,
/// handling user input, updating object transformations, and rendering the frame.
struct Renderer {
    /// SDL2 context for handling input, events, and graphics.
    sdl_context: Sdl,
    /// Canvas for rendering graphics onto the window.
    canvas: Canvas<Window>,
    /// Color buffer used for rendering pixel data.
    color_buffer: Vec<u8>,
    /// Flag indicating whether the application is running.
    is_running: bool,
    /// Field of view factor for projecting 3D points onto a 2D plane.
    fov_factor: f32,
    /// Camera position in 3D space.
    camera_position: Vec3,
    /// List of triangles to render in the current frame.
    triangles_to_render: Vec<triangle::Triangle>,
    /// The 3D mesh being rendered.
    mesh: mesh::Mesh,
}

impl Renderer {
    /// Creates a new `Renderer` instance.
    ///
    /// # Arguments
    /// - `window`: The SDL2 window for rendering.
    /// - `sdl_context`: The SDL2 context for managing SDL systems.
    ///
    /// # Returns
    /// A fully initialized `Renderer`.
    pub fn new(window: Window, sdl_context: Sdl) -> Renderer {
        let canvas = window
            .into_canvas()
            .present_vsync() // Synchronize rendering with the display's refresh rate.
            .build()
            .map_err(|e| e.to_string())
            .unwrap();

        let color_buffer = vec![0; (display::WINDOW_WIDTH * display::WINDOW_HEIGHT * 3) as usize];
        let mesh = mesh::Mesh::load_from_file("./assets/f22.obj");

        Renderer {
            sdl_context,
            canvas,
            color_buffer,
            is_running: true,
            fov_factor: 700.0, // Field of view scaling factor for projection.
            camera_position: Vec3::new(0.0, 5.0, -5.0), // Initial camera position.
            triangles_to_render: Vec::new(),
            mesh,
        }
    }

    /// Projects a 3D point onto a 2D plane using perspective projection.
    ///
    /// # Arguments
    /// - `point`: A 3D point (`Vec3`) to project.
    ///
    /// # Returns
    /// A 2D point (`Vec2`) representing the projected coordinates.
    pub fn project(&mut self, point: vector::Vec3) -> vector::Vec2 {
        vector::Vec2 {
            x: (self.fov_factor * point.x) / point.z,
            y: (self.fov_factor * point.y) / point.z,
        }
    }

    /// Processes user input and handles events such as quitting or camera movement.
    pub fn process_input(&mut self) {
        let mut events = self.sdl_context.event_pump().unwrap();
        for event in events.poll_iter() {
            match event {
                Event::Quit { .. } => self.is_running = false, // Exit the application.
                Event::MouseWheel { y, .. } => {
                    self.camera_position.z += y as f32; // Adjust camera zoom.
                }
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => self.is_running = false, // Exit on Escape key.
                _ => {}
            }
        }
    }

    /// Updates the state of the mesh and prepares triangles for rendering.
    pub fn update(&mut self) {
        // Rotate the mesh slightly in each axis.
        self.mesh.rotation.x += 0.02;
        self.mesh.rotation.y += 0.02;
        self.mesh.rotation.z += 0.02;

        let num_faces = self.mesh.faces.len();
        for i in 0..num_faces {
            let cube_face = self.mesh.faces[i];

            // Get the vertices of the current face.
            let mut face_vertices: [Vec3; 3] = [Vec3::new(0.0, 0.0, 0.0); 3];
            face_vertices[0] = self.mesh.vertices[cube_face.a - 1];
            face_vertices[1] = self.mesh.vertices[cube_face.b - 1];
            face_vertices[2] = self.mesh.vertices[cube_face.c - 1];

            // Initialize a triangle for the projected points.
            let mut projected_triangle: triangle::Triangle = triangle::Triangle {
                points: [vector::Vec2 { x: 0.0, y: 0.0 }; 3],
            };

            for j in 0..3 {
                let mut transformed_vertex = face_vertices[j];
                transformed_vertex = transformed_vertex.rotate_x(self.mesh.rotation.x);
                transformed_vertex = transformed_vertex.rotate_y(self.mesh.rotation.y);
                transformed_vertex = transformed_vertex.rotate_z(self.mesh.rotation.z);

                // Translate the vertex relative to the camera position.
                transformed_vertex.z -= self.camera_position.z;

                // Project the transformed vertex to 2D.
                let mut projected_point = self.project(transformed_vertex);

                // Center the projected point on the screen.
                projected_point.x += display::WINDOW_WIDTH as f32 / 2.0;
                projected_point.y += display::WINDOW_HEIGHT as f32 / 2.0;
                projected_triangle.points[j] = projected_point;
            }

            // Add the projected triangle to the render list.
            self.triangles_to_render.push(projected_triangle);
        }
    }

    /// Renders all triangles to the screen and updates the display.
    pub fn render(&mut self) {
        // Draw each triangle onto the color buffer.
        for triangle in &self.triangles_to_render {
            display::draw_triangle(
                &mut self.color_buffer,
                triangle.points,
                sdl2::pixels::Color::RGBA(0, 150, 0, 255), // Green color.
            );
        }

        // Clear the triangle list and update the canvas.
        self.triangles_to_render.clear();
        display::render_color_buffer(&mut self.canvas, &mut self.color_buffer);
        display::clear_color_buffer(&mut self.color_buffer);
        self.canvas.present();

        // Cap the frame rate.
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / FRAMES_PER_SECOND));
    }
}

/// Entry point of the application. Initializes SDL2, the window, and the renderer,
/// and starts the main render loop.
pub fn main() {
    let sdl_context = sdl2::init().unwrap(); // Initialize SDL2.
    let window = display::initialize_window(&sdl_context); // Create a window.
    let mut renderer = Renderer::new(window.unwrap(), sdl_context); // Create the renderer.

    // Main application loop.
    while renderer.is_running {
        renderer.process_input(); // Handle user input.
        renderer.update(); // Update object transformations.
        renderer.render(); // Render the frame.
    }
}
