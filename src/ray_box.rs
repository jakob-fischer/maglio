use crate::vec3::*;

pub type Colour = Vec3d;
pub type Point = Vec3d;
pub type Direction = Vec3d;

#[derive(Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Direction,
}

pub struct ConstrainedRay {
    pub ray: Ray,
    pub range: (f64, f64),
}

pub enum HitBoxResult {
    Miss,
    Inside(f64),
    Outside(f64, f64),
}

#[derive(Clone, Debug)]
pub struct BoundingBox3d {
    pub u: Vec3d,
    pub v: Vec3d,
}

impl BoundingBox3d {
    fn intersects_in_dimentions(&self, other: &Self, dimension: usize) -> bool {
        self.v.t[dimension] >= other.u.t[dimension] && self.u.t[dimension] <= other.v.t[dimension]
    }

    fn intersects_with_point_projected_in_dimension(
        &self,
        point: &Point,
        dimension: usize,
    ) -> bool {
        match dimension {
            0 => {
                self.u.t[1] <= point.t[1]
                    && point.t[1] <= self.v.t[1]
                    && self.u.t[2] <= point.t[2]
                    && point.t[2] <= self.v.t[2]
            }
            1 => {
                self.u.t[0] <= point.t[0]
                    && point.t[0] <= self.v.t[0]
                    && self.u.t[2] <= point.t[2]
                    && point.t[2] <= self.v.t[2]
            }
            2 => {
                self.u.t[0] <= point.t[0]
                    && point.t[0] <= self.v.t[0]
                    && self.u.t[1] <= point.t[1]
                    && point.t[1] <= self.v.t[1]
            }
            _ => unreachable!(),
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.intersects_in_dimentions(other, 0)
            && self.intersects_in_dimentions(other, 1)
            && self.intersects_in_dimentions(other, 2)
    }

    pub fn is_hit_by_ray(&self, cray: &ConstrainedRay) -> HitBoxResult {
        let ray = &cray.ray;
        let range = &cray.range;

        let mut result = vec![];

        for i in 0..=2 {
            if ray.direction.t[i] != 0.0 {
                {
                    let a_x1 = (self.u.t[i] - ray.origin.t[i]) / ray.direction.t[i];
                    let p = ray.origin + ray.direction * a_x1;
                    if self.intersects_with_point_projected_in_dimension(&p, i)
                        && a_x1 >= range.0
                        && a_x1 <= range.1
                    {
                        result.push(a_x1);
                    }
                }
                {
                    let a_x2 = (self.v.t[i] - ray.origin.t[i]) / ray.direction.t[i];
                    let p = ray.origin + ray.direction * a_x2;
                    if self.intersects_with_point_projected_in_dimension(&p, i)
                        && a_x2 >= range.0
                        && a_x2 <= range.1
                    {
                        result.push(a_x2);
                    }
                }
            }
        }
        match result.len() {
            0 => HitBoxResult::Miss,
            1 => HitBoxResult::Inside(*result.get(0).unwrap()),
            2 => {
                let x = *result.get(0).unwrap();
                let y = *result.get(1).unwrap();
                if x < y {
                    HitBoxResult::Outside(x, y)
                } else {
                    HitBoxResult::Outside(y, x)
                }
            }
            _ => unreachable!(),
        }
    }
}

impl Ray {
    pub fn new(origin: Point, direction: Direction) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point {
        &(&self.direction * t) + &self.origin
    }
}