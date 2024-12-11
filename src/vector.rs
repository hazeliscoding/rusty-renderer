use std::ops::{Add, Div, Mul, Neg, Sub};

/// A 2D vector struct, representing a point or direction in 2D space.
#[derive(Debug, Copy, Clone)]
pub struct Vec2 {
    /// The x-coordinate of the vector.
    pub(crate) x: f32,
    /// The y-coordinate of the vector.
    pub(crate) y: f32,
}

#[allow(dead_code)]
impl Vec2 {
    /// Creates a new instance of the `Vec2` struct.
    ///
    /// # Arguments
    /// - `x`: The x-coordinate of the vector.
    /// - `y`: The y-coordinate of the vector.
    ///
    /// # Returns
    /// A new instance of `Vec2`.
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { x, y }
    }

    /// Calculates the dot product of two 2D vectors.
    ///
    /// # Arguments
    /// - `self`: The first vector.
    /// - `other`: The second vector.
    ///
    /// # Returns
    /// The dot product as a `f32` value.
    pub fn dot(&self, other: Vec2) -> f32 {
        self.x * other.x + self.y * other.y
    }

    /// Calculates the length (magnitude) of the vector.
    ///
    /// # Returns
    /// The length as a `f32` value.
    pub fn len(&self) -> f32 {
        self.dot(*self).sqrt()
    }
}

/// Implements the subtraction operator for `Vec2`.
///
/// # Arguments
/// - `self`: The minuend `Vec2`.
/// - `other`: The subtrahend `Vec2`.
///
/// # Returns
/// A new `Vec2` representing the difference.
///
/// # Example
/// ```
/// let a = Vec2::new(3.0, 4.0);
/// let b = Vec2::new(1.0, 2.0);
/// let result = a - b; // Vec2 { x: 2.0, y: 2.0 }
/// ```
impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// Implements the addition operator for `Vec2`.
///
/// # Arguments
/// - `self`: The first vector.
/// - `other`: The second vector.
///
/// # Returns
/// A new `Vec2` representing the sum.
///
/// # Example
/// ```
/// let a = Vec2::new(3.0, 4.0);
/// let b = Vec2::new(1.0, 2.0);
/// let result = a + b; // Vec2 { x: 4.0, y: 6.0 }
/// ```
impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

/// Implements scalar multiplication for `Vec2`.
///
/// # Arguments
/// - `self`: The vector to be multiplied.
/// - `scalar`: The scalar value.
///
/// # Returns
/// A new `Vec2` representing the product.
///
/// # Example
/// ```
/// let v = Vec2::new(3.0, 4.0);
/// let result = v * 2.0; // Vec2 { x: 6.0, y: 8.0 }
/// ```
impl Mul<f32> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

/// Implements scalar division for `Vec2`.
///
/// # Arguments
/// - `self`: The vector to be divided.
/// - `scalar`: The scalar value.
///
/// # Returns
/// A new `Vec2` representing the quotient.
///
/// # Example
/// ```
/// let v = Vec2::new(6.0, 8.0);
/// let result = v / 2.0; // Vec2 { x: 3.0, y: 4.0 }
/// ```
impl Div<f32> for Vec2 {
    type Output = Vec2;

