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

    /// Adds another vector to this vector.
    ///
    /// # Arguments
    /// - `self`: The first `Vec2` to add to.
    /// - `other`: The other `Vec2` to add to this vector.
    ///
    /// # Returns
    /// A new `Vec2` representing the sum.
    ///
    /// # Formula
    /// The sum of two vectors `a` and `b` is calculated as:
    /// `a.x + b.x` and `a.y + b.y`.
    pub fn add(&self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
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

    /// Multiplies this vector by a scalar value.
    ///
    /// # Arguments
    /// - `self`: The `Vec2` to multiply.
    /// - `scalar`: The scalar value to multiply this vector by.
    ///
    /// # Returns
    /// A new `Vec2` representing the product.
    ///
    /// # Formula
    /// The product of a vector `a` and a scalar `s` is calculated as:
    /// `a.x * s` and `a.y * s`.
    pub fn mul(&self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }

    /// Divides this vector by a scalar value.
    ///
    /// # Arguments
    /// - `self`: The `Vec2` to divide.
    /// - `scalar`: The scalar value to divide this vector by.
    ///
    /// # Returns
    /// A new `Vec2` representing the quotient.
    ///
    /// # Formula
    /// The quotient of a vector `a` and a scalar `s` is calculated as:
    /// `a.x / s` and `a.y / s`.
    pub fn div(&self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }

    /// Calculates the length of the vector.
    ///
    /// # Arguments
    /// - `self`: The vector to calculate the length of.
    ///
    /// # Returns
    /// The length of the vector as a `f32` value.
    ///
    /// # Formula
    /// The length of a vector `a` is calculated as:
    /// `sqrt(a.x^2 + a.y^2)`.
    pub fn len(&self) -> f32 {
        self.dot(*self).sqrt()
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

    /// Calculates the dot product of two vectors.
    ///
    /// # Arguments
    /// - `self`: The first vector.
    /// - `other`: The second vector.
    ///
    /// # Returns
    /// The dot product as a `Vec3` value.
    ///
    /// # Formula
    /// The dot product of two vectors `a` and `b` is calculated as:
    /// `a x b = (a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)`.
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Multiplies this vector by a scalar value.
    ///
    /// # Arguments
    /// - `self`: The vector to multiply.
    /// - `scalar`: The scalar value to multiply this vector by.
    ///
    /// # Returns
    /// A new `Vec3` representing the product.
    ///
    /// # Formula
    /// The product of a vector `a` and a scalar `s` is calculated as:
    /// `a.x * s`, `a.y * s`, and `a.z * s`.
    pub fn mul(&self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }

    /// Divides this vector by a scalar value.
    /// 
    /// # Arguments
    /// - `self`: The vector to divide.
    /// - `scalar`: The scalar value to divide this vector by.
    /// 
    /// # Returns
    /// A new `Vec3` representing the quotient.
    /// 
    /// # Formula
    /// The quotient of a vector `a` and a scalar `s` is calculated as:
    /// `a.x / s`, `a.y / s`, and `a.z / s`.
    pub fn div(&self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }

    /// Calculates the length of the vector.
    /// 
    /// # Arguments
    /// - `self`: The vector to calculate the length of.
    /// 
    /// # Returns
    /// The length of the vector as a `f32` value.
    /// 
    /// # Formula
    /// The length of a vector `a` is calculated as:
    /// `sqrt(a.x^2 + a.y^2 + a.z^2)`.
    pub fn len(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    /// Normalizes the vector.
    /// 
    /// # Arguments 
    /// - `self`: The vector to normalize.
    /// 
    /// # Returns
    /// A new `Vec3` representing the normalized vector.
    /// 
    /// # Formula
    /// The normalized vector `a` is calculated as:
    /// `a / |a|`.
    pub fn normalize(&self) -> Vec3 {
        self.div(self.len())
    }

    /// Calculates the cross product of two vectors.
    /// 
    /// # Arguments
    /// - `self`: The first vector.
    /// - `other`: The second vector.
    /// 
    /// # Returns
    /// The cross product as a `Vec3` value.
    /// 
    /// # Formula
    /// The cross product of two vectors `a` and `b` is calculated as:
    /// `a x b = (a.y * b.z - a.z * b.y, a.z * b.x - a.x * b.z, a.x * b.y - a.y * b.x)`.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
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
