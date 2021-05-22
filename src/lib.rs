mod kd_tree;
mod ray_box;
mod traits;
mod vec2;
mod mat2;
mod vec3;
mod mat3;

pub use crate::kd_tree::*;
pub use crate::ray_box::*;
pub use crate::vec2::*;
pub use crate::mat2::*;
pub use crate::vec3::*;
pub use crate::mat3::*;

#[cfg(test)]
mod tests {
    use super::vec2::*;

    #[test]
    fn test_addassign_nested() {
        let mut x : Vec2<Vec2<i32>> = Vec2::new(Vec2::new(1,2), Vec2::new(3,4));
        let y : Vec2<Vec2<i32>> = Vec2::new(Vec2::new(4,5), Vec2::new(6,7));

        let sum : Vec2<Vec2<i32>> = Vec2::new(Vec2::new(5,7), Vec2::new(9,11));

        x += &y;
        assert_eq!(x, sum);
    }
}
