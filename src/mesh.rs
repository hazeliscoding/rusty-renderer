use std::io::Read;

use crate::triangle::Face;
use crate::vector;

/// Represents a 3D mesh, composed of vertices and faces, along with
/// attributes for rotation, scale, and translation.
pub struct Mesh {
    /// List of vertices (`Vec3`) that define the 3D geometry of the mesh.
    pub vertices: Vec<vector::Vec3>,
    /// List of faces (`Face`) that define how the vertices are connected into triangles.
    pub faces: Vec<Face>,
    /// Rotation of the mesh in 3D space (around x, y, and z axes).
    pub rotation: vector::Vec3,
    /// Scale of the mesh in 3D space (along x, y, and z axes).
    pub scale: vector::Vec3,
    /// Translation (position) of the mesh in 3D space.
    pub translation: vector::Vec3,
}

/// Number of vertices in a cube.
pub const N_CUBE_VERTICES: usize = 8;
/// Number of faces in a cube (6 sides, 2 triangles per side).
pub const N_CUBE_FACES: usize = 6 * 2;

/// Vertices of a unit cube, centered at the origin.
pub const CUBE_VERTICES: [vector::Vec3; N_CUBE_VERTICES] = [
    vector::Vec3 { x: -1.0, y: -1.0, z: -1.0 }, // Bottom-left-front
    vector::Vec3 { x: -1.0, y: 1.0, z: -1.0 },  // Top-left-front
    vector::Vec3 { x: 1.0, y: 1.0, z: -1.0 },   // Top-right-front
    vector::Vec3 { x: 1.0, y: -1.0, z: -1.0 },  // Bottom-right-front
    vector::Vec3 { x: 1.0, y: 1.0, z: 1.0 },    // Top-right-back
    vector::Vec3 { x: 1.0, y: -1.0, z: 1.0 },   // Bottom-right-back
    vector::Vec3 { x: -1.0, y: 1.0, z: 1.0 },   // Top-left-back
    vector::Vec3 { x: -1.0, y: -1.0, z: 1.0 },  // Bottom-left-back
];

/// Faces of a cube, defined by connecting vertices into triangles.
/// Each face has a color.
pub const CUBE_FACES: [Face; N_CUBE_FACES] = [
    // Front face (red)
    Face { a: 1, b: 2, c: 3 },
    Face { a: 1, b: 3, c: 4 },
    // Right face (green)
    Face { a: 4, b: 3, c: 5 },
    Face { a: 4, b: 5, c: 6 },
    // Back face (blue)
    Face { a: 6, b: 5, c: 7 },
    Face { a: 6, b: 7, c: 8 },
    // Left face (yellow)
    Face { a: 8, b: 7, c: 2 },
    Face { a: 8, b: 2, c: 1 },
    // Top face (cyan)
    Face { a: 7, b: 5, c: 3 },
    Face { a: 7, b: 3, c: 2 },
    // Bottom face (magenta)
    Face { a: 8, b: 1, c: 4 },
    Face { a: 8, b: 4, c: 6 },
];

impl Mesh {
    /// Creates a new cube mesh with predefined vertices and faces.
    ///
    /// # Returns
    /// A `Mesh` instance representing a cube.
    pub fn new_cube() -> Mesh {
        let mut vertices: Vec<vector::Vec3> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        for i in 0..N_CUBE_VERTICES {
            vertices.push(CUBE_VERTICES[i]);
        }

        for i in 0..N_CUBE_FACES {
            faces.push(CUBE_FACES[i]);
        }

        Mesh {
            vertices,
            faces,
            rotation: vector::Vec3::new(0.0, 0.0, 0.0), // No rotation by default.
            scale: vector::Vec3::new(1.0, 1.0, 1.0),    // Default scale is 1.
            translation: vector::Vec3::new(0.0, 0.0, 0.0), // Default position is the origin.
        }
    }

    /// Loads a mesh from a file in a simple format:
    /// - Lines starting with "v" define a vertex: `v x y z`.
    /// - Lines starting with "f" define a face: `f a/b/c`.
    ///
    /// # Arguments
    /// - `filename`: The path to the file to load.
    ///
    /// # Returns
    /// A `Mesh` instance loaded from the file.
    ///
    /// # Panics
    /// This function panics if the file cannot be read or contains invalid data.
    pub fn load_from_file(filename: &str) -> Mesh {
        let mut vertices: Vec<vector::Vec3> = Vec::new();
        let mut faces: Vec<Face> = Vec::new();

        let mut file = std::fs::File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let lines = contents.lines();

        for line in lines {
            let mut words = line.split_whitespace();
            let result = words.next();
            if result.is_none() {
                continue;
            }

            match result.unwrap() {
                "v" => {
                    // Parse vertex line: v x y z
                    let x: f32 = words.next().unwrap().parse().unwrap();
                    let y: f32 = words.next().unwrap().parse().unwrap();
                    let z: f32 = words.next().unwrap().parse().unwrap();
                    vertices.push(vector::Vec3::new(x, y, z));
                }
                "f" => {
                    // Parse face line: f a/b/c
                    let mut face = Face::new(0, 0, 0);
                    let mut i = 0;
                    for word in words {
                        let mut indices = word.split('/');
                        let index: usize = indices.next().unwrap().parse().unwrap();
                        match i {
                            0 => face.a = index,
                            1 => face.b = index,
                            2 => face.c = index,
                            _ => {}
                        }
                        i += 1;
                    }
                    faces.push(face);
                }
                _ => {}
            }
        }

        Mesh {
            vertices,
            faces,
            rotation: vector::Vec3::new(0.0, 0.0, 0.0), // Default rotation.
            scale: vector::Vec3::new(1.0, 1.0, 1.0),    // Default scale.
            translation: vector::Vec3::new(0.0, 0.0, 0.0), // Default translation.
        }
    }
}
