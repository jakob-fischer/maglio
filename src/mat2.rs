use std::ops::*;
pub use crate::vec2::*;

#[derive(Copy, Clone, Debug)]
pub struct Mat2<T: Copy> {
    pub r: Vec2<Vec2<T>>,
}

impl<T: Copy> Mat2<T> {
    pub fn new(row1: Vec2<T>, row2: Vec2<T>) -> Self {
        Self { r: Vec2::<Vec2<T>>::new(row1, row2)}
    }
}

impl<T: Neg<Output = T> + Copy> Neg for &Mat2<T> {
    type Output = Mat2<T>;

    fn neg(self) -> Self::Output {
        Mat2::<T> {
            r: self.r.neg()
        }
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Mat2<T> {
    type Output = Mat2<T>;

    fn neg(self) -> Self::Output {
        Mat2::<T> {
            r: self.r.neg(),
        }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Mat2<T>> for &Mat2<T> {
    type Output = Mat2<T>;

    fn add(self, rhs: &Mat2<T>) -> Self::Output {
        Mat2::<T> {
            r: self.r + rhs.r,
        }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn add(mut self, rhs: Mat2<T>) -> Self::Output {
        self.r = self.r + rhs.r;
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn add(mut self, rhs: &Mat2<T>) -> Self::Output {
        self.r = self.r + rhs.r;
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Mat2<T>> for &Mat2<T> {
    type Output = Mat2<T>;

    fn add(self, mut rhs: Mat2<T>) -> Self::Output {
        rhs.r = self.r + rhs.r;
        rhs
    }
}

impl<'a, T: AddAssign<&'a T> + Copy> AddAssign<&'a Mat2<T>> for Mat2<T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.r += &rhs.r;
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Mat2<T>> for &Mat2<T> {
    type Output = Mat2<T>;

    fn sub(self, rhs: &Mat2<T>) -> Self::Output {
        Mat2::<T> {
            r: self.r - rhs.r
        }
    }
}


impl<T: Sub<T, Output = T> + Copy> Sub<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn sub(mut self, rhs: Mat2<T>) -> Self::Output {
        self.r = self.r - rhs.r;
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn sub(mut self, rhs: &Mat2<T>) -> Self::Output {
        self.r = self.r - rhs.r;
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<Mat2<T>> for &Mat2<T> {
    type Output = Mat2<T>;

    fn sub(self, mut rhs: Mat2<T>) -> Self::Output {
        rhs.r = self.r - rhs.r;
        rhs
    }
}

impl<'a, T: SubAssign<&'a T> + Copy> SubAssign<&'a Mat2<T>> for Mat2<T> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.r -= &rhs.r;
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<&Mat2<T>> for &Mat2<T> {
    type Output = Mat2<T>;

    fn mul(self, rhs: &Mat2<T>) -> Self::Output {
        let rhs_col0 = Vec2::<T>::new(rhs.r.t[0].t[0], rhs.r.t[1].t[0]);
        let rhs_col1 = Vec2::<T>::new(rhs.r.t[0].t[1], rhs.r.t[1].t[1]);

        Mat2::<T>::new( 
            Vec2::<T>::new(self.r.t[0].dot(&rhs_col0),  self.r.t[0].dot(&rhs_col1)),
            Vec2::<T>::new(self.r.t[1].dot(&rhs_col0),  self.r.t[1].dot(&rhs_col1)),
        ) 
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T>  + Copy> Mul<Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul(mut self, rhs: Mat2<T>) -> Self::Output {
        self = &self * &rhs;
        self
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<Mat2<T>> for &Mat2<T> {
    type Output = Mat2<T>;

    fn mul(self, rhs: Mat2<T>) -> Self::Output {
        self * &rhs
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<&Mat2<T>> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul(mut self, rhs: &Mat2<T>) -> Self::Output {
        self = &self * rhs;
        self
    }
}

impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> MulAssign<&Mat2<T>> for Mat2<T> {
    fn mul_assign(&mut self, rhs: &Self) {
        self.r = (*self * rhs).r;
    }
}

// Multiplication Matrix * Vector
impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<&Vec2<T>> for &Mat2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2::<T>::new(self.r.t[0].dot(&rhs),  self.r.t[1].dot(&rhs))    
    }
}

// Multiplication Matrix * Vector - convenience function
impl<T: Mul<T, Output = T> + Add<T, Output = T> + Copy> Mul<Vec2<T>> for &Mat2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        self * &rhs
    }
}

// Multiplication Matrix * Scalar
impl<T: Mul<T, Output = T> + Copy> Mul<T> for &Mat2<T> {
    type Output = Mat2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Mat2::<T> {
            r: Vec2::new(self.r.t[0] * rhs, self.r.t[1] * rhs),
        }
    }
}

impl<T: MulAssign<T> + Copy> Mul<T> for Mat2<T> {
    type Output = Mat2<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self *= rhs;
        self 
    }
}

impl<T: MulAssign<T> + Copy> MulAssign<T> for Mat2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.r.t[0] *= rhs;
        self.r.t[1] *= rhs;
    }
}


impl<T: Div<T, Output = T> + Copy> Div<T> for &Mat2<T> {
    type Output = Mat2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Mat2::<T>::new(&self.r.t[0] / rhs, &self.r.t[1] / rhs)
    }
}

impl<T: DivAssign<T> + Copy> DivAssign<T> for Mat2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.r.t[0] /= rhs;
        self.r.t[1] /= rhs;
    }
}

pub type Mat2d = Mat2<f64>;
