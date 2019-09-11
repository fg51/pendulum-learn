mod add;
mod div;
mod mul;
mod sub;

#[derive(Debug, Clone, Default)]
pub struct Vector3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x: x, y: y, z: z }
    }

    pub fn zeros() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn dot(&self, v: &Vector3) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    pub fn length_sq(&self) -> f64 {
        return self.x * self.x + self.y * self.y + self.z * self.z;
    }
}

pub fn dot(v1: &Vector3, v2: &Vector3) -> f64 {
    v1.x * v2.x + v1.y * v2.y + v1.z * v2.z
}

pub fn distance_sq(v1: &Vector3, v2: &Vector3) -> f64 {
    return (v1.x - v2.x) * (v1.x - v2.x)
        + (v1.y - v2.y) * (v1.y - v2.y)
        + (v1.z - v2.z) * (v1.z - v2.z);
}

pub fn distance(v1: &Vector3, v2: &Vector3) -> f64 {
    return distance_sq(v1, v2).sqrt();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3_new() {
        let (x, y, z) = (1., 2., 3.);
        let vs = Vector3::new(x, y, z);
        assert_eq!(vs.x, x);
        assert_eq!(vs.y, y);
        assert_eq!(vs.z, z);
    }

    #[test]
    fn vector3_zeros() {
        let expect = 0.;
        let vs = Vector3::zeros();
        assert_eq!(vs.x, expect);
        assert_eq!(vs.y, expect);
        assert_eq!(vs.z, expect);
    }

    #[test]
    fn vector3_dot() {
        let (x1, y1, z1) = (1., 2., 3.);
        let (x2, y2, z2) = (10., 20., 30.);
        let v1 = Vector3::new(x1, y1, z1);
        let v2 = Vector3::new(x2, y2, z2);
        assert_eq!(v1.dot(&v2), x1 * x2 + y1 * y2 + z1 * z2);
    }

    #[test]
    fn func_dot() {
        let (x1, y1, z1) = (1., 2., 3.);
        let (x2, y2, z2) = (10., 20., 30.);
        let v = dot(&Vector3::new(x1, y1, z1), &Vector3::new(x2, y2, z2));
        assert_eq!(v, x1 * x2 + y1 * y2 + z1 * z2);
    }

    #[test]
    fn vector3_length_sq() {
        let (x1, y1, z1) = (1., 2., 3.);
        let v = Vector3::new(x1, y1, z1);
        assert_eq!(v.length_sq(), x1 * x1 + y1 * y1 + z1 * z1);
    }

    #[test]
    fn vector3_distance_sq() {
        let (x1, y1, z1) = (1f64, 2f64, 3f64);
        let (x2, y2, z2) = (10., 20., 30.);
        let expect = [x1 - x2, y1 - y2, z1 - z2]
            .iter()
            .map(|i| i.powi(2))
            .fold(0., |sum, j| sum + j);
        let v1 = Vector3::new(x1, y1, z1);
        let v2 = Vector3::new(x2, y2, z2);
        assert_eq!(distance_sq(&v1, &v2), expect);
    }

    #[test]
    fn vector3_distance() {
        let (x1, y1, z1) = (1., 2., 3.);
        let (x2, y2, z2) = (10., 20., 30.);
        let v1 = Vector3::new(x1, y1, z1);
        let v2 = Vector3::new(x2, y2, z2);
        assert_eq!(distance(&v1, &v2), distance_sq(&v1, &v2).sqrt());
    }
}
