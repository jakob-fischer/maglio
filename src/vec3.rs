pub use crate::traits::*;
use crate::vec2::*;
use ordered_float::NotNan;
use std::hash::{Hash, Hasher};
use std::ops::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec3<T: Copy> {
    pub t: [T; 3],
}

impl<T: Copy> Vec3<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { t: [x, y, z] }
    }

    pub fn as_vec2(&self) -> Vec2<T> {
        Vec2::new(self.t[0], self.t[1])
    }
}

impl<T: Add<T, Output = T> + Mul<T, Output = T> + Clone + Copy> Vec3<T> {
    pub fn squared_length(self: &Vec3<T>) -> T {
        self.t[0] * self.t[0] + self.t[1] * self.t[1] + self.t[2] * self.t[2]
    }
}

impl<T: Add<T, Output = T> + Mul<T, Output = T> + Clone + Copy> Vec3<T> {
    pub fn dot(self: &Vec3<T>, other: &Vec3<T>) -> T {
        self.t[0] * other.t[0] + self.t[1] * other.t[1] + self.t[2] * other.t[2]
    }
}

impl<T: Sub<T, Output = T> + Mul<T, Output = T> + Clone + Copy> Vec3<T> {
    pub fn cross(self: &Vec3<T>, other: &Vec3<T>) -> Vec3<T> {
        Self::new(
            self.t[1] * other.t[2] - self.t[2] * other.t[1],
            self.t[2] * other.t[0] - self.t[0] * other.t[2],
            self.t[0] * other.t[1] - self.t[1] * other.t[0],
        )
    }
}

impl Norm for Vec3<f64> {
    type Length = f64;
    fn length(self: &Self) -> Self::Length {
        self.squared_length().sqrt()
    }
}

impl<T: Neg<Output = T> + Copy> Neg for &Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::<T> {
            t: [-self.t[0], -self.t[1], -self.t[2]],
        }
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Vec3<T> {
    type Output = Vec3<T>;

    fn neg(self) -> Self::Output {
        Vec3::<T> {
            t: [-self.t[0], -self.t[1], -self.t[2]],
        }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3::<T> {
            t: [
                self.t[0] + rhs.t[0],
                self.t[1] + rhs.t[1],
                self.t[2] + rhs.t[2],
            ],
        }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(mut self, rhs: Vec3<T>) -> Self::Output {
        self.t[0] = self.t[0] + rhs.t[0];
        self.t[1] = self.t[1] + rhs.t[1];
        self.t[2] = self.t[2] + rhs.t[2];
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn add(mut self, rhs: &Vec3<T>) -> Self::Output {
        self.t[0] = self.t[0] + rhs.t[0];
        self.t[1] = self.t[1] + rhs.t[1];
        self.t[2] = self.t[2] + rhs.t[2];
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn add(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs.t[0] = self.t[0] + rhs.t[0];
        rhs.t[1] = self.t[1] + rhs.t[1];
        rhs.t[2] = self.t[2] + rhs.t[2];
        rhs
    }
}

impl<'a, T: AddAssign<&'a T> + Copy> AddAssign<&'a Vec3<T>> for Vec3<T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.t[0] += &rhs.t[0];
        self.t[1] += &rhs.t[1];
        self.t[2] += &rhs.t[2];
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3::<T> {
            t: [
                self.t[0] - rhs.t[0],
                self.t[1] - rhs.t[1],
                self.t[2] - rhs.t[2],
            ],
        }
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(mut self, rhs: Vec3<T>) -> Self::Output {
        self.t[0] = self.t[0] - rhs.t[0];
        self.t[1] = self.t[1] - rhs.t[1];
        self.t[2] = self.t[2] - rhs.t[2];
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn sub(mut self, rhs: &Vec3<T>) -> Self::Output {
        self.t[0] = self.t[0] - rhs.t[0];
        self.t[1] = self.t[1] - rhs.t[1];
        self.t[2] = self.t[2] - rhs.t[2];
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn sub(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs.t[0] = self.t[0] - rhs.t[0];
        rhs.t[1] = self.t[1] - rhs.t[1];
        rhs.t[2] = self.t[2] - rhs.t[2];
        rhs
    }
}

impl<'a, T: SubAssign<&'a T> + Copy> SubAssign<&'a Vec3<T>> for Vec3<T> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.t[0] -= &rhs.t[0];
        self.t[1] -= &rhs.t[1];
        self.t[2] -= &rhs.t[2];
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<&Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3::<T> {
            t: [
                self.t[0] * rhs.t[0],
                self.t[1] * rhs.t[1],
                self.t[2] * rhs.t[2],
            ],
        }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(mut self, rhs: Vec3<T>) -> Self::Output {
        self.t[0] = self.t[0] * rhs.t[0];
        self.t[1] = self.t[1] * rhs.t[1];
        self.t[2] = self.t[2] * rhs.t[2];
        self
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<Vec3<T>> for &Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, mut rhs: Vec3<T>) -> Self::Output {
        rhs.t[0] = self.t[0] * rhs.t[0];
        rhs.t[1] = self.t[1] * rhs.t[1];
        rhs.t[2] = self.t[2] * rhs.t[2];
        rhs
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<&Vec3<T>> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(mut self, rhs: &Vec3<T>) -> Self::Output {
        self.t[0] = self.t[0] * rhs.t[0];
        self.t[1] = self.t[1] * rhs.t[1];
        self.t[2] = self.t[2] * rhs.t[2];
        self
    }
}

impl<'a, T: MulAssign<&'a T> + Copy> MulAssign<&'a Vec3<T>> for Vec3<T> {
    fn mul_assign(&mut self, rhs: &'a Self) {
        self.t[0] *= &rhs.t[0];
        self.t[1] *= &rhs.t[1];
        self.t[2] *= &rhs.t[2];
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for &Vec3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec3::<T> {
            t: [self.t[0] * rhs, self.t[1] * rhs, self.t[2] * rhs],
        }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Vec3<T> {
    type Output = Vec3<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.t[0] = self.t[0] * rhs;
        self.t[1] = self.t[1] * rhs;
        self.t[2] = self.t[2] * rhs;
        self
    }
}

impl<T: MulAssign<T> + Copy> MulAssign<T> for Vec3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.t[0] *= rhs;
        self.t[1] *= rhs;
        self.t[2] *= rhs;
    }
}

impl<T: Div<T, Output = T> + Copy> Div<T> for &Vec3<T> {
    type Output = Vec3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec3::<T> {
            t: [self.t[0] / rhs, self.t[1] / rhs, self.t[2] / rhs],
        }
    }
}

impl<T: DivAssign<T> + Copy> DivAssign<T> for Vec3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.t[0] /= rhs;
        self.t[1] /= rhs;
        self.t[2] /= rhs;
    }
}

