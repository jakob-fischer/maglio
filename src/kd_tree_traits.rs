use crate::ray_box::*;
use crate::vec3::*;

pub trait BoundingBoxTrait: Sized + Clone + Default + std::fmt::Debug {
    fn partition(&self) -> Option<(Self, Self)>;
    fn extend(&self) -> Option<(Self, Self)>; // First is other, Second new parent

    fn is_contained(&self, other: &Self) -> bool;
    fn intersects(&self, other: &Self) -> bool;

    fn is_sub_scale(&self, other: &Self) -> bool;
}

pub trait HittableBoundingBoxTrait: BoundingBoxTrait {
    fn hit(&self, ray: &ConstrainedRay3d) -> HitBoxResult;
}

impl BoundingBox2d {
    fn get_most_narrow_dimension(&self) -> usize {
        let dif = self.v - self.u;

        if dif.t[0] < dif.t[1] {
            0
        } else {
            1
        }
    }

    fn get_widest_dimension(&self) -> usize {
        let dif = self.v - self.u;

        if dif.t[0] > dif.t[1] {
            0
        } else {
            1
        }
    }
}

impl BoundingBoxTrait for BoundingBox2d {
    fn partition(&self) -> Option<(Self, Self)> {
        if (self.u - self.v).length() < 1e-8 {
            return None;
        }

        let dim = self.get_widest_dimension();
        let midpoint = 0.5 * (self.v.t[dim] + self.u.t[dim]);

        let mut u_new = self.u;
        let mut v_new = self.v;
        u_new.t[dim] = midpoint;
        v_new.t[dim] = midpoint;

        Some((
            BoundingBox2d {
                u: self.u,
                v: v_new,
            },
            BoundingBox2d {
                u: u_new,
                v: self.v,
            },
        ))
    }

    fn extend(&self) -> Option<(Self, Self)> {
        if (self.u - self.v).length() > 1e8 {
            return None;
        }

        let dim = self.get_most_narrow_dimension();

        let right = self.v.t[dim];
        let left = self.u.t[dim];
        let dif = right - left;
        let not_nan_zero = 0.0;

        let left_clamped = if left < not_nan_zero {
            left
        } else {
            not_nan_zero
        };
        let right_clamped = if right > not_nan_zero {
            right
        } else {
            not_nan_zero
        };
        let mut template = self.clone();

        if -left_clamped < right_clamped {
            template.u.t[dim] -= dif;
            template.v.t[dim] -= dif;
            let parent = Self {
                u: template.u.clone(),
                v: self.v.clone(),
            };
            Some((template, parent))
        } else {
            template.u.t[dim] += dif;
            template.v.t[dim] += dif;
            let parent = Self {
                u: self.u.clone(),
                v: template.v.clone(),
            };
            Some((template, parent))
        }
    }

    fn is_contained(&self, other: &Self) -> bool {
        other.u.t[0] <= self.u.t[0]
            && self.v.t[0] <= other.v.t[0]
            && other.u.t[1] <= self.u.t[1]
            && self.v.t[1] <= other.v.t[1]
    }

    fn intersects(&self, other: &Self) -> bool {
        self.intersects(other)
    }

    fn is_sub_scale(&self, other: &Self) -> bool {
        (self.v - self.u).length() <= (other.v - other.u).length()
    }
}

impl BoundingBox3d {
    fn get_most_narrow_dimension(&self) -> usize {
        let dif = self.v - self.u;

        if dif.t[0] < dif.t[1] && dif.t[0] < dif.t[2] {
            0
        } else if dif.t[1] < dif.t[2] {
            1
        } else {
            2
        }
    }

    fn get_widest_dimension(&self) -> usize {
        let dif = self.v - self.u;

        if dif.t[0] > dif.t[1] && dif.t[0] > dif.t[2] {
            0
        } else if dif.t[1] > dif.t[2] {
            1
        } else {
            2
        }
    }
}

impl BoundingBoxTrait for BoundingBox3d {
    fn partition(&self) -> Option<(Self, Self)> {
        if (self.u - self.v).length() < 1e-8 {
            return None;
        }

        let dim = self.get_widest_dimension();
        let midpoint = 0.5 * (self.v.t[dim] + self.u.t[dim]);

        let mut u_new = self.u;
        let mut v_new = self.v;
        u_new.t[dim] = midpoint;
        v_new.t[dim] = midpoint;

        Some((
            BoundingBox3d {
                u: self.u,
                v: v_new,
            },
            BoundingBox3d {
                u: u_new,
                v: self.v,
            },
        ))
    }

    fn extend(&self) -> Option<(Self, Self)> {
        if (self.u - self.v).length() > 1e8 {
            return None;
        }

        let dim = self.get_most_narrow_dimension();
        let not_nan_zero = 0.0;

        let right = self.v.t[dim];
        let left = self.u.t[dim];
        let dif = right - left;
        let left_clamped = if left < not_nan_zero {
            left
        } else {
            not_nan_zero
        };
        let right_clamped = if right > not_nan_zero {
            right
        } else {
            not_nan_zero
        };
        let mut template = self.clone();

        if -left_clamped < right_clamped {
            template.u.t[dim] -= dif;
            template.v.t[dim] -= dif;
            let parent = Self {
                u: template.u.clone(),
                v: self.v.clone(),
            };
            Some((template, parent))
        } else {
            template.u.t[dim] += dif;
            template.v.t[dim] += dif;
            let parent = Self {
                u: self.u.clone(),
                v: template.v.clone(),
            };
            Some((template, parent))
        }
    }

    fn is_contained(&self, other: &Self) -> bool {
        other.u.t[0] <= self.u.t[0]
            && self.v.t[0] <= other.v.t[0]
            && other.u.t[1] <= self.u.t[1]
            && self.v.t[1] <= other.v.t[1]
            && other.u.t[2] <= self.u.t[2]
            && self.v.t[2] <= other.v.t[2]
    }

    fn intersects(&self, other: &Self) -> bool {
        self.intersects(other)
    }

    fn is_sub_scale(&self, other: &Self) -> bool {
        (self.v - self.u).length() <= (other.v - other.u).length()
    }
}

impl HittableBoundingBoxTrait for BoundingBox3d {
    fn hit(&self, ray: &ConstrainedRay3d) -> HitBoxResult {
        self.is_hit_by_ray(&ray)
    }
}

pub trait KdTreeContent<BoundingBox: BoundingBoxTrait>: Clone + Eq + std::hash::Hash {
    fn get_bounding_box(&self) -> BoundingBox;
}

impl KdTreeContent<BoundingBox2d> for Point2d {
    fn get_bounding_box(&self) -> BoundingBox2d {
        BoundingBox2d { u: *self, v: *self }
    }
}

impl KdTreeContent<BoundingBox3d> for Point3d {
    fn get_bounding_box(&self) -> BoundingBox3d {
        BoundingBox3d { u: *self, v: *self }
    }
}
