extern crate pendulum_learn;

use pendulum_learn::particles::Particle;

fn main() {
    println!("Hello, world!");

    let delta_time = 1E-3;
    let weight = 1.;
    let gravity = -9.8;

    let mut p = Particle::new(delta_time, weight, gravity);

    println!("time, position, velocity");
    for i in 0..10_000 {
        let t = i as f64 * delta_time;
        println!("{}, {}, {}", t, p.position.z, p.velocity.z);
        p.time_evolution(t);
        p.position.z += p.delta_position.z;
        p.velocity.z += p.delta_velocity.z;
    }
}
