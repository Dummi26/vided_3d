use std::sync::Arc;

use image::{DynamicImage, GenericImageView};

use crate::pos::{Line, Vector};

use super::drawable::{Color, Drawable, PointLightPropertiesCustomType, PointRayProperties};

/// A simple rectangle
pub struct Rect {
    /// The center position of the rectangle
    pub center: Vector,
    /// center -+ down -> top/bottom edges
    pub down: Vector,
    /// center -+ right -> left/right edges
    pub right: Vector,

    pub material: PointLightPropertiesCustomType<RectMaterialProperty>,
}

#[derive(Clone)]
pub enum RectMaterialProperty {
    Single(Color),
    Image(DynamicImage),
    ImageArc(Arc<DynamicImage>),
}
impl RectMaterialProperty {
    pub fn get_property(&self, pos: (f64, f64)) -> Color {
        match self {
            Self::Single(v) => v.clone(),
            Self::Image(img) => {
                let x = (pos.0 * (img.width().saturating_sub(1)) as f64).round() as _;
                let y = (pos.1 * (img.height().saturating_sub(1)) as f64).round() as _;
                Color::from_u8s(img.get_pixel(x, y).0)
            }
            Self::ImageArc(img) => {
                let x = (pos.0 * (img.width().saturating_sub(1)) as f64).round() as _;
                let y = (pos.1 * (img.height().saturating_sub(1)) as f64).round() as _;
                Color::from_u8s(img.get_pixel(x, y).0)
            }
        }
    }
}

impl Drawable for Rect {
    fn get_outer_bounds(&self) -> (Vector, f64) {
        (self.center.clone(), (&self.down + &self.right).len())
    }
    fn get_intersection(&self, ray: &Line) -> Option<(f64, PointRayProperties)> {
        // find the height of the ray's base over the plane (the rectangle, if it was infinitely big)
        //  get the normal vector of the plane where normal_vec.len() == 1.0
        let normal_vec = (&self.right ^ &self.down).normalized();
        //  get the vector from the ray's base to any point on the plane
        let from_ray_to_plane = &self.center - &ray.base;
        //  get the distance from the ray's base to the plane (the "height" of the point above the plane)
        let height = &from_ray_to_plane * &normal_vec;
        // find the intersection
        //  find cos(alpha) using vector properties (normal vector has length 1)
        //  because normal_vec points in opposite direction to ray.dir, this must be negative.
        let cos = (&normal_vec * &ray.dir) / ray.dir.len();
        if cos < 0.0 {
            //  the distance along the vector increases the smaller the cosine of the angle gets (-> approaching 90° between ray and normal vector -> ray || plane -> no intersection, infinite distance)
            let dist = height / cos;
            let intersection_point = &ray.base + &(&ray.dir * dist);
            let intersection_from_center = &intersection_point - &self.center;
            let intersection_coord = (
                (&intersection_from_center * &self.right) / self.right.len_sq(), // in relative coords
                (&intersection_from_center * &self.down) / self.down.len_sq(),   // (from -1 to +1)
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
            let mat = self.material.convert_ref(|v| {
                v.get_property((
                    (1.0 + intersection_coord.0) / 2.0,
                    (1.0 + intersection_coord.1) / 2.0,
                ))
            });
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
