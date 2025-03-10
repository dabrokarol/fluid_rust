use crate::{HEIGHT, WIDTH};

pub const RADIUS: usize = 15;

#[derive(Clone, Copy, Debug)]
pub struct Particle {
    pub position: [f32; 2],
    pub velocity: [f32; 2],
    pub force: [f32; 2],
    pub id: i32,
    pub radius: usize,
}

impl Particle {
    pub fn new(position: [f32; 2], velocity: [f32; 2], id: i32) -> Self {
        Particle {
            position,
            velocity,
            force: [0.0, 0.0],
            id,
            radius: RADIUS,
        }
    }

    pub fn update(&mut self, delta_time: f32) {
        if 0.0 > self.position[0] {
            self.velocity[0] *= -0.5;
            self.position[0] = 0.1;
        }
        if self.position[0] > WIDTH as f32 {
            self.velocity[0] *= -0.5;
            self.position[0] = WIDTH as f32 - 0.1;
        }
        if 0.0 > self.position[1] {
            self.velocity[1] *= -0.5;
            self.position[1] = 0.1;
        }
        if self.position[1] > HEIGHT as f32 {
            self.velocity[1] *= -0.5;
            self.position[1] = HEIGHT as f32 - 0.1;
        }

        self.velocity[0] += self.force[0] * delta_time;
        self.velocity[1] += self.force[1] * delta_time;
        self.position[0] += self.velocity[0] * delta_time;
        self.position[1] += self.velocity[1] * delta_time;
        self.force = [0.0, 0.0]
    }
}
