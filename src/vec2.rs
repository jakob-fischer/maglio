use std::ops::*;
pub use crate::traits::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Vec2<T: Copy> {
    pub t: [T; 2],
}

impl<T: Copy> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { t: [x, y] }
    }
}

impl<T: Add<T, Output = T> + Mul<T, Output = T> + Clone + Copy> Vec2<T> {
    pub fn squared_length(self: &Vec2<T>) -> T {
        self.t[0] * self.t[0] + self.t[1] * self.t[1]
    }
}

impl<T: Add<T, Output = T> + Mul<T, Output = T> + Clone + Copy> Vec2<T> {
    pub fn dot(self: &Vec2<T>, other: &Vec2<T>) -> T {
        self.t[0] * other.t[0] + self.t[1] * other.t[1]
    }
}

impl Norm for Vec2<f32> {
    type Length = f32;
    fn length(self: &Self) -> Self::Length {
        self.squared_length().sqrt()
    }
}

impl Norm for Vec2<f64> {
    type Length = f64;
    fn length(self: &Self) -> Self::Length {
        self.squared_length().sqrt()
    }
}

impl<T: Neg<Output = T> + Copy> Neg for &Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2::<T> {
            t: [-self.t[0], -self.t[1]],
        }
    }
}

impl<T: Neg<Output = T> + Copy> Neg for Vec2<T> {
    type Output = Vec2<T>;

    fn neg(self) -> Self::Output {
        Vec2::<T> {
            t: [-self.t[0], -self.t[1]],
        }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2::<T> {
            t: [
                self.t[0] + rhs.t[0],
                self.t[1] + rhs.t[1],
            ],
        }
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(mut self, rhs: Vec2<T>) -> Self::Output {
        self.t[0] = self.t[0] + rhs.t[0];
        self.t[1] = self.t[1] + rhs.t[1];
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<&Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn add(mut self, rhs: &Vec2<T>) -> Self::Output {
        self.t[0] = self.t[0] + rhs.t[0];
        self.t[1] = self.t[1] + rhs.t[1];
        self
    }
}

impl<T: Add<T, Output = T> + Copy> Add<Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn add(self, mut rhs: Vec2<T>) -> Self::Output {
        rhs.t[0] = self.t[0] + rhs.t[0];
        rhs.t[1] = self.t[1] + rhs.t[1];
        rhs
    }
}

impl<'a, T: AddAssign<&'a T> + Copy> AddAssign<&'a Vec2<T>> for Vec2<T> {
    fn add_assign(&mut self, rhs: &'a Self) {
        self.t[0] += &rhs.t[0];
        self.t[1] += &rhs.t[1];
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2::<T> {
            t: [
                self.t[0] - rhs.t[0],
                self.t[1] - rhs.t[1],
            ],
        }
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(mut self, rhs: Vec2<T>) -> Self::Output {
        self.t[0] = self.t[0] - rhs.t[0];
        self.t[1] = self.t[1] - rhs.t[1];
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<&Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn sub(mut self, rhs: &Vec2<T>) -> Self::Output {
        self.t[0] = self.t[0] - rhs.t[0];
        self.t[1] = self.t[1] - rhs.t[1];
        self
    }
}

impl<T: Sub<T, Output = T> + Copy> Sub<Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn sub(self, mut rhs: Vec2<T>) -> Self::Output {
        rhs.t[0] = self.t[0] - rhs.t[0];
        rhs.t[1] = self.t[1] - rhs.t[1];
        rhs
    }
}

impl<'a, T: SubAssign<&'a T> + Copy> SubAssign<&'a Vec2<T>> for Vec2<T> {
    fn sub_assign(&mut self, rhs: &'a Self) {
        self.t[0] -= &rhs.t[0];
        self.t[1] -= &rhs.t[1];
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<&Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: &Vec2<T>) -> Self::Output {
        Vec2::<T> {
            t: [
                self.t[0] * rhs.t[0],
                self.t[1] * rhs.t[1],
            ],
        }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(mut self, rhs: Vec2<T>) -> Self::Output {
        self.t[0] = self.t[0] * rhs.t[0];
        self.t[1] = self.t[1] * rhs.t[1];
        self
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<Vec2<T>> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, mut rhs: Vec2<T>) -> Self::Output {
        rhs.t[0] = self.t[0] * rhs.t[0];
        rhs.t[1] = self.t[1] * rhs.t[1];
        rhs
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<&Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(mut self, rhs: &Vec2<T>) -> Self::Output {
        self.t[0] = self.t[0] * rhs.t[0];
        self.t[1] = self.t[1] * rhs.t[1];
        self
    }
}

impl<'a, T: MulAssign<&'a T> + Copy> MulAssign<&'a Vec2<T>> for Vec2<T> {
    fn mul_assign(&mut self, rhs: &'a Self) {
        self.t[0] *= &rhs.t[0];
        self.t[1] *= &rhs.t[1];
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2::<T> {
            t: [self.t[0] * rhs, self.t[1] * rhs],
        }
    }
}

impl<T: Mul<T, Output = T> + Copy> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;

    fn mul(mut self, rhs: T) -> Self::Output {
        self.t[0] = self.t[0] * rhs;
        self.t[1] = self.t[1] * rhs;
        self
    }
}

impl<T: MulAssign<T> + Copy> MulAssign<T> for Vec2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.t[0] *= rhs;
        self.t[1] *= rhs;
    }
}

impl<T: Div<T, Output = T> + Copy> Div<T> for &Vec2<T> {
    type Output = Vec2<T>;

    fn div(self, rhs: T) -> Self::Output {
        Vec2::<T> {
            t: [self.t[0] / rhs, self.t[1] / rhs],
        }
    }
}

impl<T: DivAssign<T> + Copy> DivAssign<T> for Vec2<T> {
    fn div_assign(&mut self, rhs: T) {
        self.t[0] /= rhs;
        self.t[1] /= rhs;
    }
}

impl<T> Vec2<T>
where
    T: Copy,
    Vec2<T>: Norm<Length = T> + DivAssign<T>,
{
    pub fn get_normalized(&self) -> Self {
        let mut result = *self;
        result /= self.length();
        result
    }
}

impl<T> Vec2<T>
where
    T: Copy,
    Vec2<T>: Norm<Length = T> + DivAssign<T>,
{
    pub fn normalize(&mut self) {
        let len = self.length();
        self.div_assign(len);
    }
}

impl Vec2d {
    pub fn is_almost_zero(&self) -> bool {
        self.squared_length() < 0e-14
    }
}

pub type Vec2d = Vec2<f64>;





#[test]
fn test_addassign_nested_local() {
    let mut x : Vec2<Vec2<i32>> = Vec2::new(Vec2::new(1,2), Vec2::new(3,4));
    let y : Vec2<Vec2<i32>> = Vec2::new(Vec2::new(4,5), Vec2::new(6,7));

    let sum : Vec2<Vec2<i32>> = Vec2::new(Vec2::new(5,7), Vec2::new(9,11));

    x += &y;
    assert_eq!(x, sum);

    println!("{:?}", x);
}