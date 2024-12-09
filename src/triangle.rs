use crate::vector::Vec2;

/// Represents a triangle in 2D space using three points (vertices).
#[derive(Debug, Copy, Clone)] // Allows Triangle to be debugged, copied, and cloned.
pub struct Triangle {
    /// The three points (vertices) of the triangle.
    pub(crate) points: [Vec2; 3],
}

/// Represents a face of a 3D object using indices that point to vertices in a shared vertex array.
///
/// # Note
/// This is commonly used in 3D graphics to define which vertices in a vertex array
/// form a triangular face.
#[derive(Debug, Copy, Clone)] // Enables debugging, copying, and cloning of Face instances.
pub struct Face {
    /// Index of the first vertex in the vertex array.
    pub(crate) a: usize,
    /// Index of the second vertex in the vertex array.
    pub(crate) b: usize,
    /// Index of the third vertex in the vertex array.
    pub(crate) c: usize,
}

#[allow(dead_code)] // Allows unused methods for now, useful during development.
impl Triangle {
    /// Creates a new `Triangle` instance.
    ///
    /// # Arguments
    /// - `points`: An array of three `Vec2` points representing the vertices of the triangle.
    ///
    /// # Returns
    /// A new `Triangle` with the given vertices.
    pub fn new(points: [Vec2; 3]) -> Triangle {
        Triangle { points }
    }
}

#[allow(dead_code)]
impl Face {
    /// Creates a new `Face` instance.
    ///
    /// # Arguments
    /// - `a`: Index of the first vertex in the shared vertex array.
    /// - `b`: Index of the second vertex in the shared vertex array.
    /// - `c`: Index of the third vertex in the shared vertex array.
    ///
    /// # Returns
    /// A new `Face` with the given vertex indices.
    pub fn new(a: usize, b: usize, c: usize) -> Face {
        Face { a, b, c }
    }
}
