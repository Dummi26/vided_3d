use std::sync::Arc;

use image::{DynamicImage, GenericImage};

use crate::{
    content::drawable::{Color, Drawable},
    pos::{Line, Vector},
};

pub struct Renderer {
    to_render: Vec<Arc<dyn Drawable>>,
}

#[derive(Clone)]
pub struct RenderEnv {
    pub frame: u128,
    pub max_light_rays: u64,
}

impl Renderer {
    pub fn new(to_render: Vec<Arc<dyn Drawable>>) -> Self {
        Self { to_render }
    }

    pub fn render(
        &self,
        env: RenderEnv,
        res: [u32; 2],
        pos: Vector,
        dir: (f64, f64),
        fov: (f64, f64),
    ) -> DynamicImage {
        // let mut out_img = Vec::with_capacity(res[0] as usize * res[1] as usize);
        let mut out_img = image::DynamicImage::new_rgba8(res[0], res[1]);
        for y in 0..res[1] {
            let dir_y = dir.1 + fov.1 * ((2 * y) as f64 / ((res[1] - 1) as f64) - 1.0);
            for x in 0..res[0] {
                let dir_x = dir.0 + fov.0 * ((2 * x) as f64 / ((res[0] - 1) as f64) - 1.0);
                let dir = Vector {
                    x: dir_x.cos(),
                    y: dir_x.sin(),
                    z: -dir_y.sin(),
                };
                let ray = Line { base: pos, dir };
                out_img.put_pixel(
                    x,
                    y,
                    self.render_ray(
                        env.clone(),
                        ray,
                        Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        },
                    )
                    .u8s()
                    .into(),
                );
            }
        }
        out_img
    }
    pub fn render_ray(&self, env: RenderEnv, ray: Line, mut color: Color) -> Color {
        let mut hit_properties = Vec::new();
        for renderable in &self.to_render {
            if let Some(intersection) = renderable.get_intersection(&ray) {
                hit_properties.push(intersection);
            }
        }

        hit_properties.sort_unstable_by(|a, b| {
            a.0.partial_cmp(&b.0)
                .expect("There should be no NaN or other not comparable float values here.")
        });

        let mut strength = Color {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        };

        for hit in hit_properties {
            if strength.r.max(strength.g).max(strength.b) < 0.002 {
                break;
            };
            // emittance
            let emittance = &hit.1.light_properties.emittance * &strength;
            color += &emittance;
            // transparency
            //  not 0.0 or any invalid value (also ignores subnormals, but this shouldn't matter)
            //  strength gets weaker
            strength *= &hit.1.light_properties.transparency;
            // TODO
            // reflection
            // scattering
            // ... (?)
        }
        color
    }
}