impl<T> Vec3<T>
where
    T: Copy,
    Vec3<T>: Norm<Length = T> + DivAssign<T>,
{
    pub fn get_normalized(&self) -> Self {
        let mut result = *self;
        result /= self.length();
        result
    }
}

impl<T> Vec3<T>
where
    T: Copy,
    Vec3<T>: Norm<Length = T> + DivAssign<T>,
{
    pub fn normalize(&mut self) {
        let len = self.length();
        self.div_assign(len);
    }
}

impl Vec3d {
    pub fn is_almost_zero(&self) -> bool {
        self.squared_length() < 0e-14
    }

    pub fn reflect(&self, n: &Vec3d) -> Vec3d {
        self - *n * self.dot(n) * 2.0
    }

    pub fn refract(&self, n: &Vec3d, etai_over_etat: f64) -> Vec3d {
        let cos_theta = -n.dot(self).min(1.0);
        let r_out_perp = (self + n * cos_theta) * etai_over_etat;

        let z: f64 = -(1.0 - r_out_perp.squared_length()).abs();
        let u: f64 = z.sqrt();
        let r_out_parallel = n * u;
        r_out_perp + r_out_parallel
    }
}

pub type Vec3d = Vec3<f64>;

impl Eq for Vec3d {}

impl Hash for Vec3d {
    fn hash<H: Hasher>(&self, state: &mut H) {
        NotNan::<f64>::new(self.t[0]).unwrap().hash(state);
        NotNan::<f64>::new(self.t[1]).unwrap().hash(state);
        NotNan::<f64>::new(self.t[2]).unwrap().hash(state);
    }
}

impl Vec3d {
    pub fn new_raw(x: f64, y: f64, z: f64) -> Self {
        Vec3d::new(x, y, z)
    }
}
