#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vector3d {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3d {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3d { x, y, z }
    }

    pub fn dot(&self, other: Vector3d) -> Vector3d {
        Vector3d {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }

    pub fn cross(&self, other: &Vector3d) -> Vector3d {
        Vector3d {
            x: (self.y * other.z - self.z * other.y),
            y: (self.z * other.x - self.x * other.z),
            z: (self.x * other.y - self.y * other.x),
        }
    }

    pub fn abs_sq(&self) -> f32 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn abs(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalized(&self) -> Vector3d {
        let norm = self.abs();
        *self / norm // why dereferencing?
    }

    pub fn zeros() -> Vector3d {
        return Vector3d::new(0.0, 0.0, 0.0);
    }
}

impl std::ops::Add for Vector3d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Vector3d {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl std::ops::Mul<f32> for Vector3d {
    type Output = Self;

    fn mul(self, scalar: f32) -> Self {
        Vector3d {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl std::ops::Sub for Vector3d {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Vector3d {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl std::ops::Div<f32> for Vector3d {
    type Output = Self;

    fn div(self, scalar: f32) -> Self {
        Vector3d {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

impl std::ops::Mul<Vector3d> for f32 {
    type Output = Vector3d;

    fn mul(self, vector: Vector3d) -> Vector3d {
        Vector3d::new(vector.x * self, vector.y * self, vector.z * self)
    }
}

impl std::ops::AddAssign for Vector3d {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::SubAssign for Vector3d {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl std::ops::MulAssign<f32> for Vector3d {
    fn mul_assign(&mut self, scalar: f32) {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
    }
}

impl std::ops::DivAssign<f32> for Vector3d {
    fn div_assign(&mut self, scalar: f32) {
        self.x /= scalar;
        self.y /= scalar;
        self.z /= scalar;
    }
}
