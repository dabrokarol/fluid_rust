use std::vec;

use crate::{
    HEIGHT_PX, WIDTH_PX,
    particle::{Particle, RADIUS},
    vector3d::Vector3d,
};

pub const SCALE: f32 = 1e-3; // 1mm per pixel
pub const X_MAX: f32 = px2meters(WIDTH_PX);
pub const Y_MAX: f32 = px2meters(HEIGHT_PX);
pub const Z_MAX: f32 = 1.0;

const H: f32 = 3.0 * RADIUS;
const SIGMA: f32 = 1.0;
const K: f32 = 1.0;
const RO_0: f32 = 1.0;
const MU: f32 = 1.0;

const INIT_VELOCITY: Vector3d = Vector3d {
    x: 10.0,
    y: 0.0,
    z: 0.0,
};
const SPAWN_POSITION: Vector3d = Vector3d {
    x: X_MAX / 2.0,
    y: Y_MAX / 2.0,
    z: Z_MAX / 2.0,
};

const GRID_SIZE: usize = 2 * meters2px(RADIUS);

pub struct PhysicsSim {
    pub particles: Vec<Particle>,
    pub gravity: Vector3d,
    pub grid: Vec<Vec<Vec<usize>>>,
    pub forces: Vec<Vector3d>,
    pub densities: Vec<f32>,
}

impl PhysicsSim {
    pub fn new() -> PhysicsSim {
        PhysicsSim {
            gravity: Vector3d::new(0.0, 0.0, -9.81),
            particles: vec![],
            grid: vec![],
            forces: vec![],
            densities: vec![],
        }
    }

    pub fn add_particle(&mut self) {
        self.particles
            .push(Particle::new(SPAWN_POSITION, INIT_VELOCITY));
        self.forces.push(Vector3d::zeros());
        self.densities.push(0.0);
    }

    pub fn update_particles(&mut self, delta_time: f32) {
        // calculating densities
        for i in 0..self.particles.len() {
            for j in i + 1..self.particles.len() {
                let p1 = &self.particles[i];
                let p2 = &self.particles[j];
                self.densities[i] += density(p1.position, p2);
                self.densities[j] += density(p2.position, p1);
            }
        }
        // calculating forces
        for i in 0..self.particles.len() {
            for j in i + 1..self.particles.len() {
                self.calc_forces(i, j);
                self.calc_forces(j, i);
            }
        }

        for i in 0..self.particles.len() {
            self.particles[i].update(self.forces[i], delta_time);
        }
    }

    pub fn calc_forces(&mut self, i: usize, j: usize) {
        let p1 = self.particles[i];
        let p2 = self.particles[j];
        let d1 = self.densities[i];
        let d2 = self.densities[j];

        let f_pressure = -p2.mass * (pressure(d1) + pressure(d2)) / 2.0
            * grad_w_spiking(p1.position - p2.position);

        let f_viscosity = MU * p2.mass * (p2.velocity - p1.velocity) / d2
            * laplacian_w_viscosity(p1.position - p2.position);

        let n = p2.mass / d2 * grad_w_poly6(p1.position - p2.position);
        let f_tension = if n.abs() > 0.01 {
            let k = -laplacian_w_poly6(p1.position - p2.position) / n.abs();
            -SIGMA * k * n
        } else {
            Vector3d::zeros()
        };

        self.forces[i] = f_pressure + f_viscosity + f_tension;
    }
}

pub fn density(r: Vector3d, p: &Particle) -> f32 {
    p.mass * w_poly6(r - p.position)
}

pub fn pressure(ro: f32) -> f32 {
    K * (ro - RO_0)
}
pub const fn px2meters(n: usize) -> f32 {
    return n as f32 / SCALE;
}

pub const fn meters2px(x: f32) -> usize {
    if x < 0.0 {
        panic!("Float has to be positive!||sph.rs line 26");
    }
    return (x * SCALE) as usize;
}

const W_POLY6_COEF: f32 = 315.0 / (64.0 * 3.141592653589793) / (H * H * H * H * H * H * H * H * H);
fn w_poly6(r: Vector3d) -> f32 {
    let r_norm = r.abs();
    if r_norm > H {
        0.0
    } else {
        W_POLY6_COEF * (H.powi(2) - r_norm.powi(2)).powi(3)
    }
}

fn grad_w_poly6(r: Vector3d) -> Vector3d {
    let r_norm = r.abs();
    if r_norm > H {
        Vector3d::zeros()
    } else {
        -6.0 * W_POLY6_COEF * (H.powi(2) - r_norm.powi(2)).powi(2) * r
    }
}

fn laplacian_w_poly6(r: Vector3d) -> f32 {
    let r_norm = r.abs();
    if r_norm > H {
        0.0
    } else {
        let coef = -945.0 / (32.0 * 3.141592653589793 * H.powi(9));
        coef * (H.powi(2) - r_norm.powi(2)) * (H.powi(2) - 5.0 * r_norm.powi(2))
    }
}

const W_SPIKING_COEF: f32 = 15.0 / (2.0 * 3.141592653589793) / (H * H * H * H * H * H);
fn w_spiking(r: Vector3d) -> f32 {
    if r.abs() > H {
        0.0
    } else {
        W_SPIKING_COEF * (H - r.abs()).powi(3)
    }
}

fn grad_w_spiking(r: Vector3d) -> Vector3d {
    let r_norm = r.abs();
    if r_norm > H {
        Vector3d::zeros()
    } else {
        -3.0 * W_SPIKING_COEF * (H - r_norm).powi(2) * r.normalized()
    }
}

const W_VISCOSITY_COEF: f32 = 15.0 / (2.0 * 3.141592653589793 * H * H * H);
fn w_viscosity(r: Vector3d) -> f32 {
    if r.abs() > H {
        0.0
    } else {
        W_VISCOSITY_COEF
            * (-r.abs().powi(3) / 2.0 / H.powi(3)
                + r.abs_sq() / 2.0 / H.powi(2)
                + H / 2.0 / r.abs()
                + 1.0)
    }
}

fn grad_w_viscosity(r: Vector3d) -> Vector3d {
    let r_norm = r.abs();
    if r_norm > H {
        Vector3d::zeros()
    } else {
        W_VISCOSITY_COEF
            * (-3.0 * r_norm.powi(2) / 2.0 / H.powi(3) + r_norm / H.powi(2)
                - H / 2.0 / r_norm.powi(2))
            * r.normalized()
    }
}

fn laplacian_w_viscosity(r: Vector3d) -> f32 {
    let r_norm = r.abs();
    if r_norm > H {
        0.0
    } else {
        W_VISCOSITY_COEF * (-3.0 * r_norm / H.powi(3) + 1.0 / H.powi(2) + H / r_norm.powi(3))
    }
}
