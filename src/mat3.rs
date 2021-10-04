pub use crate::vec3::*;
use std::ops::*;

#[derive(Copy, Clone, Debug)]
pub struct Mat3<T: Copy> {
    pub r: Vec3<Vec3<T>>,
}

impl<T: Copy> Mat3<T> {
    pub fn new(row1: Vec3<T>, row2: Vec3<T>, row3: Vec3<T>) -> Self {
        Self {
            r: Vec3::<Vec3<T>>::new(row1, row2, row3),
        }
    }
}

impl<T: Neg<Output = T> + Copy> Neg for &Mat3<T> {
    type Output = Mat3<T>;

    fn neg(self) -> Self::Output {
        Mat3::<T> { r: self.r.neg() }
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Mat3<T> {
    type Output = Mat3<T>;

    fn neg(self) -> Self::Output {
        Mat3::<T> { r: self.r.neg() }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Mat3<T>> for &Mat3<T> {
    type Output = Mat3<T>;

    fn add(self, rhs: &Mat3<T>) -> Self::Output {
        Mat3::<T> { r: self.r + rhs.r }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn add(mut self, rhs: Mat3<T>) -> Self::Output {
        self.r = self.r + rhs.r;
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn add(mut self, rhs: &Mat3<T>) -> Self::Output {
        self.r = self.r + rhs.r;
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Mat3<T>> for &Mat3<T> {
    type Output = Mat3<T>;

    fn add(self, mut rhs: Mat3<T>) -> Self::Output {
        rhs.r = self.r + rhs.r;
        rhs
    }
}

impl<'a, T: AddAssign<&'a T> + Copy> AddAssign<&'a Mat3<T>> for Mat3<T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.r += &rhs.r;
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Mat3<T>> for &Mat3<T> {
    type Output = Mat3<T>;

    fn sub(self, rhs: &Mat3<T>) -> Self::Output {
        Mat3::<T> { r: self.r - rhs.r }
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn sub(mut self, rhs: Mat3<T>) -> Self::Output {
        self.r = self.r - rhs.r;
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn sub(mut self, rhs: &Mat3<T>) -> Self::Output {
        self.r = self.r - rhs.r;
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<Mat3<T>> for &Mat3<T> {
    type Output = Mat3<T>;

    fn sub(self, mut rhs: Mat3<T>) -> Self::Output {
        rhs.r = self.r - rhs.r;
        rhs
    }
}

impl<'a, T: SubAssign<&'a T> + Copy> SubAssign<&'a Mat3<T>> for Mat3<T> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.r -= &rhs.r;
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<&Mat3<T>> for &Mat3<T> {
    type Output = Mat3<T>;

    fn mul(self, rhs: &Mat3<T>) -> Self::Output {
        let rhs_col0 = Vec3::<T>::new(rhs.r.t[0].t[0], rhs.r.t[1].t[0], rhs.r.t[2].t[0]);
        let rhs_col1 = Vec3::<T>::new(rhs.r.t[0].t[1], rhs.r.t[1].t[1], rhs.r.t[2].t[1]);
        let rhs_col2 = Vec3::<T>::new(rhs.r.t[0].t[2], rhs.r.t[1].t[2], rhs.r.t[2].t[2]);

        Mat3::<T>::new(
            Vec3::<T>::new(
                self.r.t[0].dot(&rhs_col0),
                self.r.t[0].dot(&rhs_col1),
                self.r.t[0].dot(&rhs_col2),
            ),
            Vec3::<T>::new(
                self.r.t[1].dot(&rhs_col0),
                self.r.t[1].dot(&rhs_col1),
                self.r.t[1].dot(&rhs_col2),
            ),
            Vec3::<T>::new(
                self.r.t[2].dot(&rhs_col0),
                self.r.t[2].dot(&rhs_col1),
                self.r.t[2].dot(&rhs_col2),
            ),
        )
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul(mut self, rhs: Mat3<T>) -> Self::Output {
        self = &self * &rhs;
        self
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<Mat3<T>> for &Mat3<T> {
    type Output = Mat3<T>;

    fn mul(self, rhs: Mat3<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<&Mat3<T>> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul(mut self, rhs: &Mat3<T>) -> Self::Output {
        self = &self * rhs;
        self
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> MulAssign<&Mat3<T>> for Mat3<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        self.r = (*self * rhs).r;
    }
}

// Multiplication Matrix * Vector
impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<&Vec3<T>> for &Mat3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: &Vec3<T>) -> Self::Output {
        Vec3::<T>::new(
            self.r.t[0].dot(&rhs),
            self.r.t[1].dot(&rhs),
            self.r.t[2].dot(&rhs),
        )
    }
}

// Multiplication Matrix * Vector - convenience function
impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<Vec3<T>> for &Mat3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        self * &rhs
    }
}

// Multiplication Matrix * Scalar
impl<T: Mul<T, Output = T> + Copy> Mul<T> for &Mat3<T> {
    type Output = Mat3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Mat3::<T> {
            r: Vec3::new(self.r.t[0] * rhs, self.r.t[1] * rhs, self.r.t[2] * rhs),
        }
    }
}

impl<T: MulAssign<T> + Copy> Mul<T> for Mat3<T> {
    type Output = Mat3<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self
    }
}

impl<T: MulAssign<T> + Copy> MulAssign<T> for Mat3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.r.t[0] *= rhs;
        self.r.t[1] *= rhs;
    }
}

impl<T: Div<T, Output = T> + Copy> Div<T> for &Mat3<T> {
    type Output = Mat3<T>;

    fn div(self, rhs: T) -> Self::Output {
        Mat3::<T>::new(&self.r.t[0] / rhs, &self.r.t[1] / rhs, &self.r.t[2] / rhs)
    }
}

impl<T: DivAssign<T> + Copy> DivAssign<T> for Mat3<T> {
    fn div_assign(&mut self, rhs: T) {
        self.r.t[0] /= rhs;
        self.r.t[1] /= rhs;
    }
}

pub type Mat3d = Mat3<f64>;
