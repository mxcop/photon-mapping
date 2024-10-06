use std::f32::consts::PI;

use circle::Circle;
use color::Color;
use glam::{Vec2, Vec3, Vec4};
use minifb::{Key, Window, WindowOptions};
use photon::event::PhotonEvent;
use rand::Rng;
use ray::Ray;

const WIDTH: usize = 1920 / 4;
const HEIGHT: usize = 1080 / 4;

mod color;
mod ray;
mod circle;
mod photon;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut rng = rand::thread_rng();

    // Create a window.
    let opts = WindowOptions { 
        scale: minifb::Scale::X4,
        ..WindowOptions::default()
    };
    let mut window = Window::new(
        "Photon Mapping",
        WIDTH,
        HEIGHT,
        opts,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate.
    window.set_target_fps(60);

    let light = Circle::new(Vec2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0), 32.0);
    let floor = Circle::new(Vec2::new(WIDTH as f32 / 2.0, -1024.0 + 64.0), 1024.0);

    /// Photons per flux.
    // const PPF: u32 = 16;
    const PHOTONS: usize = 2usize.pow(16) * 4;

    let mut photon_map: Vec<PhotonEvent> = Vec::new();

    for _ in 0..PHOTONS {
        // Photon origin. (random point on circle light)
        let theta: f32 = rng.gen::<f32>() * 2.0 * PI;
        let po = light.o + Vec2::new(theta.cos(), theta.sin());

        // Photon direction.
        let phi: f32 = theta + (rng.gen::<f32>() * PI - PI * 0.5);
        let pd = Vec2::new(phi.cos(), phi.sin());

        let ray = Ray::new(po + pd * 0.01, pd.normalize());

        let hit = floor.intersect_ray(ray);
        if let Some(t) = hit {
            let mut color = Vec3::new(0.0, 0.0, 1.0);
            if theta < PI * 0.5 {
                color = Vec3::new(1.0, 0.0, 0.0);
            }
            photon_map.push(PhotonEvent {
                p: ray.o + ray.d * t,
                i: ray.d,
                c: color
            });
        }
    }

    let scale = (1.0 / PHOTONS as f32) * 256.0;

    // Render once.
    for (i, c) in buffer.iter_mut().enumerate() {
        /* Partition the index into cartesian coordinates */
        let p = Vec2::new((i % WIDTH) as f32, (i / WIDTH) as f32);

        let mut flux = Vec3::new(0.0, 0.0, 0.0);
        let normal = Vec2::new(0.0, 1.0);
        for ev in &photon_map {
            // If this event is relevant to this pixel.
            if ev.p.distance(p) < 1.0 {
                flux += ev.c * normal.dot(ev.i).max(0.0) * scale;
            }
        }

        let irradiance = Vec4::new(flux.x, flux.y, flux.z, 1.0);

        if light.intersect_point(p) {
            if p.x > WIDTH as f32 / 2.0 {
                *c = Color::red().into_argb();
            } else {
                *c = Color::blue().into_argb();
            }
        } else if floor.intersect_point(p) {
            *c = 0;
        } else {
            *c = Color::from_vec4(irradiance).into_argb();
        }
    }

    // Main loop.
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Update with buffer.
        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
