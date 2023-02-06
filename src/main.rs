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
    let renderer = Renderer::new(vec![
        // RED
        Arc::new(content::rect::Rect {
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
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 0.0,
                },
                transparency: Color {
                    r: 0.3,
                    g: 0.3,
                    b: 0.3,
                    a: 0.0,
                },
                emittance: Color {
                    r: 0.5,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
                scattering: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        }),
        // GREEN | behind red
        Arc::new(content::rect::Rect {
            center: Vector {
                x: 3.0,
                y: 1.0,
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
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 0.0,
                },
                emittance: Color {
                    r: 0.0,
                    g: 0.5,
                    b: 0.0,
                    a: 0.0,
                },
                scattering: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        }),
        // BLUE | behind the camera
        Arc::new(content::rect::Rect {
            center: Vector {
                x: -2.0,
                y: -0.5,
                z: 0.25,
            },
            down: Vector {
                x: 0.0,
                y: 0.0,
                z: -0.5,
            },
            right: Vector {
                x: 0.0,
                y: -1.0, // because it is behind the camera (rectangles are only visible from one side (might change in the future))
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
                    r: 0.5,
                    g: 0.5,
                    b: 0.5,
                    a: 0.0,
                },
                emittance: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.5,
                    a: 0.0,
                },
                scattering: Color {
                    r: 0.0,
                    g: 0.0,
                    b: 0.0,
                    a: 0.0,
                },
            },
        }),
    ]);

    let start_time = Instant::now();
    let image = renderer.render(
        RenderEnv {
            frame: 0,
            max_light_rays: 1,
        },
        [1920, 1080],
        // [3, 3],
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
