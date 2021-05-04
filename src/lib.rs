mod kd_tree;
mod ray_box;
mod vec3;

pub use crate::kd_tree::*;
pub use crate::ray_box::*;
pub use crate::vec3::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
