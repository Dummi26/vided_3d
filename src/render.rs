use std::sync::Arc;

use image::{DynamicImage, GenericImage};
use rayon::prelude::*;

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

    pub fn to_render(&self) -> &Vec<Arc<dyn Drawable>> {
        &self.to_render
    }
    pub fn to_render_mut(&mut self) -> &mut Vec<Arc<dyn Drawable>> {
        &mut self.to_render
    }

    pub fn render(
        &self,
        env: RenderEnv,
        res: [u32; 2],
        pos: Vector,
        right: Vector,
        up: Vector,
    ) -> DynamicImage {
        self.render_parallel(env, res, pos, right, up)
    }

    pub fn render_sequential(
        &self,
        env: RenderEnv,
        res: [u32; 2],
        pos: Vector,
        right: Vector,
        down: Vector,
    ) -> DynamicImage {
        let mut image = image::DynamicImage::new_rgba8(res[0], res[1]);
        let forward = (&down ^ &right).normalized();
        for y in 0..res[1] {
            let down_factor = (2 * y) as f64 / ((res[1] - 1) as f64) - 1.0;
            let down_vec = &down * down_factor;
            let temp_vec = &forward + &down_vec;
            for x in 0..res[0] {
                let right_factor = (2 * x) as f64 / ((res[0] - 1) as f64) - 1.0;
                let right_vec = &right * right_factor;
                let dir = &temp_vec + &right_vec;
                let ray = Line { base: pos, dir };
                image.put_pixel(x, y, self.render_ray(env.clone(), ray).u8s().into());
            }
        }
        image
    }

    pub fn render_parallel(
        &self,
        env: RenderEnv,
        res: [u32; 2],
        pos: Vector,
        right: Vector,
        down: Vector,
    ) -> DynamicImage {
        let width = res[0] as usize;
        let height = res[1] as usize;
        let pixels = width * height;
        let forward = (&down ^ &right).normalized();
        image::RgbaImage::from_raw(
            res[0],
            res[1],
            (0..pixels)
                .into_par_iter()
                .map(|pixel| {
                    let x = pixel % width;
                    let y = pixel / width;
                    let right_factor = (2 * x) as f64 / ((res[0] - 1) as f64) - 1.0;
                    let down_factor = (2 * y) as f64 / ((res[1] - 1) as f64) - 1.0;
                    let down_vec = &down * down_factor;
                    let temp_vec = &forward + &down_vec;
                    let right_vec = &right * right_factor;
                    let dir = &temp_vec + &right_vec;
                    let ray = Line { base: pos, dir };
                    self.render_ray(env.clone(), ray).u8s()
                })
                .flatten()
                .collect(),
        )
        .unwrap()
        .into()
    }
    pub fn render_ray(&self, env: RenderEnv, ray: Line) -> Color {
        let mut color = Color {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 0.0,
        };
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
            let hit_pos = &ray.base + &(hit.0 * &ray.dir);

            if strength.r.max(strength.g).max(strength.b).abs() < 0.002 {
                break;
            };
            // emittance
            let emittance = &hit.1.light_properties.emittance * &strength;
            color += &emittance;
            // transparency
            //  not 0.0 or any invalid value (also ignores subnormals, but this shouldn't matter)
            //  strength gets weaker
            strength *= &hit.1.light_properties.transparency;
            // reflection
            let reflectiveness = &hit.1.light_properties.reflectiveness;
            if !reflectiveness.is_transparent() {
                if env.max_light_rays != 0 {
                    let mut env = env.clone();
                    env.max_light_rays -= 1;
                    color += &(reflectiveness
                        * &self.render_ray(
                            env,
                            Line {
                                base: hit_pos,
                                dir: hit.1.orientation,
                            },
                        ));
                }
            }
            // TODO: improve this (multithreading?)
            // scattering
            if !hit.1.scattering_orientations.is_empty() {
                // divide scattering by .len(), because we add it .len() times (loop)
                let scattering =
                    &hit.1.light_properties.scattering / hit.1.scattering_orientations.len() as f64;
                if !scattering.is_transparent() {
                    let mut env = env.clone();
                    env.max_light_rays =
                        env.max_light_rays / hit.1.scattering_orientations.len() as u64;
                    for ray in &hit.1.scattering_orientations {
                        color += &(&scattering
                            * &self.render_ray(
                                env.clone(),
                                Line {
                                    base: hit_pos,
                                    dir: *ray,
                                },
                            ));
                    }
                }
            }
        }
        color
    }
}
