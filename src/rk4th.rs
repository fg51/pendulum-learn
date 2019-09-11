use super::vector::Vector3;

pub trait RungeKutta4th {
    fn time_evolution_core(&mut self, t: f64);
    fn to_velocity(&self, time: f64, position: &Vector3, velocity: &Vector3) -> Vector3;
    fn to_accel(&self, time: f64, position: &Vector3, velocity: &Vector3) -> Vector3;

    fn step1st(dt: f64, x: &Vector3, dx: &Vector3) -> Vector3 {
        x + dx * dt / 2.
    }

    fn step2nd(dt: f64, x: &Vector3, dx: &Vector3) -> Vector3 {
        x + dx * dt / 2.
    }

    fn step3rd(dt: f64, x: &Vector3, dx: &Vector3) -> Vector3 {
        x + dx * dt
    }

    fn step4th(
        dt: f64,
        x1: &Vector3,
        x2: &Vector3,
        x3: &Vector3,
        x4: &Vector3,
    ) -> Vector3 {
        dt / 6. * (x1 + 2. * x2 + 2. * x3 + x4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct MockRK4th;

    impl RungeKutta4th for MockRK4th {
        fn time_evolution_core(&mut self, _t: f64) {}

        fn to_velocity(
            &self,
            _t: f64,
            _: &Vec<Vector3>,
            _: &Vec<Vector3>,
        ) -> Vec<Vector3> {
            vec![Vector3::new_zeros()]
        }

        fn to_accel(&self, _t: f64, _: &Vec<Vector3>, _: &Vec<Vector3>) -> Vec<Vector3> {
            vec![Vector3::zeros()]
        }
    }

    #[test]
    fn rungekutta4th_step1st() {
        let v =
            Foo::step1st(4., &Vector3::new(1., 2., 3.), &Vector3::new(10., 20., 30.));
        assert_eq!(v.x, 21.);
        assert_eq!(v.y, 42.);
        assert_eq!(v.z, 63.);
    }

    #[test]
    fn rungekutta4th_step2nd() {
        let v =
            Foo::step2nd(4., &Vector3::new(1., 2., 3.), &Vector3::new(10., 20., 30.));
        assert_eq!(v.x, 21.);
        assert_eq!(v.y, 42.);
        assert_eq!(v.z, 63.);
    }

    #[test]
    fn rungekutta4th_step3rd() {
        let v =
            Foo::step3rd(2., &Vector3::new(1., 2., 3.), &Vector3::new(10., 20., 30.));
        assert_eq!(v.x, 21.);
        assert_eq!(v.y, 42.);
        assert_eq!(v.z, 63.);
    }

    #[test]
    fn rungekutta4th_step4th() {
        let v = Foo::step4th(
            6.,
            &Vector3::new(1., 2., 3.),
            &Vector3::new(10., 20., 30.),
            &Vector3::new(100., 200., 300.),
            &Vector3::new(1000., 2000., 3000.),
        );
        assert_eq!(v.x, 1221.);
        assert_eq!(v.y, 2442.);
        assert_eq!(v.z, 3663.);
    }
}
