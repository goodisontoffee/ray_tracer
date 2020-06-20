use std::ops;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    e: [f32; 3],
}

impl Vec3 {
    pub fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Vec3 { e: [e0, e1, e2] }
    }

    fn length(self) -> f32 {
        (self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }

    pub fn x(self) -> f32 {
        self.e[1]
    }

    pub fn y(self) -> f32 {
        self.e[1]
    }

    pub fn z(self) -> f32 {
        self.e[2]
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl ops::Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            e: [
                self.e[0] / rhs.e[0],
                self.e[1] / rhs.e[1],
                self.e[2] / rhs.e[2],
            ],
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let k = 1.0 / rhs;

        Vec3 {
            e: [self.e[0] * k, self.e[1] * k, self.e[2] * k],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3_add() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) + Vec3::new(1.0, 3.0, 5.0),
            Vec3::new(3.0, 7.0, 11.0)
        );
    }

    #[test]
    fn test_vec3_sub() {
        assert_eq!(
            Vec3::new(2.0, 4.0, 6.0) - Vec3::new(1.0, 3.0, 5.0),
            Vec3::new(1.0, 1.0, 1.0)
        );
    }

    #[test]
    fn test_vec3_mul() {
        assert_eq!(
            Vec3::new(2.0, 3.0, 4.0) * Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(10.0, 18.0, 28.0)
        )
    }

    #[test]
    fn test_vec3_mul_f32() {
        assert_eq!(Vec3::new(2.0, 3.0, 4.0) * 2.0, Vec3::new(4.0, 6.0, 8.0))
    }

    #[test]
    fn test_vec3_div() {
        assert_eq!(
            Vec3::new(10.0, 18.0, 28.0) / Vec3::new(5.0, 6.0, 7.0),
            Vec3::new(2.0, 3.0, 4.0)
        )
    }

    #[test]
    fn test_vec3_div_f32() {
        assert_eq!(Vec3::new(10.0, 18.0, 28.0) / 2.0, Vec3::new(5.0, 9.0, 14.0))
    }
}
