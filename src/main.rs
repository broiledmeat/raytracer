extern crate rgb;
extern crate lodepng;
extern crate rand;
mod camera;
mod material;
mod ray;
mod renderable;
mod scene;
mod vector3;
use rgb::RGBA;
use camera::Camera;
use ray::Ray;
use renderable::{Plane, Sphere};
use scene::Scene;
use vector3::Vector3;


fn main()
{
    const WIDTH: usize = 400;
    const HEIGHT: usize = 200;
    const RAY_COUNT: usize = 200;
    const BOUNCE_MAX: i32 = 100;

    let scene = Scene
    {
        renderables: vec![
            Box::new(Plane{origin: Vector3{x: 0.0, y: 0.0, z:0.0}, normal: Vector3{x: 0.0, y: 1.0, z: 0.0},
                material: Box::new(material::Lambert{albedo: Vector3{x: 0.4, y: 0.8, z: 0.4}})}),
            Box::new(Sphere{origin: Vector3{x: 1.0, y: 0.5, z:-1.0}, radius: 0.5,
                material: Box::new(material::Metal{albedo: Vector3{x: 0.8, y: 0.6, z: 0.2}, fuzz: 0.2})}),
            Box::new(Sphere{origin: Vector3{x: -1.0, y: 0.5, z:-1.0}, radius: -0.5,
                material: Box::new(material::Dielectric{refraction: 1.5})}),
            Box::new(Sphere{origin: Vector3{x: -1.25, y: 1.6, z:-2.0}, radius: 0.8,
                material: Box::new(material::Lambert{albedo: Vector3{x: 0.3, y: 0.3, z: 0.9}})}),
        ]
    };
    let camera_origin = Vector3{x: -0.5, y: 0.6, z: 1.0};
    let camera_look_at = Vector3{x: 0.0, y: 0.5, z: 0.0};
    let camera_up = Vector3{x: 0.0, y: 1.0, z:0.0};
    let camera = Camera::new(
        camera_origin, camera_look_at,
        camera_up, 72.0, (WIDTH as f64) / (HEIGHT as f64), 0.0, (camera_origin - camera_look_at).length());

    // Render to image
    let mut pixel_data = [RGBA{r: 0, g: 0, b: 0, a: 255}; WIDTH * HEIGHT];

    for y in 0..HEIGHT
    {
        for x in 0..WIDTH
        {
            let mut color = vector3::ZERO;
            for _ in 0..RAY_COUNT
            {
                let u = ((x as f64) + rand::random::<f64>()) / (WIDTH as f64);
                let v = ((y as f64) + rand::random::<f64>()) / (HEIGHT as f64);
                let ray = camera.get_ray(u, v);
                color += get_color(ray, &scene, BOUNCE_MAX);
            }
            color /= RAY_COUNT as f64;
            color = Vector3{x: color.x.sqrt(), y: color.y.sqrt(), z: color.z.sqrt()};

            pixel_data[(HEIGHT - 1 - y) * WIDTH + x] = RGBA
            {
                r: (color.x * 255.0) as u8, 
                g: (color.y * 255.0) as u8,
                b: (color.z * 255.0) as u8, 
                a: 255
            };
        }
    }

    lodepng::encode32_file("out.png", &pixel_data, WIDTH, HEIGHT).unwrap();
}

fn get_color(ray: Ray, scene: &Scene, bounce_max: i32) -> Vector3
{
    let hit_result = scene.test_hit(ray);
    match hit_result
    {
        None => {},
        Some(h) =>
        {
            if bounce_max < 0
            {
                return vector3::ZERO;
            }

            let scatter_result = h.material.scatter(ray, h);
            match scatter_result
            {
                None => {},
                Some(s) =>
                {
                    return s.attenuation * get_color(s.scattered, scene, bounce_max - 1);
                }
            }
        }
    }

    let direction = ray.direction.normalized();
    let t = 0.5 * (direction.y + 1.0);
    (1.0 - t) * vector3::ONE + t * Vector3{x: 0.5, y: 0.7, z: 1.0}
}
