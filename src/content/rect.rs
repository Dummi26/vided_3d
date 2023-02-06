use crate::pos::{Line, Vector};

use super::drawable::{Drawable, PointLightProperties, PointRayProperties};

/// A simple rectangle
pub struct Rect {
    /// The center position of the rectangle
    pub center: Vector,
    /// center -+ down -> top/bottom edges
    pub down: Vector,
    /// center -+ right -> left/right edges
    pub right: Vector,

    pub material: PointLightProperties,
}

impl Drawable for Rect {
    fn get_outer_bounds(&self) -> (Vector, f64) {
        (self.center.clone(), (&self.down + &self.right).len())
    }
    fn get_intersection(&self, ray: &Line) -> Option<(f64, PointRayProperties)> {
        // find the height of the ray's base over the plane (the rectangle, if it was infinitely big)
        //  get the normal vector of the plane where normal_vec.len() == 1.0
        //  the normal vector points in the direction from where the rectangle can be seen.
        let normal_vec = (&self.down ^ &self.right).normalized();
        //  get the vector from the ray's base to any point on the plane
        let from_ray_to_plane = &self.center - &ray.base;
        //  get the distance from the ray's base to the plane (the "height" of the point above the plane)
        //  if normal_vec points in the opposite direction of from_ray_to_plane, height is negative.
        let height = &from_ray_to_plane * &normal_vec;
        if height.is_sign_positive() {
            // rectangle is invisible from x-angles outside of +-PI ("from behind")
            return None;
        }
        let height = height.abs();
        // find the intersection
        //  find cos(alpha) using vector properties (normal vector has length 1)
        //  because normal_vec points in opposite direction to ray.dir, this must be negative.
        let cos = (&normal_vec * &ray.dir) / ray.dir.len();
        if cos < 0.0 {
            //  the distance along the vector increases the smaller the cosine of the angle gets (-> approaching 90° between ray and normal vector -> ray || plane -> no intersection, infinite distance)
            let dist = height / cos.abs();
            let intersection_point = &ray.base + &(&ray.dir * dist);
            let intersection_from_center = &intersection_point - &self.center;
            let (rlen, dlen) = (self.right.len(), self.down.len());
            let intersection_coord = (
                (&intersection_from_center * &self.right) / (rlen * rlen), // in relative coords
                (&intersection_from_center * &self.down) / (dlen * dlen),  // (from -1 to +1)
            );
            if intersection_coord.0.abs() > 1.0 || intersection_coord.1.abs() > 1.0 {
                return None;
            }
            // mirror the ray's direction vector around(?) the normal vector.
            //  visualize both  ray.dir and normal_vec as originating from a coordinate system's origin.
            //  then, the normal vector multiplied by
            //  the projection of the ray's direction vector in the normal vector's direction
            //  is the vector pointing straight up from the origin to the same "height" as the ray's direction vector.
            //  Note that the ray actually points 'into' the origin, so we have to negate it.
            let mirror_vec = &normal_vec * (&-&ray.dir * &normal_vec);
            //  Now, mirror the point ray.dir points to at the point of the mirror_vec to find a new point.
            //  This new point is where the mirrored vector points to.
            //  Note that, because the ray needs to be inverted, there is a + inside the brackets where a - would be expected.
            let reflect_vector = &mirror_vec + &(&mirror_vec + &ray.dir);
            let mat = self.material.clone();
            Some((
                dist,
                PointRayProperties {
                    orientation: reflect_vector,
                    light_properties: mat,
                },
            ))
        } else {
            None
        }
    }
}
