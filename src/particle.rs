use crate::{vector3d::Vector3d, HEIGHT_PX, WIDTH_PX, sph::X_MAX, sph::Y_MAX, sph::Z_MAX};

pub const RADIUS: f32 = 15e-3; //15 mm
pub const MASS: f32 = 1e-3;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub position: Vector3d,
    pub velocity: Vector3d,
    pub mass: f32,
}

impl Particle {
    pub fn new(position: Vector3d, velocity: Vector3d) -> Self {
        Particle {
            position,
            velocity,
            mass: MASS
        }
    }

    pub fn update(&mut self, force: Vector3d, delta_time: f32) {
        self.position += self.velocity * delta_time / 2.0;
        self.velocity += force * delta_time / 2.0;

        self.position += self.velocity * delta_time / 2.0;
        self.velocity += force * delta_time / 2.0;

        if self.position.x < 0.0 {
            self.position.x = 0.01;
            self.velocity.x *= -1.0;
        } else if self.position.x >= X_MAX{
            self.position.x = X_MAX - 0.01;
            self.velocity.x *= -1.0;
        }

        if self.position.y < 0.0 {
            self.position.y = 0.01;
            self.velocity.y *= -1.0;
        } else if self.position.y >= Y_MAX{
            self.position.y = Y_MAX - 0.01;
            self.velocity.y *= -1.0;
        }

        if self.position.z < 0.0 {
            self.position.z = 0.01;
            self.velocity.z *= -1.0;
        } else if self.position.z >= Z_MAX{
            self.position.z = Z_MAX - 0.01;
            self.velocity.z *= -1.0;
        }

    }
}
