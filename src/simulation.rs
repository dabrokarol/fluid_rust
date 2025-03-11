use crate::{HEIGHT_PX, WIDTH_PX, particle::Particle, particle::RADIUS, sph::update_particles};
use rand::Rng;
use std::time::Instant;

const GRID_SIZE: usize = 2 * RADIUS;
pub struct ParticleSystem {
    pub particles: Vec<Particle>,
    pub spawn_rate: f32,
    pub max_particles: usize,
    pub spawn_position: [f32; 2],
    pub spawn_velocity: [f32; 2],
    pub spawn_velocity_variance: f32,
    pub gravity: [f32; 2],
    pub particles_to_spawn_accumulator: f32,
    pub particle_counter: i32,
    pub grid: Vec<Vec<Vec<usize>>>,
}

impl ParticleSystem {
    pub fn new(max_particles: usize, spawn_rate: f32) -> Self {
        ParticleSystem {
            particles: Vec::with_capacity(max_particles),
            spawn_rate,
            max_particles,
            spawn_position: [WIDTH_PX as f32 / 2.0, HEIGHT_PX as f32 / 2.0 - 1.0],
            spawn_velocity: [0.0, 100.0],
            spawn_velocity_variance: 0.5,
            gravity: [0.0, 100.0],
            particles_to_spawn_accumulator: 0.0,
            particle_counter: 0,
            grid: vec![vec![vec![]; HEIGHT_PX / GRID_SIZE + 2]; WIDTH_PX / GRID_SIZE + 2],
        }
    }

    pub fn update(&mut self, delta_time: f32, mouse_pos: [f32;2]) {

        let start = Instant::now();

        self.particles_to_spawn_accumulator += self.spawn_rate * delta_time;
        let particles_to_spawn = self.particles_to_spawn_accumulator as usize;
        self.particles_to_spawn_accumulator -= particles_to_spawn as f32;
        self.spawn_particles(particles_to_spawn);

        for i in 0..self.grid.len() {
            for j in 0..self.grid[0].len() {
                self.grid[i][j].clear();
            }
        }

        for id in 0..self.particles.len() {
            let p = &self.particles[id];
            let (i, j) = ParticleSystem::grid_pos(p.position[0], p.position[1]);
            self.grid[i][j].push(id);
        }

        for i in 1..self.grid.len() - 1 {
            for j in 1..self.grid[0].len() - 1 {
                for &id in &self.grid[i][j] {
                    for dx in [-1, 0, 1] {
                        for dy in [-1, 0, 1] {
                            for &id2 in
                                &self.grid[(i as i32 + dx) as usize][(j as i32 + dy) as usize]
                            {
                                ParticleSystem::handle_particle_collisions(
                                    &mut self.particles,
                                    id,
                                    id2,
                                    delta_time,
                                );
                            }
                        }
                    }
                    ParticleSystem::handle_particle(
                        &mut self.particles[id],
                        delta_time,
                        self.gravity,
                        mouse_pos,
                    );
                }
            }
        }

        let duration = start.elapsed();
        // println!("update function: {:?}", duration)
    }

    fn handle_particle(p: &mut Particle, delta_time: f32, gravity: [f32; 2], mouse_pos: [f32;2]) {
        p.force[0] += gravity[0];
        p.force[1] += gravity[1];
        
        let dist_vec = [
            mouse_pos[0] - p.position[0],
            mouse_pos[1] - p.position[1],
        ];
        let dist = dist_vec[0] * dist_vec[0] + dist_vec[1] * dist_vec[1]; 
            
        let force = [dist_vec[0] / dist * delta_time * 3000.0, dist_vec[1] / dist * delta_time * 3000.0];
            
        p.force[0] += force[0] * 100.0;
        p.force[1] += force[1] * 100.0;
            
        p.update(delta_time);

    }

    fn handle_particle_collisions(
        particles: &mut Vec<Particle>,
        id1: usize,
        id2: usize,
        delta_time: f32,
    ) {
        if id1 == id2 {
            return;
        }
        let dist_vec = [
            particles[id1].position[0] - particles[id2].position[0],
            particles[id1].position[1] - particles[id2].position[1],
        ];
        let dist = dist_vec[0] * dist_vec[0] + dist_vec[1] * dist_vec[1];

        let d = (particles[id1].radius + particles[id2].radius).pow(2) as f32;

        let force = [dist_vec[0] / dist * d * delta_time * 3000.0, dist_vec[1] / dist * d * delta_time * 3000.0];
        
        if dist < d {
            particles[id2].force[0] += -force[0];
            particles[id2].force[1] += -force[1];
            particles[id1].force[0] += force[0];
            particles[id1].force[1] += force[1];
        }
    }

    fn spawn_particles(&mut self, count: usize) {
        let mut rng = rand::rng();
        for _ in 0..count {
            if self.particles.len() < self.max_particles {
                let velocity_x = self.spawn_velocity[0]
                    + rng
                        .random_range(-self.spawn_velocity_variance..=self.spawn_velocity_variance);
                let velocity_y = self.spawn_velocity[1]
                    + rng
                        .random_range(-self.spawn_velocity_variance..=self.spawn_velocity_variance);

                self.particles.push(Particle::new(
                    self.spawn_position,
                    [velocity_x, velocity_y],
                    self.particle_counter,
                ));
                self.particle_counter += 1;
            }
        }
    }

    fn grid_pos(x: f32, y: f32) -> (usize, usize) {
        let xmax = WIDTH_PX / RADIUS / 2;
        let ymax = HEIGHT_PX / RADIUS / 2;

        let xpos: usize = (x / RADIUS as f32 / 2.0) as usize + 1;
        let ypos: usize = (y / RADIUS as f32 / 2.0) as usize + 1;
        // println!("grid {xpos}, {ypos}, per {xmax}, {ymax}");
        (xpos.min(xmax).max(0), ypos.min(ymax).max(0))
    }
}
