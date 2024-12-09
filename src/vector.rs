/// A 2D vector struct, representing a point or direction in 2D space.
#[derive(Debug, Clone, Copy)] // Allows Vec2 to be dubugged, copied and cloned.
pub struct Vec2 {
    /// The x-coordinate of the vector.
    pub(crate) x: f32,
    /// The y-coordinate of the vector.
    pub(crate) y: f32,
}

#[allow(dead_code)] // Suppresses warnings for unused functions.
/// Creates a new instance of the `Vec2` struct.
///
/// # Arguments
/// - `x`: The x-coordinate of the vector.
/// - `y`: The y-coordinate of the vector.
///
/// # Returns
/// A new instance of the `Vec2` struct.
impl Vec2 {
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    /// Calculates the dot product of two vectors.
    ///
    /// # Arguments
    /// - `self`: The first vector.
    /// - `other`: The second vector.
    ///
    /// # Returns
    /// The dot product as a `f32` value.
    ///
    /// # Formula
    /// The dot product of two vectors `a` and `b` is calculated as:
    /// `a.x * b.x + a.y * b.y`.
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Subtracts another vector from this vector.
    ///
    /// # Arguments
    /// - `other`: The other `Vec2` to subtract from this vector.
    ///
    /// # Returns
    /// A new `Vec2` representing the difference.
    pub fn sub(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// A 3D vector struct, representing a point or direction in 3D space.
#[derive(Debug, Copy, Clone)] // Allows Vec3 to be debugged, copied, and cloned.
pub struct Vec3 {
    /// The x-coordinate of the vector.
    pub x: f32,
    /// The y-coordinate of the vector.
    pub y: f32,
    /// The z-coordinate of the vector.
    pub z: f32,
}

impl Vec3 {
    /// Creates a new `Vec3` instance.
    ///
    /// # Arguments
    /// - `x`: The x-coordinate of the vector.
    /// - `y`: The y-coordinate of the vector.
    /// - `z`: The z-coordinate of the vector.
    ///
    /// # Returns
    /// A `Vec3` instance with the given x, y, and z values.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Rotates the vector around the X-axis by a given angle.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// A new `Vec3` after rotation.
    ///
    /// # Formula
    /// ```
    /// y' = y * cos(angle) - z * sin(angle)
    /// z' = y * sin(angle) + z * cos(angle)
    /// ```
    pub fn rotate_x(&self, angle: f32) -> Vec3 {
        let y = self.y * angle.cos() - self.z * angle.sin();
        let z = self.y * angle.sin() + self.z * angle.cos();
        Vec3 { x: self.x, y, z }
    }

    /// Rotates the vector around the Y-axis by a given angle.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// A new `Vec3` after rotation.
    ///
    /// # Formula
    /// ```
    /// x' = x * cos(angle) + z * sin(angle)
    /// z' = -x * sin(angle) + z * cos(angle)
    /// ```
    pub fn rotate_y(&self, angle: f32) -> Vec3 {
        let x = self.x * angle.cos() + self.z * angle.sin();
        let z = -self.x * angle.sin() + self.z * angle.cos();
        Vec3 { x, y: self.y, z }
    }

    /// Rotates the vector around the Z-axis by a given angle.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// A new `Vec3` after rotation.
    ///
    /// # Formula
    /// ```
    /// x' = x * cos(angle) - y * sin(angle)
    /// y' = x * sin(angle) + y * cos(angle)
    /// ```
    pub fn rotate_z(&self, angle: f32) -> Vec3 {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        Vec3 { x, y, z: self.z }
    }
}
