use std::{sync::Arc, time::Instant};

use content::drawable::PointLightProperties;

use crate::{
    content::drawable::Color,
    pos::{Line, Vector},
    render::{RenderEnv, Renderer},
};

mod content;
mod pos;
mod render;

fn main() {
    let renderer = Renderer::new(vec![Arc::new(content::rect::Rect {
        center: Vector {
            x: 2.5,
            y: 0.0,
            z: 0.0,
        },
        down: Vector {
            x: 0.0,
            y: 0.0,
            z: -0.5,
        },
        right: Vector {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
        material: PointLightProperties {
            reflectiveness: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            transparency: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            emittance: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.0,
            },
            scattering: Color {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
        },
    })]);

    let start_time = Instant::now();
    let image = renderer.render(
        RenderEnv {
            frame: 0,
            max_light_rays: 0,
        },
        [1920, 1080],
        Vector {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        (0.0, 0.0),
        (16.0 / 18.0, 0.5),
    );
    eprintln!("Took {:.2}ms.", start_time.elapsed().as_secs_f64() * 1000.0);
    image.save("/tmp/render.png").unwrap();
}
