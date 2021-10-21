use crate::mat2::*;
use crate::vec3::*;

pub type Colour = Vec3d;
pub type Point3d = Vec3d;
pub type Direction3d = Vec3d;

pub type Point2d = Vec2d;
pub type Direction2d = Vec2d;

#[derive(Clone)]
pub struct Ray3d {
    pub origin: Point3d,
    pub direction: Direction3d,
}

pub struct ConstrainedRay3d {
    pub ray: Ray3d,
    pub range: (f64, f64),
}

#[derive(Clone)]
pub struct Ray2d {
    pub origin: Point2d,
    pub direction: Direction2d,
}

pub struct ConstrainedRay2d {
    pub ray: Ray2d,
    pub range: (f64, f64),
}

pub enum HitBoxResult {
    Miss,
    Inside(f64),
    Outside(f64, f64),
}

#[derive(Clone, Debug)]
pub struct BoundingBox2d {
    pub u: Vec2d,
    pub v: Vec2d,
}

impl BoundingBox2d {
    pub fn new(u: Vec2d, v: Vec2d) -> Self {
        Self { u, v }
    }

    fn intersects_in_dimentions(&self, other: &Self, dimension: usize) -> bool {
        self.v.t[dimension] >= other.u.t[dimension] && self.u.t[dimension] <= other.v.t[dimension]
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.intersects_in_dimentions(other, 0) && self.intersects_in_dimentions(other, 1)
    }

    fn intersects_with_point_projected_in_dimension(
        &self,
        point: &Point2d,
        dimension: usize,
    ) -> bool {
        match dimension {
            0 => self.u.t[1] <= point.t[1] && point.t[1] <= self.v.t[1],
            1 => self.u.t[0] <= point.t[0] && point.t[0] <= self.v.t[0],
            _ => unreachable!(),
        }
    }

    pub fn is_hit_by_ray(&self, cray: &ConstrainedRay2d) -> HitBoxResult {
        let ray = &cray.ray;
        let range = &cray.range;

        let mut result = vec![];

        for i in 0..=1 {
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

impl Default for BoundingBox2d {
    fn default() -> Self {
        BoundingBox2d {
            u: Vec2d::new(0.0, 0.0),
            v: Vec2d::new(1.0, 1.0),
        }
    }
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
        point: &Point3d,
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

    pub fn is_hit_by_ray(&self, cray: &ConstrainedRay3d) -> HitBoxResult {
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

impl Default for BoundingBox3d {
    fn default() -> Self {
        BoundingBox3d {
            u: Vec3d::new_raw(0.0, 0.0, 0.0),
            v: Vec3d::new_raw(1.0, 1.0, 1.0),
        }
    }
}

impl Ray3d {
    pub fn new(origin: Point3d, direction: Direction3d) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point3d {
        &(&self.direction * t) + &self.origin
    }
}

impl Ray2d {
    pub fn new(origin: Point2d, direction: Direction2d) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f64) -> Point2d {
        &(&self.direction * t) + &self.origin
    }

    pub fn get_ray_between_points(p1: &Point2d, p2: &Point2d) -> Ray2d {
        let rot90 = &Mat2d::new_rot90();
        let direction = rot90 * (p2 - p1);
        let origin = (p1 + p2) * 0.5;
        Ray2d { origin, direction }
    }

    pub fn get_intersection(self: &Ray2d, other: &Ray2d) -> Option<f64> {
        let det =
            self.direction.t[0] * other.direction.t[1] - self.direction.t[1] * other.direction.t[0];

        if det == 0.0 {
            return None;
        }

        let dif = other.origin - self.origin;
        Some((other.direction.t[1] * dif.t[0] - other.direction.t[0] * dif.t[1]) / det)
    }
}

#[cfg(test)]
fn expect_eq(lhs: f64, rhs: f64) {
    assert_eq!((lhs - rhs).abs() < 0.001, true);
}

#[cfg(test)]
fn expect_eq_2d(lhs: &Point2d, rhs: &Point2d) {
    assert_eq!((lhs - rhs).length() < 0.001, true);
}

#[test]
fn test_ray2d_intersect() {
    let r1 = Ray2d::new(Point2d::new_raw(1.0, 0.0), Point2d::new_raw(0.0, 0.5));
    let r2 = Ray2d::new(Point2d::new_raw(0.0, 2.0), Point2d::new_raw(0.1, 0.0));
    let alpha1 = r1.get_intersection(&r2).unwrap();
    let alpha2 = r2.get_intersection(&r1).unwrap();
    expect_eq(4.0, f64::from(alpha1));
    expect_eq(10.0, f64::from(alpha2));

    expect_eq_2d(&r1.at(alpha1), &r2.at(alpha2));
}
