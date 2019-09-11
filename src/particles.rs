use super::rk4th::RungeKutta4th;
use super::vector::Vector3;

#[derive(Debug)]
pub struct Particle {
    pub delta_time: f64,

    pub weight: f64,
    pub gravity: f64,

    pub position: Vector3,
    pub velocity: Vector3,

    pub delta_position: Vector3,
    pub delta_velocity: Vector3,
}

impl Particle {
    pub fn new(delta_time: f64, weight: f64, gravity: f64) -> Self {
        Self {
            delta_time: delta_time,
            weight: weight,
            gravity: gravity,
            position: Vector3::zeros(),
            velocity: Vector3::zeros(),
            delta_position: Vector3::zeros(),
            delta_velocity: Vector3::zeros(),
        }
    }

    pub fn time_evolution(&mut self, time: f64) {
        self.time_evolution_core(time);
    }
}

impl RungeKutta4th for Particle {
    fn time_evolution_core(&mut self, t: f64) {
        let dt = self.delta_time;

        // 1st
        let v1 = self.to_velocity(t, &self.position, &self.velocity);
        let a1 = self.to_accel(t, &self.position, &self.velocity);
        let v1a = Self::step1st(dt, &self.position, &v1);
        let a1a = Self::step1st(dt, &self.velocity, &a1);

        // 2nd
        let v2 = self.to_velocity(t + dt / 2.0, &v1a, &a1a);
        let a2 = self.to_accel(t + dt / 2.0, &v1a, &a1a);
        let v2a = Self::step2nd(dt, &self.position, &v2);
        let a2a = Self::step2nd(dt, &self.velocity, &a2);

        // 3rd
        let v3 = self.to_velocity(t + dt / 2.0, &v2a, &a2a);
        let a3 = self.to_accel(t + dt / 2.0, &v2a, &a2a);
        let v3a = Self::step3rd(dt, &self.position, &v3);
        let a3a = Self::step3rd(dt, &self.velocity, &a3);

        // 4th
        let v4 = self.to_velocity(t + dt, &v3a, &a3a);
        let a4 = self.to_accel(t + dt, &v3a, &a3a);
        self.delta_position = Self::step4th(dt, &v1, &v2, &v3, &v4);
        self.delta_velocity = Self::step4th(dt, &a1, &a2, &a3, &a4);
    }

    fn to_velocity(&self, _t: f64, _position: &Vector3, velocity: &Vector3) -> Vector3 {
        return velocity.clone();
    }

    fn to_accel(&self, _t: f64, _position: &Vector3, _velocity: &Vector3) -> Vector3 {
        return Vector3::new(0., 0., self.gravity);
    }
}