    fn div(self, scalar: f32) -> Vec2 {
        Vec2 {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

/// Implements the negation operator for `Vec2`.
///
/// # Arguments
/// - `self`: The vector to be negated.
///
/// # Returns
/// A new `Vec2` with the negated components.
///
/// # Example
/// ```
/// let v = Vec2::new(3.0, 4.0);
/// let result = -v; // Vec2 { x: -3.0, y: -4.0 }
/// ```
impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Vec2 {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

/// A 3D vector struct, representing a point or direction in 3D space.
#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    /// The x-coordinate of the vector.
    pub x: f32,
    /// The y-coordinate of the vector.
    pub y: f32,
    /// The z-coordinate of the vector.
    pub z: f32,
}

#[allow(dead_code)]
impl Vec3 {
    /// Creates a new instance of the `Vec3` struct.
    ///
    /// # Arguments
    /// - `x`: The x-coordinate of the vector.
    /// - `y`: The y-coordinate of the vector.
    /// - `z`: The z-coordinate of the vector.
    ///
    /// # Returns
    /// A new instance of `Vec3`.
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }

    /// Rotates the vector around the X-axis.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// A new rotated `Vec3`.
    pub fn rotate_x(&self, angle: f32) -> Vec3 {
        let y = self.y * angle.cos() - self.z * angle.sin();
        let z = self.y * angle.sin() + self.z * angle.cos();
        Vec3 { x: self.x, y, z }
    }

    /// Rotates the vector around the Y-axis.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// A new rotated `Vec3`.
    pub fn rotate_y(&self, angle: f32) -> Vec3 {
        let x = self.x * angle.cos() + self.z * angle.sin();
        let z = -self.x * angle.sin() + self.z * angle.cos();
        Vec3 { x, y: self.y, z }
    }

    /// Rotates the vector around the Z-axis.
    ///
    /// # Arguments
    /// - `angle`: The rotation angle in radians.
    ///
    /// # Returns
    /// A new rotated `Vec3`.
    pub fn rotate_z(&self, angle: f32) -> Vec3 {
        let x = self.x * angle.cos() - self.y * angle.sin();
        let y = self.x * angle.sin() + self.y * angle.cos();
        Vec3 { x, y, z: self.z }
    }

    /// Calculates the dot product of two 3D vectors.
    ///
    /// # Arguments
    /// - `self`: The first vector.
    /// - `other`: The second vector.
    ///
    /// # Returns
    /// The dot product as a `f32` value.
    pub fn dot(&self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Calculates the length (magnitude) of the vector.
    ///
    /// # Returns
    /// The length as a `f32` value.
    pub fn len(&self) -> f32 {
        self.dot(*self).sqrt()
    }

    /// Normalizes the vector.
    ///
    /// # Returns
    /// A new `Vec3` with a magnitude of 1.
    pub fn normalize(&self) -> Vec3 {
        self.div(self.len())
    }

    /// Calculates the cross product of two 3D vectors.
    ///
    /// # Arguments
    /// - `self`: The first vector.
    /// - `other`: The second vector.
    ///
    /// # Returns
    /// A new `Vec3` representing the cross product.
    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

/// Implements the subtraction operator for `Vec3`.
///
/// # Arguments
/// - `self`: The minuend `Vec3`.
/// - `other`: The subtrahend `Vec3`.
///
/// # Returns
/// A new `Vec3` representing the difference.
///
/// # Example
/// ```
/// let a = Vec3::new(3.0, 4.0, 5.0);
/// let b = Vec3::new(1.0, 2.0, 3.0);
/// let result = a - b; // Vec3 { x: 2.0, y: 2.0, z: 2.0 }
/// ```
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// Implements the addition operator for `Vec3`.
///
/// # Arguments
/// - `self`: The first vector.
/// - `other`: The second vector.
///
/// # Returns
/// A new `Vec3` representing the sum.
///
/// # Example
/// ```
/// let a = Vec3::new(3.0, 4.0, 5.0);
/// let b = Vec3::new(1.0, 2.0, 3.0);
/// let result = a + b; // Vec3 { x: 4.0, y: 6.0, z: 8.0 }
/// ```
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

/// Implements scalar multiplication for `Vec3`.
///
/// # Arguments
/// - `self`: The vector to be multiplied.
/// - `scalar`: The scalar value.
///
/// # Returns
/// A new `Vec3` representing the product.
///
/// # Example
/// ```
/// let v = Vec3::new(3.0, 4.0, 5.0);
/// let result = v * 2.0; // Vec3 { x: 6.0, y: 8.0, z: 10.0 }
/// ```
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

/// Implements scalar division for `Vec3`.
///
/// # Arguments
/// - `self`: The vector to be divided.
/// - `scalar`: The scalar value.
///
/// # Returns
/// A new `Vec3` representing the quotient.
///
/// # Example
/// ```
/// let v = Vec3::new(6.0, 8.0, 10.0);
/// let result = v / 2.0; // Vec3 { x: 3.0, y: 4.0, z: 5.0 }
/// ```
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f32) -> Vec3 {
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

/// Implements the negation operator for `Vec3`.
///
/// # Arguments
/// - `self`: The vector to be negated.
///
/// # Returns
/// A new `Vec3` with the negated components.
///
/// # Example
/// ```
/// let v = Vec3::new(3.0, 4.0, 5.0);
/// let result = -v; // Vec3 { x: -3.0, y: -4.0, z: -5.0 }
/// ```
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Vec3 {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
