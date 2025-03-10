use simulation::ParticleSystem;
use std::thread::sleep;
use std::time::{Duration, Instant};
use minifb::MouseButton;

mod draw;
mod particle;
mod simulation; // Import the draw module
use draw::WindowHandler;

const WIDTH: usize = 1500;
const HEIGHT: usize = 600;

fn main() {
    let mut particle_system = ParticleSystem::new(1500, 300.0);

    let mut window_handler = WindowHandler::new("Particle Simulation", WIDTH, HEIGHT);

    while window_handler.is_open() {
        let frame_start = Instant::now();
        

        let mouse_pos = window_handler.get_mouse_pos().unwrap_or((0.0, 0.0));
        let mouse_pos: [f32; 2] = [mouse_pos.0, mouse_pos.1];


        for _ in 0..10 {
            particle_system.update(1.0 / 600.0, mouse_pos); // Assuming 60 FPS
        }

        let update_duration = frame_start.elapsed();

        for particle in &particle_system.particles {
            let x = particle.position[0] as i32;
            let y = particle.position[1] as i32;
            let r = particle.radius;
            window_handler.draw_circle(x, y, r as i32, 0x39a1b1);
        }
        
        window_handler.update();



        let frame_duration = frame_start.elapsed();
        println!("updatex10 {:?} | drawing {:?}", update_duration,  frame_duration - update_duration);
        let target_duration = Duration::from_millis(16);

        if frame_duration < target_duration {
            sleep((target_duration - frame_duration).max(std::time::Duration::new(0, 0)));
        }
    }
}
