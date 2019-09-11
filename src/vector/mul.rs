use std::ops::Mul;

use super::Vector3;

impl Mul<Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<&Vector3> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<Vector3> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<&Vector3> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<f64> for &Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f64) -> Vector3 {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Mul<&Vector3> for f64 {
    type Output = Vector3;

    fn mul(self, rhs: &Vector3) -> Vector3 {
        Vector3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_mul_vector_should_return_vector() {
        let (x1, y1, z1) = (1., 2., 3.);
        let (x2, y2, z2) = (10., 20., 30.);
        let src1 = Vector3::new(x1, y1, z1);
        let src2 = Vector3::new(x2, y2, z2);
        let dst = src1 * src2;
        assert_eq!(dst.x, x1 * x2);
        assert_eq!(dst.y, y1 * y2);
        assert_eq!(dst.z, z1 * z2);
    }

    #[test]
    fn vector_mul_ref_vector_should_return_vector() {
        let (x1, y1, z1) = (1., 2., 3.);
        let (x2, y2, z2) = (10., 20., 30.);
        let src1 = Vector3::new(x1, y1, z1);
        let src2 = Vector3::new(x2, y2, z2);
        let dst = src1 * &src2;
        assert_eq!(dst.x, x1 * x2);
        assert_eq!(dst.y, y1 * y2);
        assert_eq!(dst.z, z1 * z2);
    }

    #[test]
    fn ref_vector_mul_vector_should_return_vector() {
        let (x1, y1, z1) = (1., 2., 3.);
        let (x2, y2, z2) = (10., 20., 30.);
        let src1 = Vector3::new(x1, y1, z1);
        let src2 = Vector3::new(x2, y2, z2);
        let dst = &src1 * src2;
        assert_eq!(dst.x, x1 * x2);
        assert_eq!(dst.y, y1 * y2);
        assert_eq!(dst.z, z1 * z2);
    }

    #[test]
    fn ref_vector_mul_ref_vector_should_return_vector() {
        let (x1, y1, z1) = (1., 2., 3.);
        let (x2, y2, z2) = (10., 20., 30.);
        let src1 = Vector3::new(x1, y1, z1);
        let src2 = Vector3::new(x2, y2, z2);
        let dst = &src1 * &src2;
        assert_eq!(dst.x, x1 * x2);
        assert_eq!(dst.y, y1 * y2);
        assert_eq!(dst.z, z1 * z2);
    }

    #[test]
    fn vector_mul_f64_should_return_vector() {
        let f = 10.;
        let (x1, y1, z1) = (1., 2., 3.);
        let src = Vector3::new(x1, y1, z1);
        let dst = src * f;
        assert_eq!(dst.x, x1 * f);
        assert_eq!(dst.y, y1 * f);
        assert_eq!(dst.z, z1 * f);
    }

    #[test]
    fn ref_vector_mul_f64_should_return_vector() {
        let f = 10.;
        let (x1, y1, z1) = (1., 2., 3.);
        let src = Vector3::new(x1, y1, z1);
        let dst = &src * f;
        assert_eq!(dst.x, x1 * f);
        assert_eq!(dst.y, y1 * f);
        assert_eq!(dst.z, z1 * f);
    }

    #[test]
    fn f64_mul_vector_should_return_vector() {
        let f = 10.;
        let (x1, y1, z1) = (1., 2., 3.);
        let src = Vector3::new(x1, y1, z1);
        let dst = f * src;
        assert_eq!(dst.x, f * x1);
        assert_eq!(dst.y, f * y1);
        assert_eq!(dst.z, f * z1);
    }

    #[test]
    fn f64_mul_ref_vector_should_return_vector() {
        let f = 10.;
        let (x1, y1, z1) = (1., 2., 3.);
        let src = Vector3::new(x1, y1, z1);
        let dst = f * &src;
        assert_eq!(dst.x, f * x1);
        assert_eq!(dst.y, f * y1);
        assert_eq!(dst.z, f * z1);
    }
}
